pub fn linear_sum(array: &[u8]) -> u64 {
    array.iter().map(|&i| i as u64).sum()
}

#[cfg(test)]
mod tests {
    use super::linear_sum;

    fn sum_test(sum: fn(&[u8]) -> u64) {
        assert_eq!(10, sum(&[1; 10]));
        assert_eq!(10, sum(&[2; 5]));
        assert_eq!(85, sum(&[1; 85]));
        assert_eq!(276, sum(&[1,3,5,6,7,9,45,52,1,2,3,0,0,4,138]));
    }

    #[test]
    fn test_linear() {
        sum_test(linear_sum);
    }
}
