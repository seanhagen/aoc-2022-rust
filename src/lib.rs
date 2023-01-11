#[derive(Debug, Default)]
pub struct Docks {
    stacks: Vec<Vec<char>>,
    instructions: Vec<String>,
}

impl Docks {
    fn parse_initial(input: &mut Vec<String>) -> Result<Docks, String> {
        let mut stacks = vec![vec![]; input.len() - 1];
        let _last = input.pop();

        for line in input {
            let wat = line.chars().skip(1).step_by(4).enumerate();
            wat.for_each(|(i, x)| {
                if x != ' ' {
                    stacks[i].push(x);
                }
            });
        }

        Ok(Docks {
            stacks: stacks,
            instructions: Vec::<String>::new(),
        })
    }

    pub fn parse(input: Vec<String>) -> Result<Docks, String> {
        let mut initial: Vec<String> = Vec::<String>::new();
        let mut instructions: Vec<String> = Vec::<String>::new();
        let mut finished_init = false;
        for line in input {
            if line.is_empty() {
                finished_init = true;
                continue;
            }

            match finished_init {
                false => initial.push(line),
                true => instructions.push(line),
            }
        }

        let mut st: Docks;

        match Docks::parse_initial(&mut initial) {
            Ok(s) => st = s,
            Err(e) => return Err(e),
        }
        st.instructions = instructions;
        Ok(st)
    }

    pub fn columns(&self) -> u32 {
        self.stacks.len().try_into().unwrap()
    }

    pub fn get_column(&self, col: u32) -> Result<Vec<char>, String> {
        if col == 0 {
            return Err(format!(
                "invalid column, must be between 0 and {}, got {}",
                self.stacks.len(),
                col
            ));
        }

        let col = col - 1;

        if col >= self.stacks.len().try_into().unwrap() {
            return Err(format!(
                "invalid column, must be between 0 and {}, got {}",
                self.stacks.len(),
                col
            ));
        }

        let idx = usize::try_from(col).unwrap();

        Ok(self.stacks.get(idx).unwrap().to_vec())
    }

    pub fn step(&mut self) -> bool {
        let binding = self.instructions.remove(0);
        let instruction: Vec<&str> = binding.split_whitespace().collect();

        let num_to_move = u32::from_str_radix(instruction.get(1).unwrap(), 10).unwrap();
        let source_stack_idx: usize =
            usize::from_str_radix(instruction.get(3).unwrap(), 10).unwrap() - 1;
        let dest_stack_idx: usize =
            usize::from_str_radix(instruction.get(5).unwrap(), 10).unwrap() - 1;

        let source_stack: &mut Vec<char> = self.stacks.get_mut(source_stack_idx).unwrap();

        let tmp: &mut Vec<char> = &mut Vec::<char>::new();
        for _i in 1..=num_to_move {
            let item = source_stack.remove(0);
            tmp.push(item);
        }

        let dest_stack: &mut Vec<char> = self.stacks.get_mut(dest_stack_idx).unwrap();

        for _i in 1..=num_to_move {
            let item = tmp.remove(0);
            dest_stack.insert(0, item);
        }

        return !self.instructions.is_empty();
    }

