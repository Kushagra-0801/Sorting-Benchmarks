use sorting_benchmark::test_on_all;
use std::fs;

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
    for (algo, time) in test_on_all(arr) {
        println!("{}:\t{:8} ms", algo, time.as_micros());
    }
    println!("------------DESCENDING------------");
    let arr = parse_vec!("descending");
    for (algo, time) in test_on_all(arr) {
        println!("{}:\t{:8} ms", algo, time.as_micros());
    }
    println!("------------ASCENDING------------");
    let arr = parse_vec!("ascending");
    for (algo, time) in test_on_all(arr) {
        println!("{}:\t{:8} ms", algo, time.as_micros());
    }
    println!("------------REPEATING------------");
    let arr = parse_vec!("repeating");
    for (algo, time) in test_on_all(arr) {
        println!("{}:\t{:8} ms", algo, time.as_micros());
    }
}
