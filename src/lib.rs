#![allow(unused)]
pub mod sorts {
    pub mod bubble_sort;
    pub mod cocktail_sort;
    pub mod cycle_sort;
    pub mod heapsort;
    pub mod insertion_sort;
    pub mod mergesort;
    pub mod quicksort;
    pub mod selection_sort;
    pub mod shell_sort;
}

pub mod timer;

use timer::time_it;

macro_rules! once_iter {
    ($sort: path, $name: expr, $arr: ident) => {{
        let arr = $arr.clone();
        once_with(move || ($name, time_it($sort, $arr.clone())))
    }};
}

pub fn test_on_all<'a, T: Ord + Copy>(
    arr: &'a Vec<T>,
) -> impl Iterator<Item = (&'static str, std::time::Duration)> + 'a {
    use std::iter::once_with;
    once_iter!(sorts::bubble_sort::naive_bubble_sort, "Bubble Sort 0", arr)
        .chain(once_iter!(
            sorts::bubble_sort::bubble_sort,
            "Bubble Sort 1",
            arr
        ))
        .chain(once_iter!(
            sorts::bubble_sort::bubble_sort_optimized,
            "Bubble Sort 2",
            arr
        ))
        .chain(once_iter!(
            sorts::cocktail_sort::naive_cocktail_sort,
            "Cocktail Sort 0",
            arr
        ))
        .chain(once_iter!(
            sorts::cocktail_sort::cocktail_sort,
            "Cocktail Sort 1",
            arr
        ))
        .chain(once_iter!(
            sorts::cocktail_sort::cocktail_sort_optimized,
            "Cocktail Sort 2",
            arr
        ))
        .chain(once_iter!(sorts::cycle_sort::cycle_sort, "Cycle Sort", arr))
        .chain(once_iter!(
            sorts::insertion_sort::insertion_sort,
            "Insertion Sort",
            arr
        ))
        .chain(once_iter!(
            sorts::selection_sort::selection_sort,
            "Selection Sort",
            arr
        ))
        .chain(once_iter!(sorts::shell_sort::shell_sort, "Shell Sort", arr))
        .chain(once_iter!(
            sorts::quicksort::basic_quicksort_median_pivot,
            "QSort Median",
            arr
        ))
        .chain(once_iter!(
            sorts::quicksort::basic_quicksort_right_most_pivot,
            "QSort Right",
            arr
        ))
        .chain(once_iter!(
            sorts::quicksort::quicksort_three_way_partition,
            "QSort 3-Way",
            arr
        ))
        .chain(once_iter!(
            sorts::quicksort::quicksort_insertion_below_threshold,
            "QSort Insert",
            arr
        ))
        .chain(once_iter!(sorts::heapsort::heapsort, "HeapSort", arr))
        .chain(once_iter!(sorts::mergesort::mergesort, "MergeSort", arr))
}
