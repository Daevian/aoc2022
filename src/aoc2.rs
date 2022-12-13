mod round;
mod parse;

pub fn run(input: String) -> Result<String, anyhow::Error>
{
    let rounds: Vec<_> =
        input.lines()
        .map(|line| parse::parse_round(line))
        .collect::<Result<Vec<_>, _>>()?;

    let total_score =
        rounds.iter()
        .map(|round| round::calc_round_score(round))
        .fold(0, |sum, score| sum + score);

    Ok(total_score.to_string())
}

