use std::thread::spawn;

const THREAD_COUNT: u8 = 4;

pub fn linear_sum(array: &[u8]) -> u64 {
    array.iter().map(|&i| i as u64).sum()
}

pub fn threaded_sum(array: &'static [u8]) -> u64 {
    array
        .chunks(array.len() / (THREAD_COUNT as usize)) // divide into THREAD_COUNT slices
        .map(|chunk| spawn(move || linear_sum(chunk))) // start threads and yield their handles
        // we collect the handles here to ensure we start every thread before we start joining
        // them. Without this, the *whole* iterator chain (most importantly spawn and join) is
        // applied to the first chunk, then second, etc.
        .collect::<Vec<_>>()
        .into_iter() // move the handles into the next map so it can drop them after joining
        .map(|handle| handle.join().unwrap()) // yield each chunk's sum
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static ARR1: [u8; 10] = [1; 10];
    static ARR2: [u8; 5] = [2; 5];
    static ARR3: [u8; 85] = [1; 85];
    static ARR4: [u8; 15] = [1, 3, 5, 6, 7, 9, 45, 52, 1, 2, 3, 0, 0, 4, 138];

    fn sum_test(sum: fn(&'static [u8]) -> u64) {
        assert_eq!(10, sum(&ARR1));
        assert_eq!(10, sum(&ARR2));
        assert_eq!(85, sum(&ARR3));
        assert_eq!(276, sum(&ARR4));
    }

    #[test]
    fn test_linear() {
        sum_test(linear_sum);
    }

    #[test]
    fn test_threaded() {
        sum_test(threaded_sum);
    }
}
