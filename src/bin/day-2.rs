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
            PasswordPolicy::new(
                captures[1].parse().unwrap(),
                captures[2].parse().unwrap(),
                captures[3].chars().next().unwrap(),
            ),
            captures[4].to_string(),
        )
    });

    let valid_passwords = input
        .filter(|(policy, password)| policy.check_password(password))
        .count();
    println!("Valid passwords: {}", valid_passwords);
    Ok(())
}

#[derive(Debug)]
struct PasswordPolicy {
    count: std::ops::Range<usize>,
    letter: char,
}

impl PasswordPolicy {
    fn new(min_count: usize, max_count: usize, letter: char) -> Self {
        PasswordPolicy {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_password_policy() {
        let policy = PasswordPolicy {
            count: std::ops::Range { start: 1, end: 6 },
            letter: 'a',
        };
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
}
