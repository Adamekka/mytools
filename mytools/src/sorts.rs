/// Sorts a vector using the quicksort algorithm.
///
/// # Examples
///
/// ```
/// let mut array = vec![5, 4, 3, 2, 1];
/// mytools::sorts::quicksort(&mut array);
/// assert_eq!(array, vec![1, 2, 3, 4, 5]);
/// ```
///
/// # Panics
///
/// Panics if the vector is empty.
///
/// # Performance
///
/// The quicksort algorithm has a worst-case performance of O(n^2) and an average performance of O(n log n).
///
/// # Implementation
///
/// The quicksort algorithm is a divide-and-conquer algorithm. It works by selecting a pivot element from the vector and partitioning the other elements into two subvectors, according to whether they are less than or greater than the pivot. The subvectors are then sorted recursively. This continues until the base case is reached, where the vector has length 1.
///
/// The pivot is selected as the last element in the vector. The partitioning is done by maintaining an index `i` that starts at -1. The index is incremented each time an element less than the pivot is encountered. When this happens, the element at index `i` is swapped with the element at index `j`. This ensures that all elements less than the pivot are to the left of `i`, and all elements greater than the pivot are to the right of `i`. The pivot is then swapped with the element at index `i + 1`, which is guaranteed to be greater than the pivot.
///
/// # References
///
/// - [Wikipedia](https://en.wikipedia.org/wiki/Quicksort)
/// - [YouTube](https://www.youtube.com/watch?v=SLauY6PpjW4)
/// - [YouTube](https://www.youtube.com/watch?v=COk73cpQbFQ)
/// - [YouTube](https://www.youtube.com/watch?v=7h1s2SojIRw)
/// - [YouTube](https://www.youtube.com/watch?v=8hEyhs3OV1w)
/// - [YouTube](https://www.youtube.com/watch?v=eqo2LxRADhU)
/// - [YouTube](https://www.youtube.com/watch?v=Hoixgm4-P4M)
/// - [YouTube](https://www.youtube.com/watch?v=3hH8kTHFw2A)
pub fn quicksort<T: std::fmt::Debug + Ord>(array: &mut Vec<T>) {
    _quicksort(array, 0, (array.len() - 1) as isize);
}

fn _quicksort<T: std::fmt::Debug + Ord>(array: &mut Vec<T>, low: isize, high: isize) {
    if low < high {
        let pivot = partition(array, low, high);
        #[cfg(debug_assertions)]
        {
            println!("low: {}, high: {}, pivot: {}", low, high, pivot);
            println!("{array:?}");
        }
        _quicksort(array, low, pivot - 1);
        _quicksort(array, pivot + 1, high);
    }
}

fn partition<T: Ord>(array: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high;
    let mut i = low - 1;

    for j in low..=high - 1 {
        if array[j as usize] < array[pivot as usize] {
            i += 1;
            array.swap(i as usize, j as usize);
        }
    }

    array.swap((i + 1) as usize, pivot as usize);

    i + 1
}
