use core::f64;
use crossbeam::channel;
use memmap2::Mmap;
use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry;
use std::fmt::Write;
use std::fs::File;
use std::sync::Arc;
use std::thread;
use std::{env, sync::Mutex};

#[derive(Debug)]
struct StationStats {
    min: V,
    max: V,
    count: u64,
    sum: V,
}

impl Default for StationStats {
    fn default() -> Self {
        StationStats {
            min: V::MAX,
            max: V::MIN,
            count: 0,
            sum: V::default(),
        }
    }
}

impl StationStats {
    fn update_min(&mut self, value: V) {
        self.min = self.min.min(value);
    }

    fn update_max(&mut self, value: V) {
        self.max = self.max.max(value);
    }
    fn update_sum(&mut self, value: V) {
        self.sum += value;
        self.count += 1;
    }

    pub fn update(&mut self, value: V) {
        self.update_min(value);
        self.update_max(value);
        self.update_sum(value);
    }

    pub fn average(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.sum as f64 / 10.0 / self.count as f64
        }
    }

    pub fn merge(&mut self, other: &Self) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
        self.sum += other.sum;
        self.count += other.count;
    }
}

type V = i32;
/// Parse to an i32 with single digit precision
#[inline(always)]
fn parse_numeric(val: &[u8]) -> V {
    let mut num: V = V::default();
    let mut negative = 1;
    let mut has_decimal = false;

    for v in val.iter() {
        match v {
            b'.' => has_decimal = true,
            b'-' => negative = -1,
            _ => num = num * 10 + (v - b'0') as V,
        }
    }

    if has_decimal {
        negative * num
    } else {
        negative * num * 10
    }
}

#[inline(always)]
fn format_val(val: V) -> String {
    format!("{:.1}", val as f64 / 10.0)
}

fn one_million_rows(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let f = File::open(file_name)?;
    let data = unsafe { Mmap::map(&f)? };

    let (tx, rx) = channel::bounded::<&[u8]>(100000);

    let all_stations = thread::scope(|s| {
        let all_stations: Arc<Mutex<FxHashMap<&[u8], StationStats>>> =
            Arc::new(Mutex::new(FxHashMap::default()));

        // Start worker tasks
        for _ in 0..thread::available_parallelism().unwrap().into() {
            let trx = rx.clone();

            let all_stations_local = all_stations.clone();

            s.spawn(move || {
                let mut local_stations: FxHashMap<&[u8], StationStats> = FxHashMap::default();

                while let Ok(batch) = trx.recv() {
                    let line_breaks = memchr::memchr_iter(b'\n', batch);
                    let mut cursor = 0;
                    for new_line_index in line_breaks {
                        let line = unsafe { &batch.get_unchecked(cursor..new_line_index) };

                        if !line.is_empty() {
                            if let Some(index) = memchr::memchr(b';', line) {
                                let (station_name, reading) = line.split_at(index);

                                let value = parse_numeric(unsafe { &reading.get_unchecked(1..) });
                                let station = local_stations.entry(station_name).or_default();
                                station.update(value);
                            }
                        }

                        cursor = new_line_index + 1;
                    }
                }
                // Merge into the global map
                let mut global_map = all_stations_local.lock().unwrap();

                for (key, val) in local_stations {
                    match global_map.entry(key) {
                        Entry::Occupied(mut entry) => {
                            let station = entry.get_mut();
                            station.merge(&val);
                        }
                        Entry::Vacant(entry) => {
                            entry.insert(val);
                        }
                    }
                }
            });
        }

        // Start reading task
        s.spawn(|| {
            let tx = tx; // Move tx to ensure it gets dropped after all data is sent
            let n_chunks: usize = thread::available_parallelism().unwrap().into();
            let file_size = data.len();
            let chunk_size = (file_size / n_chunks).max(1);
            let mut cursor: usize = 0;

            while cursor < file_size {
                // Propose a slice
                let mut slice_end = (cursor + chunk_size).min(file_size); //exclusive bound

                let slice = unsafe { &data.get_unchecked(cursor..slice_end) };

                let accepted_slice = if unsafe { *slice.get_unchecked(slice.len() - 1) } == b'\n' {
                    slice
                } else {
                    // Otherwise look for the next newline in the remaining data
                    let remaining = &data[cursor + chunk_size..];

                    if let Some(next_newline) = memchr::memchr(b'\n', remaining) {
                        slice_end += next_newline;
                        unsafe {
                            &data.get_unchecked(cursor..cursor + chunk_size + next_newline + 1)
                        }
                    } else {
                        unsafe { &data.get_unchecked(cursor..) }
                    }
                };

                cursor = slice_end;
                tx.send(accepted_slice).expect("Channel must be open");
            }
        });

        all_stations
    });

    // Unwrap the vector of results from the Arc<Mutex<...>>
    let stations = Arc::try_unwrap(all_stations).unwrap().into_inner().unwrap();

    let mut ordered_stations: Vec<(&[u8], StationStats)> = stations.into_iter().collect();
    ordered_stations.sort_by(|a, b| a.0.cmp(b.0));

    let mut output = String::new();
    output.push('{');

    let mut first = true;

    for (station_name, stats) in ordered_stations {
        if first {
            first = false;
        } else {
            output.push_str(", ");
        }
        write!(
            &mut output,
            "{}={}/{:.1}/{}",
            unsafe { String::from_utf8_unchecked(station_name.to_owned()) },
            format_val(stats.min),
            round_output(stats.average()),
            format_val(stats.max)
        )
        .expect("Could not write to buffer");
    }

    output.push('}');

    Ok(output)
}

