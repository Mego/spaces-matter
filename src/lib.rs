use once_cell::sync::Lazy;
use regex_lite::{Captures, Regex};

static SPACES_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s(\s*)").unwrap());

static OPERATION_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+)([*/%^+-])(\d+)").unwrap());

static EXPRESSION_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\s*\d+(\s*[*/%^+-]\s*\d+)*\s*$").unwrap());

static EVALUATED_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s*\d+\s*$").unwrap());

pub fn parse(mut s: String) -> i64 {
    assert!(EXPRESSION_REGEX.is_match(&s));

    while !EVALUATED_REGEX.is_match(&s) {
        while let evaluated = OPERATION_REGEX.replace(&s, |c: &Captures<'_>| {
            let (_, [l, op, r]) = c.extract();
            let lv: i64 = l.parse().unwrap();
            let rv: i64 = r.parse().unwrap();
            match op {
                "+" => lv + rv,
                "-" => lv - rv,
                "*" => lv * rv,
                "/" => lv / rv,
                "%" => lv % rv,
                "^" => lv.pow(rv as u32),
                _ => unreachable!(),
            }
            .to_string()
        }) && evaluated != s
        {
            s = evaluated.into_owned();
        }
        s = SPACES_REGEX.replace_all(&s, "$1").into_owned();
    }

    s.trim().parse().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_equivalent() {
        for test_case in ["3 +4*6 +7", "3 +4*6+ 7", "3+ 4*6 +7", "3+ 4*6+ 7"] {
            assert_eq!(parse(test_case.to_owned()), 34);
        }
        for test_case in ["1 +2 +3 +4 +5", "1+ 2+ 3+ 4+ 5"] {
            assert_eq!(parse(test_case.to_owned()), 15);
        }
    }

    #[test]
    fn tests() {
        let test_cases = [
            ("1+ 2+ 3  *4", 24),
            ("4*  3 +2 +1", 24),
            ("1* 2*  3*4   +6", 30),
            ("1 * 2^3  + 4", 12),
            ("1+2+3", 6),
            ("1 * 2+3 *4 +1 ^  2*4 + 4", 7355827511386641),
        ];
        for (test_case, expected) in test_cases {
            assert_eq!(parse(test_case.to_owned()), expected);
        }
    }
}
