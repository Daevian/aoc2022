pub fn run(input: String) -> Result<String, anyhow::Error>
{
    let calory_lists = 
        input
        .split("\r\n\r\n")
        .map(|group|
            group
            .lines()
            .map(|line| line.trim().parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
        )
        .collect::<Result<Vec<_>, _>>()?;
    
    let mut list_sums: Vec<i32> =
        calory_lists
        .iter()
        .map(|list| list.iter().fold(0, |sum, x| sum + x))
        .collect();

    list_sums.sort_unstable_by(|a, b| b.cmp(a));

    let sum_of_max_sums = list_sums.iter().take(3).fold(0, |sum, x| sum + x);

    Ok(sum_of_max_sums.to_string())
}
