mod day1;

fn main() {
    println!("Advent of code 2024 in Rust!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn day1_assignment_1() {
        if let Ok(file) = File::open("./src/resources/day1_1_input.txt") {
            let total = day1::find_total_distance(file);
            assert_eq!(total, 1222801);
        }
    }
    #[test]
    fn day1_assignment_2() {
        if let Ok(file) = File::open("./src/resources/day1_1_input.txt") {
            let total = day1::find_similarity_score(file);
            assert_eq!(total, 22545250);
        }
    }
}
