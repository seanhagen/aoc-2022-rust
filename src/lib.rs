use std::{str, u32};

fn split_line(input: &str) -> Vec<String> {
    let len = input.len();
    let midpoint = len / 2;
    let parts = input.split("").collect::<Vec<&str>>();
    let mut output: Vec<String> = Vec::new();
    let mut tmp = String::from("");

    parts.iter().enumerate().for_each(|(idx, chr)| {
        let chr = chr.to_string();
        if chr == "" {
            return;
        }
        tmp = format!("{}{}", tmp, chr);

        if idx == midpoint {
            output.push(tmp.clone());
            tmp = String::from("");
        }
    });
    output.push(tmp);
    output
}

fn identify_common(input: Vec<&str>) -> Result<Vec<&str>, &str> {
    if input.len() != 2 {
        return Err("input must be vector with two parts");
    }
    if input[0].len() != input[1].len() {
        return Err("the two elements in the vector must be the same length");
    }
    let part1 = input[0].split("").collect::<Vec<&str>>();
    let part2 = input[1].split("").collect::<Vec<&str>>();
    let mut in_both: Vec<&str> = Vec::new();

    part1.iter().enumerate().for_each(|(idx, chr)| {
        let a = chr.to_string();
        if a == "" {
            return;
        }

        part2.iter().enumerate().for_each(|(idx2, chr2)| {
            let b = chr2.to_string();

            if a == b {
                in_both.push(chr);
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

fn get_summed_priority(input: Vec<&str>) -> u32 {
    0
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
            Err("input must be vector with two parts"),
            identify_common(vec!["abc"])
        );

        assert_eq!(
            Err("the two elements in the vector must be the same length"),
            identify_common(vec!["abc", "abcde"])
        );
        assert_eq!(
            Ok(vec!["p"]),
            identify_common(vec!["vJrwpWtwJgWr", "hcsFMMfFFhFp"])
        );
        assert_eq!(
            Ok(vec!["L"]),
            identify_common(vec!["jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL"])
        );
        assert_eq!(
            Ok(vec!["P"]),
            identify_common(vec!["PmmdzqPrV", "vPwwTWBwg"])
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
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw",
            ])
        )
    }
}
