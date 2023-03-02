#[cfg(test)]
mod tests {
    #[test]
    fn test_quicksort() {
        let mut array = vec![5, 4, 3, 2, 1];
        mytools::sorts::quicksort(&mut array);
        assert_eq!(array, vec![1, 2, 3, 4, 5]);
        array.sort()
    }
}
