mod user_struct {
    #[derive(Clone, Copy, Eq)]
    pub struct DataType {}

    impl PartialEq for DataType {
        fn eq(&self, other: &Self) -> bool {
            todo!()
        }
    }

    impl Ord for DataType {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            todo!()
        }
    }

    impl PartialOrd for DataType {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl std::str::FromStr for DataType {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            todo!()
        }
    }
}

use sorting_benchmark::test_on_all;
use user_struct::DataType;

use std::env;
use std::fs;

fn main() {
    let mut args = env::args().skip(1);
    let data_file_path = args.next().unwrap();
    let delimiter = args.next().unwrap();

    let arr: Vec<DataType> = {
        let contents = fs::read_to_string(data_file_path).expect("File not found");
        contents
            .trim_end()
            .split(&delimiter)
            .map(|e| e.parse().unwrap())
            .collect()
    };
    for (name, dur) in test_on_all(arr) {
        println!("{}:\t{:8} ms", name, dur.as_micros());
    }
}
