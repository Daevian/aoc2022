
#[derive(Debug,Copy,Clone)]
pub struct BitSetIndex
{
    index: u8,
}

#[derive(Debug,Copy,Clone)]
pub struct BitSet
{
    bits: u64,
}

impl BitSetIndex
{
    pub fn new(index: u8) -> Result<BitSetIndex, anyhow::Error>
    {
        match index
        {
            0..=63 => Ok(BitSetIndex{index: index}),
            _ => Err(anyhow::Error::msg("index '{index}' has to between 0 and 64")) 
        }
    }

    pub fn to_u8(&self) -> u8 { self.index }
}

impl BitSet
{
    pub fn new(bits: u64) -> BitSet
    {
        BitSet{bits: bits}
    }

    pub fn new_empty() -> BitSet
    {
        BitSet::new(0)
    }

    pub fn set(&mut self, index: &BitSetIndex)
    {
        self.bits |= 1 << index.to_u8();
    }

    pub fn get_set_bits(&self) -> Vec<u8>
    {
        (0..=63).into_iter()
            .filter(|index| self.bits & (1 << index) != 0)
            .clone()
            .collect::<Vec<_>>()
    }

    pub fn intersect(a: &BitSet, b: &BitSet) -> BitSet
    {
        BitSet::new(a.bits & b.bits)
    }

    pub fn extend(a: &BitSet, b: &BitSet) -> BitSet
    {
        BitSet::new(a.bits | b.bits)
    }
}

impl FromIterator<BitSetIndex> for BitSet
{
    fn from_iter<T: IntoIterator<Item = BitSetIndex>>(iter: T) -> BitSet
    {
        let mut set = BitSet::new_empty();
        for i in iter
        {
            set.set(&i);
        }

        set
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn bitset() -> Result<(), anyhow::Error>
    {
        let mut a = BitSet::new_empty();
        let mut b = BitSet::new_empty();

        println!("a: {:?}, b: {:?}", a, b);
        assert_eq!(
            BitSet::intersect(&a, &b).get_set_bits().len() == 0,
            true
        );

        a.set(&BitSetIndex::new(63).unwrap());
        println!("a: {:?}, b: {:?}", a, b);
        assert_eq!(
            BitSet::intersect(&a, &b).get_set_bits().len() == 0,
            true
        );

        b.set(&BitSetIndex::new(0).unwrap());
        println!("a: {:?}, b: {:?}", a, b);
        assert_eq!(
            BitSet::intersect(&a, &b).get_set_bits().len() == 0,
            true
        );

        a.set(&BitSetIndex::new(0).unwrap());
        println!("a: {:?}, b: {:?}", a, b);
        assert_eq!(
            BitSet::intersect(&a, &b).get_set_bits().len() == 1,
            true
        );

        assert_eq!(
            BitSet::extend(&a, &b).get_set_bits().len() == 2,
            true
        );

        assert_eq!(
            [1, 2, 3, 1].into_iter()
                .map(|index| BitSetIndex::new(index).unwrap())
                .collect::<BitSet>()
                .get_set_bits().len() == 3,
            true
        );

        Ok(())
    }
}