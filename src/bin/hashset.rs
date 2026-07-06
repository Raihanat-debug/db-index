use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    // Number of operations
    let n: usize = iter.next().unwrap().parse().unwrap();

    let mut index = HashSet::<i32>::new();

    for _ in 0..n {
        let operation = iter.next().unwrap();
        let value: i32 = iter.next().unwrap().parse().unwrap();

        match operation {
            "INSERT" => {
                index.insert(value);
            }

            "FIND" => {
                if index.contains(&value) {
                    println!("FOUND");
                } else {
                    println!("NOT FOUND");
                }
            }

            _ => {}
        }
    }
}