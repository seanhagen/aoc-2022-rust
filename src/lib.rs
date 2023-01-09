use std::str::FromStr;

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
    fn contains(&self, other: &Range) -> bool {
        let myself: Vec<u32> = (self.min..=self.max).collect::<Vec<u32>>();
        let other: Vec<u32> = (other.min..=other.max).collect::<Vec<u32>>();

        match find_subseq(myself.as_slice(), other.as_slice()) {
            Some(_) => true,
            _ => false,
        }
    }
}

pub fn fully_contained_count(input: Vec<String>) -> u32 {
    let mut fully_contained: u32 = 0;

    for line in input {
        let mut range_one: Range = Range { min: 0, max: 0 };
        let mut range_two: Range = Range { min: 0, max: 0 };

        line.split(",").by_ref().enumerate().for_each(|(i, s)| {
            let wat = s
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

            if i == 0 {
                range_one.min = first;
                range_one.max = last;
            } else {
                range_two.min = first;
                range_two.max = last;
            }
        });
        if range_one.contains(&range_two) || range_two.contains(&range_one) {
            fully_contained += 1;
        }
    }

    fully_contained
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
}
