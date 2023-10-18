use rust_sc2::prelude::*;
//use crate::early_experiments::ex_1::*; // Ex1
use crate::early_experiments::feedback_loop::*; // FeedbackLoop1

pub mod early_experiments;

fn main() -> SC2Result<()> {
    let mut bot = FeedbackLoop::default();
    run_vs_computer(
        &mut bot,
        Computer::new(Race::Random, Difficulty::Medium, None),
        "PillarsofGoldLE",
        Default::default(),
    )
}