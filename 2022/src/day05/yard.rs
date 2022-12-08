use std::str::FromStr;

pub struct Yard {
    stack: Vec<Vec<char>>,
}

impl Yard {
    pub fn relocate(&mut self, count: usize, from: usize, to: usize) {
        for _ in 1..=count {
            if let Some(item) = self.stack[from - 1].pop() {
                self.stack[to - 1].push(item);
            }
        }
    }

    pub fn relocate_batch(&mut self, count: usize, from: usize, to: usize) {
        let mut items: Vec<char> = Vec::new();
        for _ in 1..=count {
            if let Some(item) = self.stack[from - 1].pop() {
                items.push(item);
            }
        }

        items.reverse();

        self.stack[to - 1].append(&mut items);
    }

    pub fn message(&self) -> String {
        self.stack.iter().filter_map(|c| c.last()).collect()
    }
}

impl FromStr for Yard {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, _) = s.rsplit_once('\n').unwrap();

        let rows: Vec<Vec<String>> = s
            .split('\n')
            .map(|row| {
                row.chars()
                    .collect::<Vec<char>>()
                    .chunks(4)
                    .map(|c| c.iter().collect::<String>().trim().to_string())
                    .collect()
            })
            .collect();

        let mut stack: Vec<Vec<char>> = vec![Vec::new(); rows.len() + 1];

        for (y, row) in rows.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                if !item.is_empty() {
                    stack[x].insert(0, item.chars().nth(1).expect("should be in nth 1"))
                }
            }
        }

        Ok(Yard { stack })
    }
}
