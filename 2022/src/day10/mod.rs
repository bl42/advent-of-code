// https://adventofcode.com/2022/day/10
// --- Day 10 ---

fn part1(input: &str) -> i32 {
    let signals: Vec<(&str, Option<i32>)> = input
        .lines()
        .map(|sig| {
            if sig == "noop" {
                return (sig, None);
            }
            let (sig, n) = sig.split_once(' ').unwrap();
            (sig, Some(n.parse().unwrap()))
        })
        .collect();

    let mut batch_totals: Vec<i32> = Vec::new();
    batch_totals.insert(0, 1);
    let mut cycle = 0;

    for (_, num) in signals {
        if let Some(num) = num {
            cycle += 2;

            let batch = if cycle < 20 {
                0
            } else {
                (f64::ceil(((cycle - 20) / 40) as f64) as usize) + 1
            };

            if let Some(total) = batch_totals.get_mut(batch) {
                *total += num;
            } else {
                let prev_batch = batch_totals.get(batch - 1).unwrap_or(&0);
                batch_totals.insert(batch, num + prev_batch)
            }
        } else {
            cycle += 1;
        }
    }

    let mut sum: Vec<i32> = batch_totals
        .iter()
        .enumerate()
        .map(|(i, total)| total * (((i as i32) * 40) + 20))
        .collect();

    sum.drain(6..);

    sum.iter().sum::<i32>()
}

fn part2(input: &str) -> &str {
    let signals: Vec<(&str, Option<i32>)> = input
        .lines()
        .map(|sig| {
            if sig == "noop" {
                return (sig, None);
            }
            let (sig, n) = sig.split_once(' ').unwrap();
            (sig, Some(n.parse().unwrap()))
        })
        .collect();

    let mut batch_totals: Vec<i32> = Vec::new();
    let mut screen = vec![vec!['.'; 40]; 6];
    batch_totals.insert(0, 1);
    let mut cycle = 1;

    let mut sig = signals.iter();

    for clock in 0.. {
        let row_index = f64::ceil((clock / 40) as f64) as usize;
        let pixel_index = clock % 40;

        if let Some(p) = batch_totals.get(row_index) {
            mark_screen(&mut screen, (row_index, pixel_index), p - 1);
            mark_screen(&mut screen, (row_index, pixel_index), *p);
            mark_screen(&mut screen, (row_index, pixel_index), p + 1);
        }

        if cycle == clock {
            if let Some((_, num)) = sig.next() {
                if let Some(num) = num {
                    cycle += 2;

                    let batch = f64::ceil((cycle / 40) as f64) as usize;

                    if let Some(total) = batch_totals.get_mut(batch) {
                        *total += num;
                    } else {
                        let prev_batch = batch_totals.get(batch - 1).unwrap_or(&0);
                        batch_totals.insert(batch, num + prev_batch)
                    }
                } else {
                    cycle += 1;
                }
            } else {
                break;
            }
        }
    }

    // println!("---SCREEN---");
    // for row in screen {
    //     let mut s = String::new();
    //     for pixel in row {
    //         s += &pixel.to_string();
    //     }
    //     println!("{s}");
    // }
    // println!("---SCREEN---");

    "hidden"
}

fn mark_screen(screen: &mut [Vec<char>], position: (usize, usize), pixel: i32) {
    if pixel < 0 {
        return;
    };
    let pixel = pixel as usize;

    if position.1 == pixel {
        if let Some(x) = screen.get_mut(position.0) {
            if let Some(y) = x.get_mut(position.1) {
                *y = '#';
            }
        }
    }
}

pub fn run() -> String {
    let input = include_str!("input.txt");

    format!("Day 10.1: {} \t Day 10.2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = include_str!("input.sample.txt");

    #[test]
    fn first() {
        assert_eq!(part1(SAMPLE_INPUT), 13140);
    }
    #[test]
    fn second() {
        assert_eq!(part2(SAMPLE_INPUT), "hidden");
    }
}
