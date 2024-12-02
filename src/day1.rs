use std::{
    collections::HashMap,
    io::{BufReader, Read},
};

pub fn find_total_distance(file: std::fs::File) -> i32 {
    // Init the left and right array
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    // Read the file into a buffer
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("Unable to read to end");
    let collected_lines: Vec<String> = contents.lines().map(String::from).collect();

    for line in collected_lines {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        if numbers.len() == 2 {
            left.push(numbers[0]);
            right.push(numbers[1]);
        }
    }

    // Sort the lists
    left.sort_by(|a, b| a.cmp(b));
    right.sort_by(|a, b| a.cmp(b));

    let mut total: i32 = 0;
    for it in left.into_iter().zip(right.into_iter()) {
        if it.0 > it.1 {
            total += it.0 - it.1
        } else {
            total += it.1 - it.0;
        }
    }

    return total;
}

pub fn find_similarity_score(file: std::fs::File) -> i32 {
    // Init the left and right array
    let mut left: Vec<i32> = Vec::new();
    let mut right: HashMap<i32, i32> = HashMap::new();

    // Read the file into a buffer
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("Unable to read to end");
    let collected_lines: Vec<String> = contents.lines().map(String::from).collect();

    for line in collected_lines {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        if numbers.len() == 2 {
            left.push(numbers[0]);
            right
                .entry(numbers[1])
                .and_modify(|current| *current += 1)
                .or_insert(1);
        }
    }

    let mut total: i32 = 0;
    for it in left.into_iter() {
        if let Some(num) = right.get(&it) {
            if it == 81665 {
                println!("Times: {}", num);
            }
            total += it * num;
        }
    }

    return total;
}
