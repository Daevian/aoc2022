use super::round::{Round, Choice, Their, Your, RoundResult};

pub fn parse_round(round_line: &str) -> Result<Round, anyhow::Error>
{
    // first puzzle
    // let to_round =
    //     |(their, your)|
    //     parse_choice(their)
    //     .and_then(
    //         |their|
    //         parse_choice(your)
    //             .map(|your| Round::new(Their(their), Your(your))
    //         )
    //     );

    // second puzzle
    let to_round =
        |(their, result)|
        parse_choice(their)
        .and_then(
            |their|
            parse_wanted_result(result)
                .map(|result| their.get_choice_for_result(result))
                .and_then(|your| Ok(Round::new(Their(their), Your(your)))
            )
        );

    round_line
        .split_once(' ')
        .map_or_else(
            || Err(anyhow::Error::msg(format!("couldn't split line '{round_line}'"))),
            to_round
        )
}

fn parse_choice(choice_str: &str) -> Result<Choice, anyhow::Error>
{
    match choice_str.trim()
    {
        "A" | "X" => Ok(Choice::Rock),
        "B" | "Y" => Ok(Choice::Paper),
        "C" | "Z" => Ok(Choice::Scissors),
        _ => Err(anyhow::Error::msg(format!("'{choice_str}' is not a valid choice."))),
    }
}

fn parse_wanted_result(result_str: &str) -> Result<RoundResult, anyhow::Error>
{
    match result_str.trim()
    {
        "X" => Ok(RoundResult::Lose),
        "Y" => Ok(RoundResult::Draw),
        "Z" => Ok(RoundResult::Win),
        _ => Err(anyhow::Error::msg(format!("'{result_str}' is not a valid result."))),
    }
}