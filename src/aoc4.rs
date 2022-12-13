use anyhow::Ok;


pub fn run(input: String) -> Result<String, anyhow::Error>
{
    let pairs = parse_elf_pairs(&input)?;

    // Part 1
    let _num_fully_contained_pairs = pairs.iter()
        .fold(0, |sum, pair| sum + if ElfPair::are_any_fully_contained(&pair) { 1 } else { 0 } );

    // Part 2
    let num_overlapping_pairs = pairs.iter()
        .fold(0, |sum, pair| sum + if ElfPair::are_any_overlapping(&pair) { 1 } else { 0 } );

    Ok(num_overlapping_pairs.to_string())
}

#[derive(Debug)]
struct Assignment
{
    min: u8,
    max: u8,
}

#[derive(Debug)]
struct ElfPair(Assignment, Assignment);

impl ElfPair {
    fn are_any_fully_contained(pair: &ElfPair) -> bool
    {
        let first_is_contained = (pair.1.min <= pair.0.min) && (pair.1.max >= pair.0.max);
        let second_is_contained = (pair.0.min <= pair.1.min) && (pair.0.max >= pair.1.max);
        first_is_contained || second_is_contained
    }

    fn are_any_overlapping(pair: &ElfPair) -> bool
    {
        (pair.1.min <= pair.0.max) && (pair.0.min <= pair.1.max)
    }
}

fn parse_elf_pairs(input: &String) -> Result<Vec<ElfPair>, anyhow::Error>
{
    input.lines()
        .map(|line| parse_elf_pair(line))
        .collect::<Result<Vec<_>, _>>()
}

fn parse_elf_pair(pair_str: &str) -> Result<ElfPair, anyhow::Error>
{
    let split_str = pair_str.split_once(',')
        .ok_or_else(|| anyhow::Error::msg(format!("couldn't split pair {pair_str}")))?;
    
    let min = parse_assignment(split_str.0)?;
    let max = parse_assignment(split_str.1)?;
    Ok(ElfPair(min, max))
}

fn parse_assignment(assignment_str: &str) -> Result<Assignment, anyhow::Error>
{
    let ass_strs = assignment_str.split_once('-')
        .ok_or_else(|| anyhow::Error::msg(format!("couldn't split assignment {assignment_str}")))?;
    
    ass_strs.0.parse::<u8>()
        .map_err(|e| e.into())
        .and_then(
            |min| ass_strs.1.parse::<u8>()
                .map(|max| Assignment{min: min, max: max})
                .map_err(|e| e.into())
        )
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn fully_contained()
    {
        assert_eq!(ElfPair::are_any_fully_contained(&parse_elf_pair("2-4,6-8").unwrap()), false);
        assert_eq!(ElfPair::are_any_fully_contained(&parse_elf_pair("2-3,4-5").unwrap()), false);
        assert_eq!(ElfPair::are_any_fully_contained(&parse_elf_pair("5-7,7-9").unwrap()), false);
        assert_eq!(ElfPair::are_any_fully_contained(&parse_elf_pair("2-8,3-7").unwrap()), true);
        assert_eq!(ElfPair::are_any_fully_contained(&parse_elf_pair("6-6,4-6").unwrap()), true);
        assert_eq!(ElfPair::are_any_fully_contained(&parse_elf_pair("2-6,4-8").unwrap()), false);
    }

    #[test]
    fn overlapping()
    {
        assert_eq!(ElfPair::are_any_overlapping(&parse_elf_pair("2-4,6-8").unwrap()), false);
        assert_eq!(ElfPair::are_any_overlapping(&parse_elf_pair("2-3,4-5").unwrap()), false);
        assert_eq!(ElfPair::are_any_overlapping(&parse_elf_pair("5-7,7-9").unwrap()), true);
        assert_eq!(ElfPair::are_any_overlapping(&parse_elf_pair("2-8,3-7").unwrap()), true);
        assert_eq!(ElfPair::are_any_overlapping(&parse_elf_pair("6-6,4-6").unwrap()), true);
        assert_eq!(ElfPair::are_any_overlapping(&parse_elf_pair("2-6,4-8").unwrap()), true);
    }
}