    pub fn print(&self) -> String {
        println!("printing stacks!");
        println!("stack 1: {:?}", self.stacks.get(0));
        println!("stack 2: {:?}", self.stacks.get(1));
        println!("stack 3: {:?}", self.stacks.get(2));
        println!("done!\n\n--------");

        let mut tallest_stack = 0;
        for st in &self.stacks {
            if st.len() > tallest_stack {
                tallest_stack = st.len();
            }
        }
        tallest_stack = tallest_stack - 1;

        let mut output = String::from("");

        for height in (0..=(tallest_stack)).rev() {
            self.stacks.iter().enumerate().for_each(|(idx, x)| {
                if let Some(_) = x.get(tallest_stack - height) {
                    if let Some(i) = x.get(height) {
                        output.push('[');
                        output.push(*i);
                        output.push(']');
                        println!(
                            "height: {}, idx: {}, item: {:?}",
                            tallest_stack - height,
                            idx,
                            i
                        );
                    } else {
                        output.push_str("   ");
                        println!("height: {}, idx: {}, item: NA", tallest_stack - height, idx);
                    }
                } else {
                    output.push_str("   ");
                    println!("height: {}, idx: {}, item: NA", tallest_stack - height, idx);
                }
            });
            output.push('\n');
        }

        println!("tallest stack has {} crates", tallest_stack);
        /*
            [D]
        [N] [C]
        [Z] [M] [P]
         1   2   3
         */

        println!("output:\n\n{}\n\n-----------------", output);

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_docks() -> Docks {
        let input: Vec<String> = vec![
            "    [D]    ".to_string(),
            "[N] [C]    ".to_string(),
            "[Z] [M] [P]".to_string(),
            " 1   2   3 ".to_string(),
            "".to_string(),
            "move 1 from 2 to 1".to_string(),
            "move 3 from 1 to 3".to_string(),
            "move 2 from 2 to 1".to_string(),
            "move 1 from 1 to 2".to_string(),
        ];
        let result = Docks::parse(input);
        let docks: Docks;

        match result {
            Err(e) => {
                panic!("got error trying to build Docks to test: {}", e);
            }
            Ok(s) => docks = s,
        }
        docks
    }

    #[test]
    fn can_step() {
        let mut docks = build_docks();
        let more = docks.step();
        assert_eq!(true, more);
        assert_eq!(vec!['D', 'N', 'Z'], docks.get_column(1).unwrap());
        assert_eq!(vec!['C', 'M'], docks.get_column(2).unwrap());

        let more = docks.step();
        assert_eq!(true, more);
        assert_eq!(Vec::<char>::new(), docks.get_column(1).unwrap());
        assert_eq!(vec!['Z', 'N', 'D', 'P'], docks.get_column(3).unwrap());

        let more = docks.step();
        assert_eq!(true, more);
        assert_eq!(vec!['M', 'C'], docks.get_column(1).unwrap());
        assert_eq!(Vec::<char>::new(), docks.get_column(2).unwrap());

        let more = docks.step();
        assert_eq!(false, more);
        assert_eq!(vec!['C'], docks.get_column(1).unwrap());
        assert_eq!(vec!['M'], docks.get_column(2).unwrap());
    }

    #[test]
    fn can_print() {
        let mut docks = build_docks();

        assert_eq!(
            "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3 "
                .to_string(),
            docks.print()
        );

        let _ = docks.step();
        assert_eq!(
            "[D]
[N] [C]
[Z] [M] [P]
 1   2   3 "
                .to_string(),
            docks.print()
        );
    }

    #[test]
    fn parsing_file_works_correct_number_of_columns() {
        let docks = build_docks();

        assert_eq!(3, docks.columns());
    }

    #[test]
    fn get_error_for_column_zero() {
        let docks = build_docks();

        match docks.get_column(0) {
            Err(_) => assert!(true),
            Ok(_) => panic!("shouldn't be able to get column '0'"),
        }
    }

    #[test]
    fn get_error_for_invalid_column_number() {
        let docks = build_docks();

        match docks.get_column(10) {
            Err(_) => assert!(true),
            Ok(_) => panic!("shouldn't be able to get column '0'"),
        }
    }

    #[test]
    fn correct_values_in_first_stack() {
        let docks = build_docks();
        match docks.get_column(1) {
            Err(_) => panic!("expected a first column"),
            Ok(c) => assert_eq!(vec!['N', 'Z'], c),
        }
    }

    #[test]
    fn correct_values_in_second_stack() {
        let docks = build_docks();

        match docks.get_column(2) {
            Err(_) => panic!("expected a second column"),
            Ok(c) => assert_eq!(vec!['D', 'C', 'M'], c),
        }
    }

    #[test]
    fn correct_values_in_third_stack() {
        let docks = build_docks();

        match docks.get_column(3) {
            Err(_) => panic!("expected a third column"),
            Ok(c) => assert_eq!(vec!['P'], c),
        }
    }
}
