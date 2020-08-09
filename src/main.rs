use sorting_benchmark::sorts;
use sorting_benchmark::timer::time_it;
use std::fs;

macro_rules! time {
    ($f: path, $name: expr, $arr: ident) => {
        let t = time_it($f, $arr.clone());
        println!(concat!($name, ":\t{:8} ms"), t.as_micros());
    };
}

fn test_on_all(arr: Vec<u32>) {
    time!(sorts::bubble_sort::bubble_sort, "Bubble Sort 1", arr);
    time!(
        sorts::bubble_sort::bubble_sort_optimized,
        "Bubble Sort 2",
        arr
    );
    time!(sorts::cycle_sort::cycle_sort, "Cycle Sort", arr);
    time!(sorts::insertion_sort::insertion_sort, "Insertion Sort", arr);
    time!(sorts::selection_sort::selection_sort, "Selection Sort", arr);
    time!(sorts::shell_sort::shell_sort, "Shell Sort", arr);
    time!(
        sorts::quicksort::basic_quicksort_median_pivot,
        "QSort Median",
        arr
    );
    time!(
        sorts::quicksort::basic_quicksort_right_most_pivot,
        "QSort Right",
        arr
    );
    time!(sorts::heapsort::heapsort, "HeapSort", arr);
    time!(sorts::mergesort::mergesort, "MergeSort", arr);
}

macro_rules! parse_vec {
    ($name: literal) => {{
        let contents = fs::read_to_string(concat!("datasets/", $name)).expect(concat!(
            "Cannot open ",
            $name,
            " dataset"
        ));
        contents
            .trim_end()
            .split_ascii_whitespace()
            .map(|i| i.parse().expect(&format!("Invalid value in data: {}", i)))
            .collect::<Vec<u32>>()
    }};
}

fn main() {
    println!("------------RANDOM------------");
    let arr = parse_vec!("random");
    test_on_all(arr);
    println!("------------DESCENDING------------");
    let arr = parse_vec!("descending");
    test_on_all(arr);
    println!("------------ASCENDING------------");
    let arr = parse_vec!("ascending");
    test_on_all(arr);
    println!("------------REPEATING------------");
    let arr = parse_vec!("repeating");
    test_on_all(arr);
}
