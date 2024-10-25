use core::f64;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct StationStats {
    min: f64,
    max: f64,
    count: u64,
    sum: f64,
}

impl Default for StationStats {
    fn default() -> Self {
        StationStats {
            min: f64::MAX,
            max: f64::MIN,
            count: 0,
            sum: 0.0,
        }
    }
}

impl StationStats {
    fn update_min(&mut self, value: f64) {
        self.min = self.min.min(value);
    }

    fn update_max(&mut self, value: f64) {
        self.max = self.max.max(value);
    }
    fn update_sum(&mut self, value: f64) {
        self.sum += value;
        self.count += 1;
    }

    pub fn update(&mut self, value: f64) {
        self.update_min(value);
        self.update_max(value);
        self.update_sum(value);
    }

    pub fn average(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.sum / self.count as f64
        }
    }
}

fn one_million_rows(file_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let f = File::open(file_name)?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();

    let mut stations: HashMap<String, StationStats> = HashMap::new();

    while let Ok(n_bytes) = reader.read_line(&mut line) {
        if !line.is_empty() {
            let (station_name, reading) = line
                .split_once(";")
                .expect("Row shoud be in format name;value");

            let value = reading.trim().parse::<f64>()?;

            let station = stations
                .entry(station_name.to_string())
                .or_insert(StationStats::default());
            station.update(value);
        }
        line.clear(); // Clear the string buffer

        if n_bytes == 0 {
            break;
        }
    }

    let mut ordered_stations: Vec<(String, StationStats)> =
        stations.into_iter().map(|v| v).collect();

    ordered_stations.sort_by(|a, b| a.0.cmp(&b.0));

    let mut result = Vec::new();
    for (station_name, stats) in ordered_stations {
        result.push(format!(
            "{}={:.1}/{:.1}/{:.1}",
            station_name,
            round_output(stats.min),
            round_output(stats.average()),
            round_output(stats.max)
        ));
    }

    let mut output = String::new();
    output.push('{');
    output.push_str(&result.join(", "));
    output.push('}');

    Ok(output)
}

fn round_output(value: f64) -> f64 {
    (value * 10.0).ceil() / 10.0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = one_million_rows("measurements.txt")?;
    println!("{}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, error::Error, fs, path::PathBuf};

    use crate::one_million_rows;

    fn test_samples() -> Result<Vec<(PathBuf, PathBuf)>, Box<dyn Error>> {
        let dir = PathBuf::from("samples");

        // Use a HashSet to collect unique file stems (without extensions)
        let mut unique_files = HashSet::new();

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
}
