use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn has_match_count(s: &String, goal: i32) -> bool {
    let mut counts = HashMap::new();

    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    counts.values().any(|count| *count == goal)
}

fn has_double(s: &String) -> bool {
    has_match_count(s, 2)
}

fn has_triple(s: &String) -> bool {
    has_match_count(s, 3)
}

struct MatchCount {
    doubles: i32,
    triples: i32,
}

fn calculate_checksum(ids: Vec<String>) -> i32 {
    let counts = ids.iter().fold(MatchCount { doubles: 0, triples: 0 }, |acc, s| {
        MatchCount {
            doubles: acc.doubles + if has_double(&s) { 1 } else { 0 },
            triples: acc.triples + if has_triple(&s) { 1 } else { 0 }
        }
    });

    counts.doubles * counts.triples
}

fn get_input() -> Vec<String> {
    let mut f = File::open("input.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong when reading the file");

    contents.split('\n')
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .map(|line| line.to_owned())
        .collect::<Vec<String>>()
}

fn main() {

    let input = get_input();

    println!("Checksum: {}", calculate_checksum(input));


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_checksum() {
        assert_eq!(calculate_checksum(vec!["abcdef".to_owned(), "bababc".to_owned(), "abbcde".to_owned(), "abcccd".to_owned(), "aabcdd".to_owned(), "abcdee".to_owned(), "ababab".to_owned()]), 12)
    }

    #[test]
    fn no_match() {
        assert_eq!(has_double(&String::from("abcdef")), false);
        assert_eq!(has_triple(&String::from("abcdef")), false);
    }

    #[test]
    fn double_triple_match() {
        assert_eq!(has_double(&String::from("bababc")), true);
        assert_eq!(has_triple(&String::from("bababc")), true);
    }

    #[test]
    fn only_double_match() {
        assert_eq!(has_double(&String::from("abbcde")), true);
        assert_eq!(has_triple(&String::from("abbcde")), false);
    }

    #[test]
    fn only_triple_match() {
        assert_eq!(has_double(&String::from("abcccd")), false);
        assert_eq!(has_triple(&String::from("abcccd")), true);
    }

    #[test]
    fn two_double_match() {
        assert_eq!(has_double(&String::from("aabcdd")), true);
        assert_eq!(has_triple(&String::from("aabcdd")), false);
    }

    #[test]
    fn simple_double_match() {
        assert_eq!(has_double(&String::from("abcdee")), true);
        assert_eq!(has_triple(&String::from("abcdee")), false);
    }

    #[test]
    fn two_triple_match() {
        assert_eq!(has_double(&String::from("ababab")), false);
        assert_eq!(has_triple(&String::from("ababab")), true);
    }
}