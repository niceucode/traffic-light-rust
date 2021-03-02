use crate::lib::input::{IntersectionID, Problem, Street};
use crate::lib::output::{IntersectionSchedule, StreetSchedule, Submission};
use crate::lib::solutions::Solution;
use std::collections::HashMap;

pub struct Solver {
    pub problem: Problem,
}

struct Intersection {
    in_streets: Vec<Street>,
}

impl Solution for Solver {
    fn solve(&self) -> Submission {
        let intersections = self.problem.streets.iter().fold(
            HashMap::<IntersectionID, Intersection>::new(),
            |mut acc, s| {
                let intersection = acc
                    .entry(s.ends_in)
                    .or_insert(Intersection { in_streets: vec![] });
                intersection.in_streets.push(s.clone());
                acc
            },
        );

        Submission {
            intersections: intersections
                .iter()
                .map(|(&id, intersection)| IntersectionSchedule {
                    id,
                    schedules: intersection
                        .in_streets
                        .iter()
                        .map(|s| StreetSchedule {
                            name: s.name.clone(),
                            green_time: 1,
                        })
                        .collect(),
                })
                .collect(),
        }
    }

    fn score(&self) -> usize {
        unimplemented!()
    }
}
