use framework::{do_while, Solution};
use std::convert::Infallible;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

pub struct Day05;

#[derive(Copy, Clone)]
pub struct Range(pub u64, pub u64);

pub struct ProductIdRanges {
    ranges: Vec<Range>,
    ids: Vec<u64>
}

impl FromStr for ProductIdRanges {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ranges: Vec<Range> = Vec::new();
        let mut ids: Vec<u64> = Vec::new();

        let mut l = 0;

        for line in s.lines() {
            l += 1;

            if line.contains("-") {
                let (a, b) = line.split_once("-").unwrap();
                let from = a.parse::<u64>()
                    .expect(&format!("Failed to parse {} in line {} to u64", a, l));
                let to = b.parse::<u64>()
                    .expect(&format!("Failed to parse {} in line {} to u64", b, l));
                let range = Range(from, to);
                //println!("Parsed line {l:03} to range {range:?}");
                ranges.push(range);
            } else if !line.is_empty() {
                let id = line.parse::<u64>()
                    .expect(&format!("Failed to parse {} in line {} to u64", line, l));
                ids.push(id);
            }
        }

        println!("Parsed {} ranges and {} ids", ranges.len(), ids.len());

        Ok(ProductIdRanges { ranges, ids })
    }
}

impl Debug for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} to {})", self.0, self.1)?;
        Ok(())
    }
}

impl Solution for Day05 {
    type ParsedInput = ProductIdRanges;
    type ResultType = u64;

    fn part_one(input: Self::ParsedInput) -> Option<Self::ResultType> {
        let mut fresh_products = 0;

        for id in input.ids {
            let mut is_fresh;
            for range in &input.ranges {
                is_fresh = in_range(id, range.0, range.1);
                if is_fresh {
                    fresh_products += 1;
                    break
                }
            }
        }

        Some(fresh_products)
    }

    fn part_two(input: Self::ParsedInput) -> Option<Self::ResultType> {
        let mut fresh_ids = 0;
        let mut merged_ranges: Vec<Range> = Vec::new();

        for range in input.ranges {
            (merged_ranges, _) = merge_with(range, merged_ranges);
        }

        println!("{} ranges after merge:", merged_ranges.len());
        for range in &merged_ranges {
            println!("{range:?}")
        }

        let mut total_merges;
        do_while!{{
            total_merges = 0;
            for range in merged_ranges.clone() {
                let merges;
                (merged_ranges, merges) = merge_with(range, merged_ranges);
                total_merges += merges;
            }
            println!("{} ranges after merge:", merged_ranges.len());
            for range in &merged_ranges {
                println!("{range:?}")
            }
        } while total_merges > 0}

        println!("Merged ranges:");
        for range in &merged_ranges {
            let len = range.1 - range.0 + 1;
            println!("{len:13} ids in range {range:?}");
            fresh_ids += len;
        }

        Some(fresh_ids)
    }
}

pub(crate) fn in_range(num: u64, from: u64, to: u64) -> bool {
    num >= from && num <= to
}

pub(crate) fn merge_with(range: Range, ranges: Vec<Range>) -> (Vec<Range>, u64) {
    let mut result: Vec<Range> = Vec::new();
    let mut range_merged = false;
    let mut merges = 0;

    for r in ranges {
        //println!("Merging range {range:?} with {r:?}");

        // ranges sind identisch
        if range.0 == r.0 && range.1 == r.1 {
            if range_merged {
                continue
            } else {
                println!("identical ranges");
                result.push(r);
                range_merged = true;
            }
        }
        // range ist komplett in r
        else if range.0 >= r.0 && range.1 <= r.1 {
            //println!("{range:?} fully inside {r:?}");
            result.push(r);
            range_merged = true;
            merges += 1;
        }
        // r ist komplett in range
        else if r.0 >= range.0 && r.1 <= range.1 {
            //println!("{r:?} fully inside {range:?}");
            result.push(range);
            range_merged = true;
            merges += 1;
        }
        // range.0 ist innerhalb von r und range.1 liegt über r
        else if range.0 >= r.0 && range.0 <= r.1 && range.1 > r.1 {
            println!("{range:?} + {r:?} --> ({} to {})", r.0, range.1);
            result.push(Range(r.0, range.1));
            range_merged = true;
            merges += 1;
        }
        // range.0 liegt unter r und range.1 ist innerhalb von r
        else if range.0 < r.0 && range.1 >= r.0 && range.1 <= r.1 {
            println!("{range:?} + {r:?} --> ({} to {})", range.0, r.1);
            result.push(Range(range.0, r.1));
            range_merged = true;
            merges += 1;
        }
        // else: range und r haben keine Schnittmenge, also nur r adden
        else {
            //println!("no intersection, adding {r:?}");
            result.push(r);
        }
    }

    // wenn range mit keiner anderen range gemerged wurde, füge range extra hinzu
    if !range_merged {
        //println!("{range:?} is completely unique, adding it to the list of ranges");
        result.push(range);
    }

    println!("Number of ranges in result: {}", result.len());

    (result, merges)
}
