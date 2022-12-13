mod rucksack;
mod bitset;
use rucksack::*;

pub fn run(input: String) -> Result<String, anyhow::Error>
{
    let rucksacks = input.lines()
        .map(|line| parse_rucksack(line))
        .collect::<Result<Vec<_>, _>>()?;
    
    if rucksacks.is_empty() {
        return Err(anyhow::Error::msg("There are no rucksacks!"));
    }

    // Part 1
    let _duplicates = rucksacks.iter()
        .map(|rucksack| Rucksack::find_duplicates_in_pockets(&rucksack))
        .collect::<Result<Vec<_>, _>>()?
        .iter().flatten().copied().collect::<Vec<_>>();
    
    // Part 2
    if rucksacks.len() % 3 != 0 {
        return Err(anyhow::Error::msg("There needs to be a multiple of 3 number of rucksacks!"));
    }

    let duplicates = rucksacks.chunks_exact(3)
        .map(|rucksacks| Rucksack::find_duplicates_in_rucksacks(rucksacks))
        .collect::<Result<Vec<_>, _>>()?
        .iter().flatten().copied().collect::<Vec<_>>();
    
    let priorities = duplicates.iter()
        .map(|item| item.get_prio())
        .collect::<Vec<_>>();

    let sum_of_priorities = priorities.iter().fold(0u32, |sum, x| sum + (*x) as u32);

    Ok(sum_of_priorities.to_string())
}
