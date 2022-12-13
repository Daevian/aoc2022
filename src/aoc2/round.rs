
pub fn calc_round_score(round: &Round) -> i32
{
    round.your.0.get_score() + round.get_score()
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum Choice
{
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug,Clone,Copy)]
pub struct Their(pub Choice);

#[derive(Debug,Clone,Copy)]
pub struct Your(pub Choice);

#[derive(Debug)]
pub struct Round
{
    their: Their,
    your: Your,
}

impl Choice
{
    fn get_score(&self) -> i32
    {
        match *self
        {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn get_winning_choice(&self) -> Choice
    {
        match *self
        {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        }
    }

    fn get_losing_choice(&self) -> Choice
    {
        match *self
        {
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
            Choice::Rock => Choice::Scissors,
        }
    }

    fn get_drawing_choice(&self) -> Choice
    {
        *self
    }

    fn against(&self, choice: Choice) -> RoundResult
    {
        match choice.get_drawing_choice().eq(&self)
        {
            true => RoundResult::Draw,
            false =>
            match choice.get_winning_choice().eq(self)
            {
                true => RoundResult::Win,
                false => RoundResult::Lose,
            }
        }
    }

    pub fn get_choice_for_result(&self, result: RoundResult) -> Choice
    {
        match result
        {
            RoundResult::Draw => self.get_drawing_choice(),
            RoundResult::Win => self.get_winning_choice(),
            RoundResult::Lose => self.get_losing_choice(),
        }
    }
}

impl Round
{
    pub fn new(their: Their, your: Your) -> Round
    {
        Round {their: their, your: your}
    }

    fn get_score(&self) -> i32
    {
        match self.your.0.against(self.their.0)
        {
            RoundResult::Lose => 0,
            RoundResult::Draw => 3,
            RoundResult::Win => 6,
        }
    }
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum RoundResult
{
    Draw,
    Win,
    Lose,
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_choices()
    {    
        assert_eq!(Choice::Rock.against(Choice::Rock), RoundResult::Draw);
        assert_eq!(Choice::Scissors.against(Choice::Scissors), RoundResult::Draw);
        assert_eq!(Choice::Paper.against(Choice::Paper), RoundResult::Draw);
        assert_eq!(Choice::Rock.against(Choice::Paper), RoundResult::Lose);
        assert_eq!(Choice::Paper.against(Choice::Scissors), RoundResult::Lose);
        assert_eq!(Choice::Scissors.against(Choice::Rock), RoundResult::Lose);
        assert_eq!(Choice::Rock.against(Choice::Scissors), RoundResult::Win);
        assert_eq!(Choice::Scissors.against(Choice::Paper), RoundResult::Win);
        assert_eq!(Choice::Paper.against(Choice::Rock), RoundResult::Win);
    }
}