use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1, take_while_m_n},
    character::complete::{crlf, digit1, multispace0, newline},
    combinator::map_res,
    multi::separated_list1,
    sequence::{delimited, pair, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
struct Game {
    shown: Vec<BallCollection>,
    id: u32,
}
impl Game {
    fn limits(&self) -> BallCollection {
        self.shown.iter().fold(
            BallCollection {
                red: 0,
                green: 0,
                blue: 0,
            },
            |acc, current| BallCollection {
                red: acc.red.max(current.red),
                green: acc.green.max(current.green),
                blue: acc.blue.max(current.blue),
            },
        )
    }
}

#[derive(Debug, PartialEq)]
struct BallCollection {
    red: u32,
    green: u32,
    blue: u32,
}

fn from_str(input: &str) -> Result<u32, std::num::ParseIntError> {
    input.parse::<u32>()
}

fn count_balls(input: &str) -> IResult<&str, BallCollection> {
    let (input, items) = separated_list1(
        tag(","),
        pair(
            delimited(multispace0, map_res(digit1, from_str), multispace0),
            alt((tag("red"), tag("green"), tag("blue"))),
        ),
    )(input)?;
    let mut hashed = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
    items.iter().for_each(|(v, k)| {
        assert!(hashed.contains_key(k));
        hashed.insert(k, *v);
    });

    Ok((
        input,
        BallCollection {
            red: *hashed.get("red").unwrap(),
            green: *hashed.get("green").unwrap(),
            blue: *hashed.get("blue").unwrap(),
        },
    ))
}

fn game_id(input: &str) -> IResult<&str, u32> {
    delimited(multispace0, map_res(digit1, from_str), tag(":"))(input)
}

fn parse_one_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game")(input)?;
    let (input, id) = game_id(input)?;
    let (input, shown) = separated_list1(tag(";"), count_balls)(input)?;

    Ok((input, Game { shown, id }))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(crlf, parse_one_game)(input)
}

pub fn main() {
    advent_of_code_2023::util::qn(|value| {
        // Determine which games would have been possible if the bag had been loaded with only 12 red cubes,
        // 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?
        let blue_limit = 14;
        let green_limit = 13;
        let red_limit = 12;
        let (remaining, games) = parse_games(&value).unwrap();
        assert!(remaining.is_empty());

        let answer: u32 = games
            .iter()
            .filter_map(|g| {
                let v: BallCollection = g.limits();
                if v.blue > blue_limit || v.green > green_limit || v.red > red_limit {
                    return None;
                }

                Some(g.id)
            })
            .sum();
        answer.to_string()
    });
}
