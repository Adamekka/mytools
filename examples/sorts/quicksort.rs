fn main() {
    let mut array = vec![5, 4, 3, 2, 1];
    mytools::sorts::quicksort(&mut array);
    println!("{array:?}");
}
