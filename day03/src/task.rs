use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

pub enum Part {
    One,
    Two
}

trait Print {
    fn print(&self);
}

impl Print for Vec<u8> {
    fn print(&self) {
        print!("[");
        if self.len() > 0 {
            print!("{}", self[0]);
            for i in 1..self.len() {
                print!(" {}", self[i]);
            }
        }
        println!("]");
    }
}

pub fn run(file_path: &str, part: Part) -> io::Result<u64> {
    let reader = BufReader::new(File::open(file_path)?);

    let mut sum: u64 = 0;

    for mut line in reader.lines().map(|line| line.unwrap().chars().collect::<Vec<char>>()) {
        let digits: Vec<u8> = line.iter_mut()
            .map(|ch| ch.to_digit(10).unwrap() as u8).collect();

        match part {
            Part::One => {
                let (d1, d2) = biggest_2_digits_in_order(digits);
                println!("biggest digits in order: {}, {}", d1, d2);
                sum += (10 * d1 + d2) as u64;
            },
            Part::Two => {
                let biggest_digits = biggest_n_digits_in_order(digits, 12);
                biggest_digits.print();
                sum += to_number(biggest_digits);
            },
        }
    }
    println!("{sum}");
    Ok(sum)
}

pub(crate) fn biggest_2_digits_in_order(digits: Vec<u8>) -> (u8, u8) {
    let mut largest_idx = 0;
    let mut d1 = 0;
    for i in 0..digits.len()-1 {
        if digits[i] > d1 {
            d1 = digits[i];
            largest_idx = i;
        }
    }

    let mut d2 = 0;
    for i in largest_idx+1..digits.len() {
        if digits[i] > d2 {
            d2 = digits[i];
        }
    }

    (d1, d2)
}

pub(crate) fn biggest_n_digits_in_order(digits: Vec<u8>, n: u8) -> Vec<u8> {
    let mut res = vec![];
    let mut largest_idx = 0;

    for num in 1..=n {
        let mut curr_d = 0;
        for i in largest_idx..digits.len() - (n-num) as usize {
            if digits[i] > curr_d {
                curr_d = digits[i];
                largest_idx = i+1;
            }
        }
        res.push(curr_d);
    }

    res
}

pub(crate) fn to_number(digits: Vec<u8>) -> u64 {
    let len = digits.len();
    let mut num: u64 = 0;

    for i in 0..len {
        num += digits[i] as u64 * 10u64.pow((len - (i + 1)) as u32);
    }
    num
}