use anyhow::Ok;

mod aoc6;

fn main() -> Result<(), anyhow::Error>
{
    let input = std::fs::read_to_string("input.txt")?;
    
    println!("Input:\n{input}");
    let output = aoc6::run(input)?;
    println!("Output:\n{output}");
    std::fs::write("output.txt", output)?;

    Ok(())
}
