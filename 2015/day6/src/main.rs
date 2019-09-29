// Solution for https://adventofcode.com/2015/day/6
#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::fs;

type Coord = [usize; 2];

struct Grid {
    lights: Vec<usize>,
}

#[derive(Debug)]
enum Operation {
    TurnOn,
    TurnOff,
    Toggle,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            lights: vec![0; 1_000_000],
        }
    }

    fn count(&mut self) -> usize {
        let mut counter = 0;

        for x in 0..1000 {
            for y in 0..1000 {
                counter += self.lights[x + y * 1000];
            }
        }

        counter
    }

    fn apply(&mut self, op: Operation, topleft: Coord, bottomright: Coord) {
        for x in topleft[0]..=bottomright[0] {
            for y in topleft[1]..=bottomright[1] {
                self.applyat(x, y, &op);
            }
        }
    }

    fn applyat(&mut self, x: usize, y: usize, op: &Operation) {
        match op {
            Operation::TurnOff => {
                if self.lights[x + y * 1000] > 0 {
                    self.lights[x + y * 1000] -= 1
                }
            }
            Operation::TurnOn => self.lights[x + y * 1000] += 1,
            Operation::Toggle => self.lights[x + y * 1000] += 2,
        }
    }
}

type ParseResult = (Operation, Coord, Coord);

fn parse(line: &str) -> Option<ParseResult> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"([a-z\s]+) (\d{1, 3}),(\d{1, 3}) [a-z]+ (\d{1, 3}),(\d{1, 3})").unwrap();
    }

    match RE.captures(line) {
        Some(m) => Some((
            match &m[1] {
                "turn off" => Operation::TurnOff,
                "turn on" => Operation::TurnOn,
                "toggle" => Operation::Toggle,
                _ => panic!("invalid operation {}", &m[1]),
            },
            [
                m[2].parse::<usize>().unwrap(),
                m[3].parse::<usize>().unwrap(),
            ],
            [
                m[4].parse::<usize>().unwrap(),
                m[5].parse::<usize>().unwrap(),
            ],
        )),
        None => None,
    }
}

fn main() {
    let filename = "inputs/2015-6.txt";

    let mut grid = Grid::new();
    println!("Created grid");

    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    for line in content.split('\n') {
        match parse(line) {
            Some((op, topleft, bottomright)) => grid.apply(op, topleft, bottomright),
            None => continue,
        }
    }

    println!("Total of {} lights turned on", grid.count());
}
