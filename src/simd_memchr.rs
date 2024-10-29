#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx2"
))]
use std::arch::x86_64::{
    __m256i, _mm256_cmpeq_epi8, _mm256_loadu_si256, _mm256_movemask_epi8, _mm256_set1_epi8,
};

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx2"
))]
pub unsafe fn memchr_unsafe(needle: u8, haystack: &[u8]) -> Option<usize> {
    unsafe { memchr_iter_unsafe(needle, haystack).next() }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn memchr(needle: u8, haystack: &[u8]) -> Option<usize> {
    unsafe { memchr_unsafe(needle, haystack) }
}

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx2"
))]
pub unsafe fn memchr_iter_unsafe(needle: u8, haystack: &[u8]) -> Memchr<'_> {
    unsafe {
        Memchr {
            needle,
            needle_vec: _mm256_set1_epi8(needle as i8),
            haystack,
            index: 0,
        }
    }
}

/// Return an iterator over indices of needles in a haystack.
/// Efficient simd impementation
pub fn memchr_iter(needle: u8, haystack: &[u8]) -> Memchr<'_> {
    unsafe { memchr_iter_unsafe(needle, haystack) }
}

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx2"
))]
#[derive(Debug)]
pub struct Memchr<'a> {
    needle: u8,
    needle_vec: __m256i,
    haystack: &'a [u8],
    index: usize,
}

impl<'a> Iterator for Memchr<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let haystack_len = self.haystack.len();

        let mask_size = 32;

        while self.index < haystack_len {
            // Safety: Check there is at least 32 bytes (256 bits) left to read.
            // If there are less than 32 bytes then process remaining without simd
            // to avoid out of bounds read.
            if haystack_len - self.index >= mask_size {
                let ptr = self.haystack[self.index..].as_ptr() as *const __m256i;
                unsafe {
                    let haystack_vec = _mm256_loadu_si256(ptr);
                    let cmp_vec = _mm256_cmpeq_epi8(self.needle_vec, haystack_vec);
                    let mask = _mm256_movemask_epi8(cmp_vec);

                    if mask != 0 {
                        let index = mask.trailing_zeros() as usize;
                        self.index += index + 1;
                        return Some(self.index - 1);
                    } else {
                        self.index += mask_size;
                    }
                }
            } else {
                self.index += 1;
                if self.haystack[self.index - 1] == self.needle {
                    return Some(self.index - 1);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::simd_memchr::memchr_iter_unsafe;

    use super::memchr;
    #[test]
    fn test_memchr() {
        let haystack = [1_u8, 2, 3, 4, 5, 6];
        let needle = 5_u8;

        assert_eq!(Some(4), memchr(needle, &haystack[..]));

        assert_eq!(Some(4), unsafe {
            memchr_iter_unsafe(needle, &haystack[..]).next()
        });
    }

    #[test]
    fn test_memchr_iter() {
        let haystack = [b'a', b'b', b'c', b'a', b'b', b'c', b'c', b'a', b'c'];
        let needle = b'c';

        let mut mechr_iterator = unsafe { memchr_iter_unsafe(needle, &haystack[..]) };

        assert_eq!(Some(2), mechr_iterator.next());
        println!("{:?}", mechr_iterator);
        assert_eq!(Some(5), mechr_iterator.next());
        assert_eq!(Some(6), mechr_iterator.next());
        assert_eq!(Some(8), mechr_iterator.next());
        assert_eq!(None, mechr_iterator.next());
    }

    #[test]
    fn test_memchr_iter_sample() {
        let f = fs::read("samples/measurements-complex-utf8.txt").unwrap();

        let mechr_iterator = unsafe { memchr_iter_unsafe(b'\n', &f[..]) };
        let line_iterator = f.iter().enumerate().filter(|&b| *b.1 == b'\n').map(|b| b.0);

        let all_index = mechr_iterator.collect::<Vec<_>>();
        let expected_index = line_iterator.collect::<Vec<_>>();

        assert_eq!(all_index, expected_index);
    }
}
