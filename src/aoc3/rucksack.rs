use std::{hash::Hash};

use super::bitset::{BitSet, BitSetIndex};

#[derive(Debug,Clone,Copy,Hash,Eq,PartialEq)]
pub struct Item
{
    prio: u8,
}

#[derive(Debug)]
pub struct Pocket
{
    items: Vec<Item>,
}

#[derive(Debug)]
pub struct Rucksack
{
    left_pocket: Pocket,
    right_pocket: Pocket,
}

impl Rucksack {
    fn new(left_pocket: Pocket, right_pocket: Pocket) -> Rucksack
    {
        Rucksack
        {
            left_pocket: left_pocket,
            right_pocket: right_pocket,
        }
    }

    fn new_empty() -> Rucksack
    {
        Rucksack::new(Pocket::new_empty(), Pocket::new_empty())
    }

    fn to_bitset(&self) -> Result<BitSet, anyhow::Error>
    {
        let left_set = self.left_pocket.to_bitset()?;
        let right_set = self.right_pocket.to_bitset()?;
        Ok(BitSet::extend(&left_set, &right_set))
    }

    pub fn find_duplicates_in_pockets(rucksack: &Rucksack) -> Result<Vec<Item>, anyhow::Error>
    {
        let left_set = rucksack.left_pocket.to_bitset()?;
        let right_set = rucksack.right_pocket.to_bitset()?;
        let duplicate_set = BitSet::intersect(&left_set, &right_set);
        
        let duplicate_items = duplicate_set.get_set_bits().into_iter()
            .map(|prio| Item::new(prio))
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(duplicate_items)
    }

    pub fn find_duplicates_in_rucksacks(rucksacks: &[Rucksack]) -> Result<Vec<Item>, anyhow::Error>
    {
        let duplicate_prios = rucksacks.iter()
            .map(|rucksack| rucksack.to_bitset())
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .reduce(|a,b| BitSet::intersect(&a, &b)).ok_or(anyhow::Error::msg(""))
            .map(|set| set.get_set_bits())?;
        
        duplicate_prios.into_iter()
            .map(|prio| Item::new(prio))
            .collect::<Result<Vec<_>, _>>()
    }
}

impl Pocket
{
    fn new(items: Vec<Item>) -> Pocket
    {
        Pocket { items: items }
    }

    fn new_empty() -> Pocket
    {
        Pocket::new(Vec::new())
    }

    fn to_bitset(&self) -> Result<BitSet, anyhow::Error>
    {
        self.items.iter()
            .map(|item| BitSetIndex::new(item.get_prio()))
            .collect::<Result<BitSet, _>>()
    }
}

impl Item
{
    fn new(prio: u8) -> Result<Item, anyhow::Error>
    {
        match Item::is_valid(prio)
        {
            true => Ok(Item{prio: prio}),
            false => Err(anyhow::Error::msg(format!("prio '{prio}' is not valid for item!")))
        }
    }

    pub fn get_prio(&self) -> u8 { self.prio }

    pub fn to_priority(char: char) -> Option<u8>
    {
        match char
        {
            'a'..='z' => Some(1 + (char as u8 - 'a' as u8)),
            'A'..='Z' => Some(27 + (char as u8 - 'A' as u8)),
            _ => None,
        }
    }

    fn is_valid(prio: u8) -> bool
    {
        match prio
        {
            1..=52 => true,
            _ => false,
        }
    }
}

pub fn parse_rucksack(rucksack_str: &str) -> Result<Rucksack, anyhow::Error>
{
    if rucksack_str.len() % 2 == 1
    {
        return Err(anyhow::Error::msg("The rucksack string '{rucksack_str}' needs to be divisible in half."));
    }

    if rucksack_str.is_empty()
    {
        return Ok(Rucksack::new_empty());
    }

    let rucksack_split = rucksack_str.split_at(rucksack_str.len() / 2);

    Ok(Rucksack::new(parse_pocket(rucksack_split.0)?, parse_pocket(rucksack_split.1)?))
}

fn parse_pocket(pocket_str: &str) -> Result<Pocket, anyhow::Error>
{
    let item_vec = pocket_str.chars()
        .into_iter()
        .map(|char| parse_item(char))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Pocket::new(item_vec))
}

fn parse_item(char: char) -> Result<Item, anyhow::Error>
{
    match Item::to_priority(char)
    {
        Some(prio) => Item::new(prio),
        None => Err(anyhow::Error::msg(format!("char {char} is not a valid item"))),
    }
}

#[cfg(test)]
mod tests
{
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn duplicate_items()
    {
        let rucksack = Rucksack::new(
            parse_pocket("aAbB").unwrap(),
            parse_pocket("cCdD").unwrap(),
        );

        assert_eq!(
            diff(
                &Rucksack::find_duplicates_in_pockets(&rucksack).unwrap(),
                &Vec::<Item>::new()
            ).is_empty(), true);

        let rucksack = Rucksack::new(
            parse_pocket("aAbBCC").unwrap(),
            parse_pocket("cCdD").unwrap(),
        );

        let duplicates = Rucksack::find_duplicates_in_pockets(&rucksack);
        println!("{:?}", duplicates);
        assert_eq!(
            diff(&duplicates.unwrap(), &Vec::<Item>::new()).len() == 1,
            true
        );
        
        let rucksack = Rucksack::new(
            parse_pocket("aAbBCC").unwrap(),
            parse_pocket("cCdDb").unwrap(),
        );

        assert_eq!(
            diff(
                &Rucksack::find_duplicates_in_pockets(&rucksack).unwrap(),
                &Vec::<Item>::new()
            ).len() == 2,
            true
        );
    }

    #[test]
    fn priorities() -> Result<(), anyhow::Error>
    {
        let items = ('a'..='z').chain('A'..='Z')
            .map(|char| Item::new(Item::to_priority(char).unwrap()))
            .collect::<Result<Vec<_>, _>>()?;

        for (index, item) in items.iter().enumerate()
        {
            let priority = index as u8 + 1;
            println!("{:?} == {priority}", item);
            assert_eq!(item.get_prio(), priority);
        }
        
        Ok(())
    }

    fn diff<T: Copy + Hash + Eq>(vec1: &Vec<T>, vec2: &Vec<T>) -> Vec<T>
    {
        let set1 = vec1.iter().copied().collect::<HashSet<_>>();
        let set2 = vec2.iter().copied().collect::<HashSet<_>>();
        set1.difference(&set2).copied().collect::<Vec<_>>()
    }
}