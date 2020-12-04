use regex::Regex;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), &'static str> {
    let policy_matcher = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").expect("regex format error");
    let input = BufReader::new(std::io::stdin()).lines().map(|line| {
        let line = line.unwrap();
        let captures = policy_matcher
            .captures(&line)
            .expect("valid password policy and password");
        (
            RangePasswordPolicy::new(
                captures[1].parse().unwrap(),
                captures[2].parse().unwrap(),
                captures[3].chars().next().unwrap(),
            ),
            PositionPasswordPolicy::new(
                captures[1].parse().unwrap(),
                captures[2].parse().unwrap(),
                captures[3].chars().next().unwrap(),
            ),
            captures[4].to_string(),
        )
    });

    let valid_passwords = input.fold(
        (0, 0),
        |mut acc, (range_policy, position_policy, password)| {
            if range_policy.check_password(&password) {
                acc.0 += 1
            }
            if position_policy.check_password(&password) {
                acc.1 += 1
            }
            acc
        },
    );

    println!("Valid range passwords: {}", valid_passwords.0);
    println!("Valid position passwords: {}", valid_passwords.1);

    Ok(())
}

#[derive(Debug)]
struct RangePasswordPolicy {
    count: std::ops::Range<usize>,
    letter: char,
}

impl RangePasswordPolicy {
    fn new(min_count: usize, max_count: usize, letter: char) -> Self {
        RangePasswordPolicy {
            count: std::ops::Range {
                start: min_count,
                end: max_count + 1,
            },
            letter,
        }
    }

    fn check_password(&self, password: &str) -> bool {
        self.count
            .contains(&password.chars().filter(|ch| ch == &self.letter).count())
    }
}

#[derive(Debug)]
struct PositionPasswordPolicy {
    indices: [usize; 2],
    letter: char,
}

impl PositionPasswordPolicy {
    fn new(first_position: usize, last_position: usize, letter: char) -> Self {
        PositionPasswordPolicy {
            indices: [first_position, last_position],
            letter,
        }
    }

    fn check_password(&self, password: &str) -> bool {
        let first_position = password.chars().nth(self.indices[0] - 1);
        let last_position = password.chars().nth(self.indices[1] - 1);

        match (
            first_position == Some(self.letter),
            last_position == Some(self.letter),
        ) {
            (true, false) | (false, true) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range_password_policy() {
        let policy = RangePasswordPolicy::new(1, 5, 'a');
        let cases = [
            ("", false),
            ("a", true),
            ("aa", true),
            ("aaa", true),
            ("aaaa", true),
            ("aaaaa", true),
            ("aaaaaa", false),
            ("aaaaaaa", false),
            ("baaaaa", true),
            ("aaaaab", true),
            ("abacadaeaf", true),
            ("abacadaeafagah", false),
            ("bcdef", false),
        ];

        for (case, result) in cases.iter() {
            assert_eq!(
                policy.check_password(case),
                *result,
                "Checking password '{}' against policy {:?}",
                case,
                policy
            );
        }
    }

    #[test]
    fn test_position_password_policy() {
        let cases = [
            (PositionPasswordPolicy::new(1, 3, 'a'), "abcde", true),
            (PositionPasswordPolicy::new(1, 3, 'b'), "cdefg", false),
            (PositionPasswordPolicy::new(2, 9, 'c'), "ccccccccc", false),
        ];

        for (policy, password, result) in cases.iter() {
            assert_eq!(
                policy.check_password(password),
                *result,
                "Checking password '{}' against policy {:?}",
                password,
                policy
            );
        }
    }
}
