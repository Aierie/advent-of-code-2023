pub fn main() {
    advent_of_code_2023::util::qn(|value| {
        let s = value
            .split("\n")
            .map(|s| {
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
