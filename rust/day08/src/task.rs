use framework::Solution;
use std::convert::Infallible;
use std::str::FromStr;

pub struct Day08;

/// A 3D Vector with u32 components
#[derive(Debug, Copy, Clone, Eq, PartialOrd, PartialEq, Ord)]
pub struct Vec3U {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

pub struct VecList(pub Vec<Vec3U>);

impl FromStr for VecList {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vecs: Vec<Vec3U> = vec![];

        for line in s.lines() {
            let components = line.split(",").collect::<Vec<&str>>();
            let x = components[0].parse::<i32>().unwrap_or(0);
            let y = components[1].parse::<i32>().unwrap_or(0);
            let z = components[2].parse::<i32>().unwrap_or(0);
            vecs.push(Vec3U { x, y, z })
        }

        Ok(VecList(vecs))
    }
}

impl Vec3U {
    pub fn dist_squared(&self, other: &Vec3U) -> u32 {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as u32
    }

    pub fn dist(&self, other: &Vec3U) -> f32 {
        (self.dist_squared(other) as f32).sqrt()
    }
}

/// Number of shortest connections to find (for part 1)
const ITERATIONS: u64 = 1000;

impl Solution for Day08 {
    type ParsedInput = VecList;
    type ResultType = u64;

    fn part_one(input: Self::ParsedInput) -> Option<Self::ResultType> {
        let circuit_sizes = create_circuits(&input, ITERATIONS);
        let product = circuit_sizes.iter().fold(1, |a, b| a * b) as u64;
        Some(product)
    }

    fn part_two(input: Self::ParsedInput) -> Option<Self::ResultType> {
        todo!()
    }
}

pub(crate) fn create_circuits(positions: &VecList, connections: u64) -> Vec<u32> {
    let mut pairs: Vec<(Vec3U, Vec3U)> = vec![];
    let mut distances: Vec<(Vec3U, Vec3U, f32)> = vec![];
    let mut circuits: Vec<Vec<Vec3U>> = vec![];
    let mut circuit_sizes: Vec<u32> = vec![];

    for v in &positions.0 {
        for w in &positions.0 {
            if v != w && !pairs.contains(&(*w, *v)) {
                pairs.push((*v, *w));
            }
        }
    }

    distances = pairs.iter().map(|(a, b)| (*a, *b, a.dist(b))).collect();

    distances.sort_by(|(_, _, d1), (_, _, d2)| d1.partial_cmp(d2).unwrap());

    let shortest = &distances[0..connections as usize];

    // add first position to save the is_empty check in the loop
    circuits.push(vec![positions.0[0]]);

    // for all following positions
    for connection in shortest {
        circuits
            .iter_mut()
            .for_each(|circuit| {
                let has_a = circuit.contains(&connection.0);
                let has_b = circuit.contains(&connection.1);
                if has_a && !has_b {
                    circuit.push(connection.0)
                } else if has_b && !has_a {
                    circuit.push(connection.0)
                }
            });
    }

    circuit_sizes = circuits.iter().map(|conn| conn.len() as u32).collect();
    circuit_sizes.sort_by(|l1, l2| l1.partial_cmp(l2).unwrap());
    circuit_sizes
}
