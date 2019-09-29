// Solution for https://adventofcode.com/2015/day/7
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::{env, fs, num::Wrapping};

#[macro_use]
extern crate lazy_static;

type Signal = Wrapping<u16>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct IdVal {
    identifier: Option<String>,
    value: Option<Signal>,
}

impl IdVal {
    fn new(literal: &str) -> IdVal {
        if let Ok(value) = literal.parse::<u16>() {
            IdVal {
                identifier: None,
                value: Some(Wrapping(value)),
            }
        } else {
            IdVal {
                identifier: Some(literal.to_string()),
                value: None,
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Assign(IdVal),
    Not(IdVal),
    And(IdVal, IdVal),
    Or(IdVal, IdVal),
    LShift(IdVal, IdVal),
    RShift(IdVal, IdVal),
}

struct Dependency {
    id: String,
    op: Operation,
}

fn parse(line: &str) -> Option<Dependency> {
    lazy_static! {
        static ref MAIN_RE: Regex = Regex::new(r"((?P<binary>[a-z\d]+ [A-Z]+ [a-z\d]+)|(?P<unary>NOT [a-z]+)|(?P<assignment>[a-z\d]+)) -> (?P<destination>[a-z]+)").unwrap();
        static ref UNARY_RE: Regex = Regex::new(r"NOT (?P<id>[a-z\d]+)").unwrap();
        static ref BINARY_RE: Regex = Regex::new(r"(?P<lhs>[a-z\d]+) (?P<op>[A-Z]+) (?P<rhs>[a-z\d]+)").unwrap();
    }

    let main_capture = MAIN_RE.captures(line)?;
    let destination = main_capture.name("destination")?.as_str().to_string();

    let op = if let Some(assignment) = main_capture.name("assignment") {
        Some(Operation::Assign(IdVal::new(assignment.as_str())))
    } else if let Some(unary) = main_capture.name("unary") {
        UNARY_RE
            .captures(unary.as_str())
            .and_then(|capture| capture.name("id"))
            .map(|id| Operation::Not(IdVal::new(id.as_str())))
    } else if let Some(binary) = main_capture.name("binary") {
        let bin_capture = BINARY_RE.captures(binary.as_str())?;
        let bin_op = bin_capture.name("op")?;

        match bin_op.as_str() {
            "AND" => Some(Operation::And(
                IdVal::new(bin_capture.name("lhs")?.as_str()),
                IdVal::new(bin_capture.name("rhs")?.as_str()),
            )),
            "OR" => Some(Operation::Or(
                IdVal::new(bin_capture.name("lhs")?.as_str()),
                IdVal::new(bin_capture.name("rhs")?.as_str()),
            )),
            "LSHIFT" => Some(Operation::LShift(
                IdVal::new(bin_capture.name("lhs")?.as_str()),
                IdVal::new(bin_capture.name("rhs")?.as_str()),
            )),
            "RSHIFT" => Some(Operation::RShift(
                IdVal::new(bin_capture.name("lhs")?.as_str()),
                IdVal::new(bin_capture.name("rhs")?.as_str()),
            )),
            _ => None,
        }
    } else {
        None
    };

    op.map(|op| Dependency {
        id: destination,
        op,
    })
}

struct Bindings {
    bindings: HashMap<String, Operation>,
    attempted: HashSet<IdVal>,
    cache: HashMap<IdVal, Signal>,
}

impl Bindings {
    fn new() -> Bindings {
        Bindings {
            bindings: HashMap::new(),
            attempted: HashSet::new(),
            cache: HashMap::new(),
        }
    }

    fn get(&mut self, idval: IdVal) -> Signal {
        if let Some(value) = idval.value {
            value
        } else if let Some(identifier) = &idval.identifier {
            if let Some(signal) = self.cache.get(&idval) {
                *signal
            } else {
                self.attempted.insert(idval.clone());
                let value = self.compute(identifier.to_string());
                self.attempted.remove(&idval);
                self.cache.insert(idval, value);
                value
            }
        } else {
            panic!("Invalid idval instance!")
        }
    }

    fn get_two(&mut self, idval1: IdVal, idval2: IdVal) -> (Signal, Signal) {
        if !self.attempted.contains(&idval1) {
            (self.get(idval1), self.get(idval2))
        } else if !self.attempted.contains(&idval2) {
            (self.get(idval2), self.get(idval1))
        } else {
            panic!("Tried calculating both sides of this branch and none have returned!")
        }
    }

    fn compute(&mut self, id: String) -> Signal {
        let op = self.bindings.get(&id).unwrap().clone();
        println!("Computing {:?} for {:?}", op, id);
        match op {
            Operation::Assign(idvalue) => self.get(idvalue.clone()),
            Operation::Not(source) => !self.get(source.clone()),
            Operation::And(left, right) => {
                let (lhs, rhs) = self.get_two(left.clone(), right.clone());
                lhs & rhs
            }
            Operation::Or(left, right) => {
                let (lhs, rhs) = self.get_two(left.clone(), right.clone());
                lhs | rhs
            }
            Operation::LShift(left, right) => {
                let (lhs, rhs) = self.get_two(left.clone(), right.clone());
                lhs << rhs.0 as usize
            }
            Operation::RShift(left, right) => {
                let (lhs, rhs) = self.get_two(left.clone(), right.clone());
                lhs >> rhs.0 as usize
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let wire = &args[1];

    let filename = "inputs/input.txt";

    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut bindings = Bindings::new();

    println!("Parsing lines...");

    for line in content.split('\n') {
        if let Some(dependency) = parse(line) {
            bindings.bindings.insert(dependency.id, dependency.op);
        }
    }

    println!("Computing values...");

    println!(
        "Cable a has a value of {}.",
        bindings.compute(wire.to_string())
    );
}
