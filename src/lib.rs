pub struct ElfCollection {
    elves: Vec<Elf>,
}

struct Elf {
    carrying: Vec<u32>,
    total: u32,
}

impl ElfCollection {
    pub fn new() -> ElfCollection {
        ElfCollection {
            elves: vec![Elf::new()],
        }
    }

    pub fn add(&mut self, input: &str) {
        if input == "" {
            self.elves.push(Elf::new());
            return;
        }

        if let Ok(val) = input.parse::<u32>() {
            if let Some(elf) = self.elves.last_mut() {
                elf.push(val);
            }
        }
    }

    pub fn totals(&self) -> Vec<u32> {
        let mut totals: Vec<u32> = Vec::new();

        for elf in &self.elves {
            totals.push(elf.total());
        }

        totals
    }

    pub fn biggest(&self) -> u32 {
        let totals = self.totals();
        let mut largest = 0;

        for t in totals {
            if t > largest {
                largest = t;
            }
        }

        largest
    }

    pub fn top_three(&self) -> Vec<u32> {
        let mut totals = self.totals();
        totals.sort();

        let mut vals = Vec::new();
        for _ in 1..=3 {
            if let Some(val) = totals.pop() {
                vals.push(val);
            }
        }

        vals
    }
}

impl Elf {
    fn new() -> Elf {
        Elf {
            carrying: Vec::new(),
            total: 0,
        }
    }

    fn push(&mut self, val: u32) {
        self.carrying.push(val);
        self.total = self.carrying.iter().sum();
    }

    fn total(&self) -> u32 {
        self.total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_items_to_elves() {
        let items = vec!["100", "200", "", "400", "", "500", "600"];
        let mut ec = ElfCollection::new();

        for i in items {
            ec.add(i);
        }

        let got = ec.totals();

        assert_eq!(vec![300, 400, 1100], got);
    }

    #[test]
    fn get_biggest_total() {
        let items = vec!["100", "200", "", "400", "", "500", "600"];
        let mut ec = ElfCollection::new();

        for i in items {
            ec.add(i);
        }

        assert_eq!(1100, ec.biggest());
    }

    #[test]
    fn get_top_three() {
        let items = vec![
            "100", "200", "", "400", "", "500", "600", "", "800", "800", "", "900", "1000",
        ];
        let mut ec = ElfCollection::new();

        for i in items {
            ec.add(i);
        }

        // elves: [300, 400, 1100, 1600, 1900]
        // top three: 1100, 1600, 1900
        assert_eq!(vec![1900, 1600, 1100], ec.top_three());
    }
}
