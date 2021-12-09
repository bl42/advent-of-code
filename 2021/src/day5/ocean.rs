use std::ops::Range;
use std::str::FromStr;

fn fancy_fish_range(start: usize, finish: usize) -> Range<usize> {
    if start < finish {
        start..finish + 1
    } else {
        finish..start + 1
    }
}

#[derive(Debug)]
pub struct Floor {
    vents: Vec<Vent>,
    map: Vec<Vec<u16>>,
}

impl Floor {
    pub fn vents_overlap(self) -> i32 {
        let mut overlap: i32 = 0;

        for (x, rows) in self.map.iter().enumerate() {
            for value in rows.iter() {
                if value >= &2 {
                    overlap += 1;
                }
            }
        }

        overlap
    }
    pub fn find_horizontal_segments(&mut self) {
        for vent in &self.vents {
            if vent.x1 == vent.x2 {
                for i in fancy_fish_range(vent.y1, vent.y2) {
                    self.map[i][vent.x1] += 1
                }
            }
        }
    }
    pub fn find_vertical_segments(&mut self) {
        for vent in &self.vents {
            if vent.y1 == vent.y2 {
                for i in fancy_fish_range(vent.x1, vent.x2) {
                    self.map[vent.y1][i] += 1
                }
            }
        }
    }
    pub fn find_diagonal_segments(&mut self) {
        for vent in &self.vents {
            let angle = vent.angle();

            match angle as i16 {
                45 => {
                    for i in fancy_fish_range(0, vent.x1 - vent.x2) {
                        self.map[vent.x2 + i][vent.y2 + i] += 1;
                    }
                }
                -45 => {
                    for i in fancy_fish_range(0, vent.x2 - vent.x1) {
                        self.map[vent.x1 + i][vent.y1 - i] += 1;
                    }
                }
                135 => {
                    for i in fancy_fish_range(0, vent.x1 - vent.x2) {
                        self.map[vent.x1 - i][vent.y1 + i] += 1;
                    }
                }
                -135 => {
                    for i in fancy_fish_range(0, vent.x2 - vent.x1) {
                        self.map[vent.x1 + i][vent.y1 + i] += 1;
                    }
                }
                _ => {}
            }
        }
    }
}

impl FromStr for Floor {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let vents: Vec<Vent> = lines.iter().map(|l| l.parse::<Vent>().unwrap()).collect();

        Ok(Self {
            vents,
            map: vec![vec![0; 1000]; 1000],
        })
    }
}

#[derive(Debug)]
struct Vent {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Vent {
    fn angle(&self) -> f64 {
        let x1 = self.x1 as f64;
        let x2 = self.x2 as f64;
        let y1 = self.y1 as f64;
        let y2 = self.y2 as f64;

        let x = x1 - x2;
        let y = y1 - y2;
        let rads = x.atan2(y);

        rads.to_degrees()
    }
    fn distance(&self) -> f64 {
        let a = if self.x1 > self.x2 {
            self.x1 - self.x2
        } else {
            self.x2 - self.x1
        };
        let b = if self.y1 > self.y2 {
            self.y1 - self.y2
        } else {
            self.y2 - self.y1
        };

        (((a * a) + (b * b)) as f64).sqrt()
    }
}

impl FromStr for Vent {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(" -> ").unwrap();
        let (x1, y1) = left.split_once(',').unwrap();
        let (x2, y2) = right.split_once(',').unwrap();

        Ok(Self {
            x1: x1.parse().unwrap(),
            y1: y1.parse().unwrap(),
            x2: x2.parse().unwrap(),
            y2: y2.parse().unwrap(),
        })
    }
}
