use std::{collections::HashSet, str, u32};

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

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn identify_badge(input: Vec<String>) -> Result<String, String> {
    if input.len() != 3 {
        return Err("input must be vector with three parts".to_string());
    }

    let part1: HashSet<char> = HashSet::from_iter(input[0].chars());
    let part2: HashSet<char> = HashSet::from_iter(input[1].chars());
    let part3: HashSet<char> = HashSet::from_iter(input[2].chars());

    let in_1_and_2: HashSet<String> = HashSet::from_iter(
        part1
            .intersection(&part2)
            .into_iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>()
            .into_iter(),
    );

    let in_1_and_3: HashSet<String> = HashSet::from_iter(
        part1
            .intersection(&part3)
            .into_iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>()
            .into_iter(),
    );

    let in_2_and_3: HashSet<String> = HashSet::from_iter(
        part2
            .intersection(&part3)
            .into_iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>()
            .into_iter(),
    );

    let first_intersection: HashSet<String> = HashSet::from_iter(
        in_1_and_2
            .intersection(&in_1_and_3)
            .into_iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>()
            .into_iter(),
    );

    let mut final_intersection: HashSet<String> = HashSet::from_iter(
        first_intersection
            .intersection(&in_2_and_3)
            .into_iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<String>>()
            .into_iter(),
    );

    if final_intersection.is_empty() {
        return Err("final intersection empty".to_string());
    }

    final_intersection
        .drain()
        .collect::<Vec<String>>()
        .first()
        .ok_or("no values in common".to_string())
        .cloned()
}

pub fn get_summed_badges(input: Vec<String>) -> u32 {
    let mut lines_to_get_badges: Vec<String> = Vec::new();
    let mut badges: Vec<String> = Vec::new();

    for line in input {
        lines_to_get_badges.push(line);
        if lines_to_get_badges.len() == 3 {
            if let Ok(badge) =
                identify_badge(lines_to_get_badges.drain(..).collect::<Vec<String>>())
            {
                badges.push(badge);
            }
        }
    }

    let mut sum: u32 = 0;
    for b in badges {
        sum += get_priority(&b);
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

    #[test]
    fn identify_badge_works() {
        assert_eq!(
            Err("input must be vector with three parts".to_string()),
            identify_badge(vec!["nope".to_string()])
        );

        assert_eq!(
            Err("input must be vector with three parts".to_string()),
            identify_badge(vec!["nope".to_string(), "not enough".to_string(),])
        );

        assert_eq!(
            Err("final intersection empty".to_string()),
            identify_badge(vec!["a".to_string(), "b".to_string(), "c".to_string()])
        );

        assert_eq!(
            Ok("r".to_string()),
            identify_badge(vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
                "PmmdzqPrVvPwwTWBwg".to_string(),
            ])
        );
    }

    #[test]
    fn get_summed_badges_works() {
        assert_eq!(
            70,
            get_summed_badges(vec![
                "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
                "PmmdzqPrVvPwwTWBwg".to_string(),
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
                "ttgJtRGJQctTZtZT".to_string(),
                "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
            ])
        );
    }
}
