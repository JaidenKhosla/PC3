use crate::database::database::Database;
use crate::teams::Team;
use crate::Res;

use crate::collections::BinaryHeap::BinaryHeap;
pub struct Scoreboard<'a>
{
    teams: &'a Vec<Team>,
    ordering: BinaryHeap<'a, usize>
}

impl<'a> Scoreboard<'a>
{
    #[allow(unused)]
    pub async fn new(teams: &'a Vec<Team>, ranking: fn(&Team, &Team) -> i8)
    {
        let usize_ranking = |a: &usize, b: &usize| -> i8
        {
            return (ranking)(&teams[*a], &teams[*b]);
        };

        let ordering: BinaryHeap<usize> = BinaryHeap::new(Box::new(&usize_ranking));
    }
    
    #[allow(unused)]
    pub fn seed(&mut self) -> Res![_ ()]
    {
        Ok(())
    }
}

/*
DEFAULT SCORING:

Score, Highest Weight of Solved Problem, Fastest Time for Highest Weight Problem
*/