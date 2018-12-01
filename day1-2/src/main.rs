use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;


fn find_first_repeat(numbers: Vec<i32>) -> i32 {
    let mut seen = HashSet::new();
    seen.insert(0);

    numbers.iter().cycle()
        .scan(0, |acc, x| {
            *acc = *acc + x;
            Some(*acc)
        })
        .map(|frequency| frequency.to_owned())
        .skip_while(|frequency| -> bool {
            if seen.contains(frequency) {
                return false
            }
            seen.insert(frequency.clone());
            true
        })
        .take(1)
        .collect::<Vec<i32>>()[0]
}

fn main() {
    let mut f = File::open("input.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong when reading the file");

    let vec = contents.split('\n').collect::<Vec<&str>>();

    let numbers = vec.into_iter()
        .filter(|line| line.len() > 0)
        .map(|line: &str| -> i32 {
            if line.starts_with('-') {
                line.parse::<i32>().expect("Not a proper number")
            } else {
                line.chars().skip(1).collect::<String>().parse::<i32>().expect("Not a number")
            }
        }).collect::<Vec<i32>>();

    println!("Result: {}", find_first_repeat(numbers))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn freq1() {
        assert_eq!(find_first_repeat(vec![1, -1]), 0)
    }

    #[test]
    fn freq2() {
        assert_eq!(find_first_repeat(vec![3, 3, 4, -2, -4]), 10)
    }

    #[test]
    fn freq3() {
        assert_eq!(find_first_repeat(vec![-6, 3, 8, 5, -6]), 5)
    }

    #[test]
    fn freq4() {
        assert_eq!(find_first_repeat(vec![7, 7, -2, -7, 4]), 14)
    }
}
