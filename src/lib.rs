use std::{collections::HashSet, str, str::FromStr, u32};

#[derive(Debug)]
struct Range {
    min: u32,
    max: u32,
}

fn find_subseq(haystack: &[u32], needle: &[u32]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

impl Range {
    fn get_ranges(input: String) -> (Range, Range) {
        let mut range_one: Range = Range { min: 0, max: 0 };
        let mut range_two: Range = Range { min: 0, max: 0 };

        input.split(",").by_ref().enumerate().for_each(|(i, s)| {
            if i == 0 {
                range_one = Range::from_string(s.to_string());
            } else {
                range_two = Range::from_string(s.to_string());
            }
        });

        (range_one, range_two)
    }

    fn from_string(input: String) -> Range {
        let wat = input
            .split("-")
            .map(|x| {
                if let Ok(r) = u32::from_str(x) {
                    return r;
                }
                0
            })
            .collect::<Vec<u32>>();

        let first: u32 = match wat.first() {
            Some(r) => *r,
            _ => 0,
        };
        let last: u32 = match wat.last() {
            Some(r) => *r,
            _ => 0,
        };

        Range {
            min: first,
            max: last,
        }
    }

    fn contains(&self, other: &Range) -> bool {
        let myself: Vec<u32> = (self.min..=self.max).collect::<Vec<u32>>();
        let other: Vec<u32> = (other.min..=other.max).collect::<Vec<u32>>();

        match find_subseq(myself.as_slice(), other.as_slice()) {
            Some(_) => true,
            _ => false,
        }
    }

    fn overlaps(&self, other: &Range) -> bool {
        let one: HashSet<u32> =
            HashSet::from_iter((self.min..=self.max).collect::<Vec<u32>>().into_iter());

        let two: HashSet<u32> =
            HashSet::from_iter((other.min..=other.max).collect::<Vec<u32>>().into_iter());

        one.intersection(&two).count().clone() > 0
    }
}

pub fn fully_contained_count(input: Vec<String>) -> u32 {
    let mut fully_contained: u32 = 0;

    for line in input {
        let (range_one, range_two) = Range::get_ranges(line);
        if range_one.contains(&range_two) || range_two.contains(&range_one) {
            fully_contained += 1;
        }
    }

    fully_contained
}

pub fn any_overlap_count(input: Vec<String>) -> u32 {
    let mut any_overlap: u32 = 0;

    for line in input {
        let (range_one, range_two) = Range::get_ranges(line);
        if range_one.overlaps(&range_two) || range_two.overlaps(&range_one) {
            any_overlap += 1;
        }
    }

    any_overlap
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fully_contained_count_works() {
        let input = vec![
            "2-4,6-8".to_string(),
            "2-3,4-5".to_string(),
            "5-7,7-9".to_string(),
            "2-8,3-7".to_string(),
            "6-6,4-6".to_string(),
            "2-6,4-8".to_string(),
        ];
        assert_eq!(2, fully_contained_count(input));
    }

    #[test]
    fn any_overlap_works() {
        let input = vec![
            "2-4,6-8".to_string(),
            "2-3,4-5".to_string(),
            "5-7,7-9".to_string(),
            "2-8,3-7".to_string(),
            "6-6,4-6".to_string(),
            "2-6,4-8".to_string(),
        ];

        assert_eq!(4, any_overlap_count(input));
    }
}
