use crate::problem::JudgeProblem;
use crate::profile::Profile;

use std::collections::LinkedList;
pub struct Judge<'a>
{
    username: String,
    password: String,

    queue: LinkedList<JudgeProblem<'a>>
}

impl Judge<'_>
{
    pub fn 
}