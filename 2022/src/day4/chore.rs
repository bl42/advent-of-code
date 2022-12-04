use std::ops::Range;
use std::str::FromStr;

#[derive(Debug)]
pub struct Chore {
    range: Range<u32>,
}

impl Chore {
    pub fn contains(&self, c: &Chore) -> bool {
        if self.range.start <= c.range.start && c.range.end <= self.range.end {
            return true;
        }
        false
    }
    pub fn contains_or_cotained(&self, c: &Chore) -> bool {
        self.contains(c) || c.contains(self)
    }
    pub fn overlaps(&self, c: &Chore) -> bool {
        (self.range.start <= c.range.start && c.range.start <= self.range.end)
            || (self.range.start <= c.range.end && c.range.end <= self.range.end)
    }
}

impl FromStr for Chore {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bounds: Vec<&str> = s.split('-').collect();
        let start = bounds[0].parse::<u32>().unwrap();
        let end = bounds[1].parse::<u32>().unwrap();

        Ok(Chore {
            range: (start..end),
        })
    }
}

#[test]
fn chores_contains() {
    let chore_a: Chore = "1-2".parse().unwrap();
    let chore_b: Chore = "1-5".parse().unwrap();

    assert_eq!(chore_b.contains(&chore_a), true);
    assert_eq!(chore_a.contains(&chore_b), false);
}
#[test]
fn chores_contains_or_cotained() {
    let chore_a: Chore = "1-2".parse().unwrap();
    let chore_b: Chore = "1-5".parse().unwrap();

    assert_eq!(chore_b.contains_or_cotained(&chore_a), true);
    assert_eq!(chore_a.contains_or_cotained(&chore_b), true);
}

#[test]
fn overlaps() {
    let chore_a: Chore = "1-2".parse().unwrap();
    let chore_b: Chore = "2-5".parse().unwrap();

    assert_eq!(chore_b.overlaps(&chore_a), true);
    assert_eq!(chore_a.overlaps(&chore_b), true);
}
