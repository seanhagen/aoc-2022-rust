use std::{str, u32};

fn split_line(input: &str) -> Vec<String> {
    let parts = input.split("").collect::<Vec<&str>>();

    let len = input.len();
    let midpoint = len / 2;

    let mut output: Vec<String> = Vec::new();

    let mut tmp: Vec<String> = Vec::new();

    // let mut tmp = String::from("");

    parts.iter().enumerate().for_each(|(idx, chr)| {
        let x = chr.to_string();
        if x == "" {
            return;
        }
        tmp.push(x);
        // tmp = format!("{}{}", tmp, x);

        if idx == midpoint {
            let mut to_push = String::from("");

            tmp.iter()
                .for_each(|s| to_push = format!("{}{}", to_push, s));
            output.push(to_push);
            // tmp.clear();
            // tmp = String::from("");
            tmp = Vec::new();
        }
    });
    let mut to_push = String::from("");
    tmp.iter()
        .for_each(|s| to_push = format!("{}{}", to_push, s));
    // tmp.iter().for_each(|s| to_push.push(s.chars()[0]));
    output.push(to_push);
    // output.push(tmp);
    output
}

fn identify_common(input: Vec<String>) -> Result<Vec<String>, String> {
    if input.len() != 2 {
        return Err("input must be vector with two parts".to_string());
    }
    if input[0].len() != input[1].len() {
        return Err("the two elements in the vector must be the same length".to_string());
    }

    let part1 = input[0]
        .split("")
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let part2 = input[1]
        .split("")
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let mut in_both: Vec<String> = Vec::new();

    part1.iter().enumerate().for_each(|(_, chr)| {
        let a = chr.to_string();
        if a == "" {
            return;
        }

        part2.iter().enumerate().for_each(|(_, chr2)| {
            let b = chr2.to_string();

            if a == b {
                in_both.push(chr.to_string());
            }
        })
    });
    in_both.sort();
    in_both.dedup();
    Ok(in_both)
}

const CHARS: &str = "-abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn get_priority(input: &str) -> u32 {
    if let Some(idx) = CHARS.find(input) {
        idx as u32
    } else {
        0
    }
}

pub fn get_summed_priority(input: Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    for line in input {
        let split = split_line(line.as_str());

        let res = identify_common(split);

        sum += match res {
            Err(_) => 0,
            Ok(r) => {
                let mut tmp_sum = 0;
                for rr in r {
                    let pri = get_priority(&rr);
                    tmp_sum += pri;
                }
                tmp_sum
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_line_properly() {
        /*
        The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which
        means its first compartment contains the items vJrwpWtwJgWr, while the
        second compartment contains the items hcsFMMfFFhFp. The only item type
        that appears in both compartments is lowercase p.
                 */
        assert_eq!(
            vec!["vJrwpWtwJgWr", "hcsFMMfFFhFp"],
            split_line("vJrwpWtwJgWrhcsFMMfFFhFp")
        );

        /*
        The second rucksack's compartments contain jqHRNqRjqzjGDLGL
        and rsFMfFZSrLrFZsSL. The only item type that appears in both
        compartments is uppercase L.
         */
        assert_eq!(
            vec!["jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"],
            split_line("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL")
        );

        /*
        The third rucksack's compartments contain PmmdzqPrV and
        vPwwTWBwg; the only common item type is uppercase P.
         */

        assert_eq!(
            vec!["PmmdzqPrV", "vPwwTWBwg"],
            split_line("PmmdzqPrVvPwwTWBwg")
        );
    }

    #[test]
    fn identify_common_works() {
        assert_eq!(
            Err("input must be vector with two parts".to_string()),
            identify_common(vec![String::from("abc")])
        );

        assert_eq!(
            Err("the two elements in the vector must be the same length".to_string()),
            identify_common(vec![String::from("abc"), String::from("abcde")])
        );
        assert_eq!(
            Ok(vec!["p".to_string()]),
            identify_common(vec![
                String::from("vJrwpWtwJgWr"),
                String::from("hcsFMMfFFhFp")
            ])
        );
        assert_eq!(
            Ok(vec!["L".to_string()]),
            identify_common(vec![
                String::from("jqHRNqRjqzjGDLGL"),
                String::from("rsFMfFZSrLrFZsSL")
            ])
        );
        assert_eq!(
            Ok(vec!["P".to_string()]),
            identify_common(vec![String::from("PmmdzqPrV"), String::from("vPwwTWBwg")])
        );
    }

    #[test]
    fn get_priority_works() {
        assert_eq!(1, get_priority("a"));
        assert_eq!(52, get_priority("Z"));
        assert_eq!(16, get_priority("p"));
        assert_eq!(38, get_priority("L"));
        assert_eq!(42, get_priority("P"));
        assert_eq!(22, get_priority("v"));
        assert_eq!(20, get_priority("t"));
        assert_eq!(19, get_priority("s"));
    }

    #[test]
    fn get_summed_priority_works() {
        assert_eq!(
            157,
            get_summed_priority(vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
                "PmmdzqPrVvPwwTWBwg".to_string(),
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
                "ttgJtRGJQctTZtZT".to_string(),
                "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
            ])
        )
    }
}
