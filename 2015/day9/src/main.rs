// Solution for https://adventofcode.com/2015/day/9
use std::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    fmt::{Debug, Display, Error, Formatter},
    fs,
};

struct Distance<'a> {
    from: &'a str,
    to: &'a str,
    distance: usize,
}

fn parse<'a>(line: &'a str, cities: &mut Vec<&'a str>) -> Distance<'a> {
    let parts: Vec<&str> = line.split_whitespace().collect();

    if !cities.contains(&parts[0]) {
        cities.push(&parts[0]);
    }

    if !cities.contains(&parts[2]) {
        cities.push(&parts[2]);
    }

    Distance {
        from: parts[0],
        to: parts[2],
        distance: parts[4].parse::<usize>().unwrap(),
    }
}

#[derive(Clone)]
struct Path<'a> {
    cities: Vec<&'a str>,
    length: usize,
}

impl<'a> Display for Path<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.cities[0])?;
        for city in &self.cities[1..] {
            write!(f, "-> {}", city)?;
        }

        Ok(())
    }
}

impl<'a> PartialEq for Path<'a> {
    fn eq(&self, rhs: &Path) -> bool {
        self.length == rhs.length
    }
}

impl<'a> Eq for Path<'a> {}

impl<'a> PartialOrd for Path<'a> {
    fn partial_cmp(&self, rhs: &Path) -> Option<Ordering> {
        self.length.partial_cmp(&rhs.length)
    }
}

impl<'a> Ord for Path<'a> {
    fn cmp(&self, rhs: &Path) -> Ordering {
        self.length.cmp(&rhs.length)
    }
}

fn permutations<T: Clone + Debug>(k: usize, items: &mut Vec<T>) -> Vec<Vec<T>> {
    let mut perms: Vec<Vec<T>> = vec![];
    if k == 1 {
        perms.push(items.to_vec());
    } else {
        perms.append(&mut permutations(k - 1, items));

        for i in 0..k - 1 {
            if k % 2 == 0 {
                items.swap(i, k - 1);
            } else {
                items.swap(0, k - 1);
            }

            perms.append(&mut permutations(k - 1, items));
        }
    }
    perms
}

fn calculate_shortest<'a>(cities: &'a mut Vec<&'a str>, distances: &Vec<Distance>) -> Path<'a> {
    let mut paths: Vec<Path> = vec![];

    for path in permutations(cities.len(), cities) {
        let mut p = Path {
            cities: path,
            length: 0,
        };
        compute_length(&mut p, distances);
        paths.push(p);
    }

    paths.iter().max().unwrap().clone()
}

fn compute_length(path: &mut Path, distances: &Vec<Distance>) {
    for (i, city) in path.cities[..path.cities.len() - 1].iter().enumerate() {
        let from = city.to_string();
        let to = path.cities[i + 1].to_string();
        for distance in distances {
            if (distance.from == from && distance.to == to)
                || (distance.to == from && distance.from == to)
            {
                path.length += distance.distance;
                break;
            }
        }
    }
}

fn main() {
    let filename = "inputs/input.txt";

    let content = fs::read_to_string(filename).unwrap();

    let mut distances: Vec<Distance> = vec![];
    let mut cities: Vec<&str> = vec![];

    for line in content.lines() {
        distances.push(parse(line, &mut cities));
    }

    let path = calculate_shortest(&mut cities, &distances);

    println!(
        "Shortest path has length {} and goes through: {}",
        path.length, path
    );
}
