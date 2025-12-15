use framework::Solution;
use std::convert::Infallible;
use std::str::FromStr;

pub struct Day08;

pub struct Vec3U {
    x: u64,
    y: u64,
    z: u64
}

pub struct VecList(Vec<Vec3U>);

impl FromStr for VecList {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vecs: Vec<Vec3U> = vec![];

        for line in s.lines() {
            let components = line.split(",").collect::<Vec<&str>>();
            let x = components[0].parse::<u64>().unwrap_or(0);
            let y = components[1].parse::<u64>().unwrap_or(0);
            let z = components[2].parse::<u64>().unwrap_or(0);
            vecs.push(Vec3U {x, y, z})
        }

        Ok(VecList(vecs))
    }
}

impl Vec3U {
    pub fn dist_squared(&self, other: &Vec3U) -> u64 {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)
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
        todo!()
    }

    fn part_two(input: Self::ParsedInput) -> Option<Self::ResultType> {
        todo!()
    }
}