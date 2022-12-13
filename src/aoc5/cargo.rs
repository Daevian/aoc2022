
#[derive(Debug)]
pub struct CargoLayout
{
    stacks: Vec<Vec<char>>,
}

#[derive(Debug)]
pub struct Operation
{
    amount: usize,
    from: usize,
    to: usize,
}

impl CargoLayout
{
    fn from_crate_stacks(stacks: Vec<Vec<char>>) -> CargoLayout
    {
        CargoLayout{stacks: stacks}
    }

    pub fn get_top_crates(cargo: &CargoLayout) -> Vec<char>
    {
        cargo.stacks.iter()
            .filter_map(|stack| stack.last().copied())
            .collect::<Vec<_>>()
    }

    pub fn apply_op(cargo: &mut CargoLayout, op: &Operation) -> Result<(), anyhow::Error>
    {
        let moved_crates = cargo.stacks.get_mut(op.from - 1)
            .ok_or_else(|| anyhow::Error::msg(format!("from stack {} doesn't exist!", op.from)))
            .and_then(|from|
                match from.len() - op.amount
                {
                    0.. => Ok(from.drain((from.len() - op.amount)..).collect::<Vec<_>>()),
                    _ => Err(anyhow::Error::msg(format!("Can't move {} crates from stack {:?}", op.amount, from))),
                }
            )?;
        
        cargo.stacks.get_mut(op.to - 1)
            .ok_or_else(|| anyhow::Error::msg(format!("to stack {} doesn't exist!", op.to)))
            .and_then(|to|
                Ok(moved_crates.into_iter()
                    //.rev() // use for Day 1
                    .for_each(|char| to.push(char))
                )
            )
    }
}

impl Operation {
    fn new(amount: usize, from: usize, to: usize) -> Operation
    {
        Operation { amount: amount, from: from, to: to }
    }
}

pub fn parse_cargo_layout(layout_str: &str) -> Result<CargoLayout, anyhow::Error>
{
    let crate_rows = layout_str.lines().rev()
        .skip(1)
        .map(
            |line|
            line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .filter(|(_, char)| char != &' ')
            .map(
                |(idx, char)|
                match char.is_ascii_uppercase() {
                    true => Ok((idx, char)),
                    false => Err(anyhow::Error::msg(format!("crate char '{}' is not ascii uppercase!", char))),
                }
            )
            .collect::<Result<Vec<_>, _>>()
        ).collect::<Result<Vec<_>, _>>()?;
    
    let num_of_stacks = crate_rows.iter().map(|row| row.len()).max()
        .ok_or_else(|| anyhow::Error::msg(format!("No stacks!")))?;
    let number_of_rows = crate_rows.len();
    
    let crate_stacks = crate_rows.into_iter().flatten()
        .try_fold(
            vec![Vec::with_capacity(number_of_rows); num_of_stacks],
            |mut stacks, (idx, char)|
            {
                stacks.get_mut(idx)
                    .ok_or_else(|| anyhow::Error::msg(format!("No stack for column index!")))?
                    .push(char);
                    Ok::<_, anyhow::Error>(stacks)
            }
        )?;

    Ok(CargoLayout::from_crate_stacks(crate_stacks))
}

pub fn parse_operations(operations_str: &str) -> Result<Vec<Operation>, anyhow::Error>
{
    enum Arg
    {
        Amount = 0,
        From = 1,
        To = 2,
    }

    let op_args = operations_str.lines()
        .map(|line|
            line.split_whitespace()
            .skip(1)
            .step_by(2)
            .map(|arg| arg.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()
        ).collect::<Result<Vec<_>, _>>()?;
    
    let ops = op_args.into_iter()
        .map(|args|
            args.get(Arg::Amount as usize).ok_or_else(|| anyhow::Error::msg(format!("missing move Amount arg"))).and_then(|amount|
                args.get(Arg::From as usize).ok_or_else(|| anyhow::Error::msg(format!("missing From arg"))).and_then(|from|
                    args.get(Arg::To as usize).ok_or_else(|| anyhow::Error::msg(format!("missing To arg"))).and_then(|to|
                        Ok(Operation::new(*amount, *from, *to))
                    )
                )
            )
        ).collect::<Result<Vec<_>, _>>()?;
        
    Ok(ops)
}