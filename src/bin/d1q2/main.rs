const TARGETS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const REVERSED_TARGETS: [&str; 10] = [
    "orez", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
];

pub fn main() {
    advent_of_code_2023::util::qn(|value| {
        let s = value
            .trim_end()
            .split("\n")
            .map(|s| {
                let mut pre = String::new();
                let mut post = String::new();
                let mut first_numeric: Option<u32> = None;
                let mut last_numeric: Option<u32> = None;
                let mut pre_is_done = false;
                for c in s.chars() {
                    if c.is_numeric() {
                        let digit = c.to_digit(10);
                        if first_numeric.is_none() {
                            first_numeric = digit.clone();
                        }
                        last_numeric = digit.clone();
                        pre_is_done = true;
                        post = String::new();
                    } else {
                        if pre_is_done {
                            post.push(c);
                        } else {
                            pre.push(c);
                        }
                    }
                }

                if let Some((value, _)) = TARGETS
                    .iter()
                    .enumerate()
                    .filter_map(|(value, t)| pre.find(t).map(|char_index| (value, char_index)))
                    .min_by(|a, b| a.1.cmp(&b.1))
                {
                    first_numeric = Some(value as u32);
                }

                let reversed_post = post.chars().rev().collect::<String>();
                if let Some((value, _)) = REVERSED_TARGETS
                    .iter()
                    .enumerate()
                    .filter_map(|(value, t)| {
                        reversed_post.find(t).map(|char_index| (value, char_index))
                    })
                    .min_by(|a, b| a.1.cmp(&b.1))
                {
                    last_numeric = Some(value as u32);
                }

                return format!("{}{}", first_numeric.unwrap(), last_numeric.unwrap())
                    .parse::<u32>()
                    .unwrap();
            })
            .reduce(|acc, n| acc + n)
            .unwrap();

        return s.to_string();
    });
}
