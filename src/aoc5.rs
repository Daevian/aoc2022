mod cargo;

pub fn run(input: String) -> Result<String, anyhow::Error>
{
    let split_input = input.split_once("\r\n\r\n")
        .ok_or_else(|| anyhow::Error::msg("Input doesn't have an empty line to split the cargo layout and operations list."))?;

    let mut cargo = cargo::parse_cargo_layout(split_input.0)?;
    println!("{:?}", cargo);
    let operations = cargo::parse_operations(split_input.1)?;
    println!("{:?}", operations);

    operations.iter().try_for_each(|op| {cargo::CargoLayout::apply_op(&mut cargo, &op) })?;
    println!("{:?}", cargo);

    let top_crates = cargo::CargoLayout::get_top_crates(&cargo);
    let output = top_crates.into_iter().collect::<String>();

    Ok(output)
}