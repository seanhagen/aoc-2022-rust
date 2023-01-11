use std::collections::HashSet;
// use std::{collections::HashSet, str, str::FromStr, u32};

// #[derive(Debug)]
// struct Range {
//     min: u32,
//     max: u32,
// }

pub fn find_packet_start(input: String) -> usize {
    //let buf: [char; 4] = [' ', ' ', ' ', ' '];
    // let data: Vec<char> = input.chars().into_iter().collect();
    // println!("data is: \n{:?}", data);

    let mut start_idx: usize = 0;
    let mut end_idx: usize = 4;
    let max_idx: usize = input.len();

    while end_idx <= max_idx {
        let mut set: HashSet<char> = HashSet::new();
        let _ = &input[start_idx..end_idx].chars().into_iter().for_each(|c| {
            set.insert(c);
        });
        if set.len() == 4 {
            return end_idx;
        }

        start_idx += 1;
        end_idx += 1;
    }

    end_idx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_of_marker_works() {
        assert_eq!(
            7,
            find_packet_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string())
        );
        assert_eq!(
            5,
            find_packet_start("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string())
        );
        assert_eq!(
            6,
            find_packet_start("nppdvjthqldpwncqszvftbrmjlhg".to_string())
        );
        assert_eq!(
            10,
            find_packet_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string())
        );
        assert_eq!(
            11,
            find_packet_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string())
        );
    }
}
