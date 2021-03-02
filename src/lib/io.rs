use std::str::FromStr;
use std::num::ParseIntError;

type IntersectionID = usize;
type CarID = usize;

struct Street {
    starts_in: IntersectionID,
    ends_in: IntersectionID,
    name: String,
    length: usize,
}

#[derive(Debug)]
pub struct ParseStreetError {}
impl FromStr for Street {
    type Err = ParseStreetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        Ok(Street {
            starts_in: parts[0].parse::<usize>().map_err(|_| ParseStreetError {})?,
            ends_in: parts[1].parse::<usize>().map_err(|_| ParseStreetError {})?,
            name: parts[2].to_owned(),
            length: parts[3].parse::<usize>().map_err(|_| ParseStreetError {})?,
        })
    }
}

struct Car {
    id: CarID,
    path: Vec<String>,
}

#[derive(Debug)]
pub struct ParseCarError {}
impl FromStr for Car {
    type Err = ParseCarError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Car {
            id: 0,
            path: s
                .split_whitespace()
                .skip(1)
                .map(|s| s.to_string())
                .collect(),
        })
    }
}

struct Problem {
    simulation_time: usize,
    intersection_count: usize,
    streets: Vec<Street>,
    cars: Vec<Car>,
    reaching_destination_bonus: usize,
}

#[derive(Debug)]
pub enum ParseProblemError {
    NoHeaderLine,
    Parse(ParseIntError),
}

impl From<ParseIntError> for ParseProblemError {
    fn from(err: ParseIntError) -> ParseProblemError {
        ParseProblemError::Parse(err)
    }
}

impl FromStr for Problem {
    type Err = ParseProblemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().into_iter();
        let header = lines.next().ok_or(ParseProblemError::NoHeaderLine)?;
        let header_parts = header.split_whitespace().collect::<Vec<_>>();

        let simulation_time = header_parts[0].parse::<usize>()?;
        let intersection_count = header_parts[1].parse::<usize>()?;
        let streets_count = header_parts[2].parse::<usize>()?;
        let cars_count = header_parts[3].parse::<usize>()?;
        let reaching_destination_bonus = header_parts[4].parse::<usize>()?;

        let streets = lines.by_ref()
            .take(streets_count)
            .filter_map(|s| Street::from_str(s).ok())
            .collect();
        let cars = lines.by_ref()
            .take(cars_count)
            .enumerate()
            .filter_map(|(id, s)| {
                let car = Car::from_str(s).ok();
                car.and_then(|mut car| {
                    car.id = id;
                    Some(car)
                })
            })
            .collect();

        Ok(Problem {
            simulation_time,
            intersection_count,
            streets,
            cars,
            reaching_destination_bonus,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::lib::io::Problem;

    #[test]
    fn example_problem_parsing() {
        let problem = Problem::from_str(include_str!("../../inputs/a.txt")).unwrap();
        assert_eq!(problem.simulation_time, 6);
        assert_eq!(problem.intersection_count, 4);
        assert_eq!(problem.streets.len(), 5);
        assert_eq!(problem.cars.len(), 2);
        assert_eq!(problem.reaching_destination_bonus, 1000);
    }
}
