use std::fs::File;
use std::io::Read;

fn get_diff_count(s1: &String, s2: &String) -> i32 {

    let common_length = get_common_string(s1, s2).len();

    let diff_count = s1.len() - common_length;

    diff_count as i32
}

fn get_common_string(s1: &String, s2: &String) -> String {
    s1.chars().zip(s2.chars()).filter(|(l, r)| *l == *r).map(|(_l, r)| r).collect()
}

fn calculate_common_string(ids: &Vec<String>) -> String {
    for s1 in ids {
        for s2 in ids {
            if get_diff_count(&s1, &s2) == 1 {
                return get_common_string(&s1, &s2)
            }
        }
    }
    panic!("Didn't find value")
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

    println!("common string: {}", calculate_common_string(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diff_count_2() {
        assert_eq!(get_diff_count(&String::from("abcde"), &String::from("axcye")), 2)
    }

    #[test]
    fn diff_count_1() {
        assert_eq!(get_diff_count(&String::from("fghij"), &String::from("fguij")), 1)
    }

    #[test]
    fn diff_count_0() {
        assert_eq!(get_diff_count(&String::from("abcdef"), &String::from("abcdef")), 0)
    }

    #[test]
    fn common_string() {
        assert_eq!(get_common_string(&String::from("fghij"), &String::from("fguij")), String::from("fgij"))
    }

    #[test]
    fn calculate_common() {
        assert_eq!(calculate_common_string(vec![String::from("abcde"),String::from("fghij"),String::from("klmno"),String::from("pqrst"),String::from("fguij"),String::from("axcye"),String::from("wvxyz")]), String::from("fgij"))
    }
}