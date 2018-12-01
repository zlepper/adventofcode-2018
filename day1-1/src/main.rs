use std::fs::File;
use std::io::prelude::*;

fn calculate_frequency(numbers: Vec<i32>) -> i32 {
    numbers.iter().fold(0, |acc, x| acc + x)
}

fn main() {
    let mut f = File::open("input.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong when reading the file");

    let vec = contents.split('\n').collect::<Vec<&str>>();

    let numbers = vec.into_iter().filter(|line| line.len() > 0).map(|line: &str| -> i32 {
        if line.starts_with('-') {
            line.parse::<i32>().expect("Not a proper number")
        } else {
            line.chars().skip(1).collect::<String>().parse::<i32>().expect("Not a number")
        }
    }).collect::<Vec<i32>>();

    println!("Result: {}", calculate_frequency(numbers));


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn freq1() {
        assert_eq!(calculate_frequency(vec![1, -2, 3, 1]), 3);
    }

    #[test]
    fn freq2() {
        assert_eq!(calculate_frequency(vec![1, 1, 1]), 3);
    }

    #[test]
    fn freq3() {
        assert_eq!(calculate_frequency(vec![1, 1, -2]), 0);
    }

    #[test]
    fn freq4() {
        assert_eq!(calculate_frequency(vec![-1, -2, -3]), -6);
    }
}