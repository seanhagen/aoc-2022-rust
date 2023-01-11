use std::collections::HashSet;

fn find_start(input: String, min_len: usize) -> usize {
    let mut start_idx: usize = 0;
    let mut end_idx: usize = min_len;
    let max_idx: usize = input.len();

    while end_idx <= max_idx {
        let mut set: HashSet<char> = HashSet::new();
        let _ = &input[start_idx..end_idx].chars().into_iter().for_each(|c| {
            set.insert(c);
        });
        if set.len() == min_len {
            return end_idx;
        }

        start_idx += 1;
        end_idx += 1;
    }

    end_idx
}

pub fn find_packet_start(input: String) -> usize {
    find_start(input, 4)
}

pub fn find_message_start(input: String) -> usize {
    find_start(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_of_message_works() {
        assert_eq!(
            19,
            find_message_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string())
        );

        assert_eq!(
            23,
            find_message_start("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string())
        );

        assert_eq!(
            23,
            find_message_start("nppdvjthqldpwncqszvftbrmjlhg".to_string())
        );

        assert_eq!(
            29,
            find_message_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string())
        );

        assert_eq!(
            26,
            find_message_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string())
        );
    }

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
