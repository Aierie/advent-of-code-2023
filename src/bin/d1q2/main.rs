use nom::FindSubstring;

const TARGETS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const REVERSED_TARGETS: [&str; 10] = [
    "orez", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
];

pub fn main() {
    advent_of_code_2023::util::qn(|value| {
        let s = value
            .split("\n")
            .map(|s| {
                // let first_word = TARGETS
                //     .iter()
                //     .enumerate()
                //     .filter_map(|(index_in_array, t)| {
                //         s.find_substring(t)
                //             .map(|char_index| (index_in_array, char_index))
                //     })
                //     .min_by(|a, b| a.1.cmp(&b.1));

                // let last_word = TARGETS
                //     .iter()
                //     .enumerate()
                //     .filter_map(|(index_in_array, t)| {
                //         s.find_substring(t)
                //             .map(|char_index| (index_in_array, char_index))
                //     })
                //     .min_by(|a, b| a.1.cmp(&b.1));
                let trimmed = s
                    .trim_matches(|c: char| !c.is_numeric())
                    .trim_end_matches(|c: char| !c.is_numeric());

                let iter = trimmed.chars();
                let reversed = iter.clone().rev();
                let first = iter.take(1).next().unwrap();
                let last = reversed.take(1).next().unwrap();

                return format!("{first}{last}").parse::<u32>().unwrap();
            })
            .reduce(|acc, n| acc + n)
            .unwrap();

        return s.to_string();
    });
}
