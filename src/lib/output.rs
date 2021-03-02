use std::fmt;
use crate::lib::input::IntersectionID;

pub struct StreetSchedule {
    pub(crate) name: String,
    pub(crate) green_time: usize,
}

pub struct IntersectionSchedule {
    pub(crate) id: IntersectionID,
    pub(crate) schedules: Vec<StreetSchedule>,
}

pub struct Submission {
    pub(crate) intersections: Vec<IntersectionSchedule>,
}

impl fmt::Display for Submission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.intersections.len())?;
        for intersection in &self.intersections {
            writeln!(f, "{}", intersection.id)?;
            writeln!(f, "{}", intersection.schedules.len())?;
            for street in &intersection.schedules {
                writeln!(f, "{} {}", street.name, street.green_time)?;
            }
        }
        Ok({})
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::output::{IntersectionSchedule, StreetSchedule, Submission};

    #[test]
    fn example_problem_submission() {
        let submission = Submission{
            intersections: vec![
                IntersectionSchedule{id: 1, schedules: vec![
                    StreetSchedule{name: "rue-d-athenes".to_string(), green_time: 2},
                    StreetSchedule{name: "rue-d-amsterdam".to_string(), green_time: 1},
                ]},
                IntersectionSchedule{id: 0, schedules: vec![StreetSchedule{name: "rue-de-londres".to_string(), green_time: 2}]},
                IntersectionSchedule{id: 2, schedules: vec![StreetSchedule{name: "rue-de-moscou".to_string(), green_time: 1}]},
            ]
        };

        assert_eq!(submission.to_string(), "3
1
2
rue-d-athenes 2
rue-d-amsterdam 1
0
1
rue-de-londres 2
2
1
rue-de-moscou 1
".to_string());
    }
}