#[inline(always)]
fn round_output(value: f64) -> f64 {
    (value * 10.0).ceil() / 10.0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let res = one_million_rows(&args[1])?;
            println!("{}", res);
        }
        _ => panic!("Only expected one argument"),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fs, path::PathBuf};

    use rustc_hash::FxHashSet;

    use crate::{format_val, one_million_rows, parse_numeric};

    fn test_samples() -> Result<Vec<(PathBuf, PathBuf)>, Box<dyn Error>> {
        let dir = PathBuf::from("samples");

        // Use a HashSet to collect unique file stems (without extensions)
        let mut unique_files = FxHashSet::default();

        // Iterate over the directory entries and collect unique file stems
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();

            // Only process if it's a file and has a valid file stem
            if path.is_file() {
                if let Some(stem) = path.file_stem() {
                    unique_files.insert(stem.to_string_lossy().to_string());
                }
            }
        }

        // Create a vector to store full paths for both .txt and .out files
        let mut file_paths = Vec::new();

        for stem in unique_files {
            let txt_path = dir.join(format!("{}.txt", stem));
            let out_path = dir.join(format!("{}.out", stem));

            // Store the full file paths
            file_paths.push((txt_path, out_path));
        }

        Ok(file_paths)
    }

    #[test]
    fn test_suite() -> Result<(), Box<dyn Error>> {
        for (input, expected) in test_samples()? {
            let expected_output = fs::read_to_string(expected)?.replace("\n", "");
            println!("Input {}", input.to_str().unwrap());
            let result = one_million_rows(input.to_str().unwrap())?;
            assert_eq!(expected_output, result);
        }

        Ok(())
    }

    #[test]
    fn test_custom_parse() {
        assert_eq!(124.2, parse_numeric("124.2".as_bytes()) as f64 / 10.0);
        assert_eq!(10.5, parse_numeric("10.5".as_bytes()) as f64 / 10.0);

        assert_eq!(4.0, parse_numeric("4".as_bytes()) as f64 / 10.0);
        assert_eq!(42.0, parse_numeric("42".as_bytes()) as f64 / 10.0);

        assert_eq!(-53.3, parse_numeric("-53.3".as_bytes()) as f64 / 10.0);
        assert_eq!(-53.0, parse_numeric("-53".as_bytes()) as f64 / 10.0);

        assert_eq!("1.0", format_val(parse_numeric("1.0".as_bytes())));
    }
}
