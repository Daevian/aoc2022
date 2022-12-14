use std::{ops::ControlFlow, collections::HashMap};

use anyhow::Ok;

pub fn run(input: String) -> Result<String, anyhow::Error>
{
    let window_size = 14; // 4 for Part 1
    let index = get_index_of_first_unique_sequence(&input, window_size)
        .ok_or_else(|| anyhow::Error::msg("sequence didn't have a window of unique characters."))?;

    Ok(index.to_string())
}

fn get_index_of_first_unique_sequence(input: &str, sequence_size: usize) -> Option<usize>
{
    let initial_set = input.bytes().take(sequence_size)
        .fold(
            HashMap::new(),
            |mut set, char|
            {
                set.entry(char)
                    .and_modify(|entry| *entry += 1 )
                    .or_insert(1);
                set
            }
        );

    let initial_result =
        if initial_set.len() == sequence_size {
            Some(sequence_size)
        } else {
            None
        };

    if initial_result.is_some()
    {
        return initial_result;
    }

    let result = input.bytes().zip(input.bytes().skip(sequence_size))
        .try_fold(
            (None::<i32>, initial_set, sequence_size + 1),
            |(_, mut set, index), (back, front)| {
                let back_entry = set.entry(back).and_modify(|entry| *entry -= 1).or_default();
                if back_entry == &0 {
                    set.remove(&back);
                }

                set.entry(front)
                    .and_modify(|entry| *entry += 1 )
                    .or_insert(1);

                if set.len() == sequence_size {
                    ControlFlow::Break((Some(index), set, index + 1))
                } else {
                    ControlFlow::Continue((None, set, index + 1))
                }
            }
        );
    
    match result
    {
        ControlFlow::Break((index, ..)) => index,
        ControlFlow::Continue(_) => None,
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test()
    {
        let window_size = 4;
        assert_eq!(get_index_of_first_unique_sequence("abcdefghjklmnopqrstuvwxyz", window_size), Some(window_size));
        assert_eq!(get_index_of_first_unique_sequence("asdaasdaasdaasdaasdaasdaasda", window_size), None);

        assert_eq!(get_index_of_first_unique_sequence("mjqjpqmgbljsphdztnvjfqwrcgsmlb", window_size), Some(7));
        assert_eq!(get_index_of_first_unique_sequence("bvwbjplbgvbhsrlpgdmjqwftvncz", window_size), Some(5));
        assert_eq!(get_index_of_first_unique_sequence("nppdvjthqldpwncqszvftbrmjlhg", window_size), Some(6));
        assert_eq!(get_index_of_first_unique_sequence("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", window_size), Some(10));
        assert_eq!(get_index_of_first_unique_sequence("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", window_size), Some(11));

        let window_size = 14;
        assert_eq!(get_index_of_first_unique_sequence("abcdefghjklmnopqrstuvwxyz", window_size), Some(window_size));
        assert_eq!(get_index_of_first_unique_sequence("asdaasdaasdaasdaasdaasdaasda", window_size), None);

        assert_eq!(get_index_of_first_unique_sequence("mjqjpqmgbljsphdztnvjfqwrcgsmlb", window_size), Some(19));
        assert_eq!(get_index_of_first_unique_sequence("bvwbjplbgvbhsrlpgdmjqwftvncz", window_size), Some(23));
        assert_eq!(get_index_of_first_unique_sequence("nppdvjthqldpwncqszvftbrmjlhg", window_size), Some(23));
        assert_eq!(get_index_of_first_unique_sequence("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", window_size), Some(29));
        assert_eq!(get_index_of_first_unique_sequence("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", window_size), Some(26));

    }
}