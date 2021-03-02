use crate::lib::output::Submission;

pub mod fair_share;

pub trait Solution {
    fn solve(&self) -> Submission;
    fn score(&self) -> usize;
}