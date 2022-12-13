// https://adventofcode.com/2022/day/12
// --- Day 12 ---

const SEARCH_FOR_NEXT_NODE: [(usize, usize); 4] =
    [(1, 0), (usize::MAX, 0), (0, 1), (0, usize::MAX)];

/// previous version ran for 10 hours and got to ~200 iterations.
/// ended up down a deep Google + reddit + wikipedia rabit hole...
/// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
/// https://en.m.wikipedia.org/wiki/Shortest_Path_Faster_Algorithm#:~:text=The%20Shortest%20Path%20Faster%20Algorithm,that%20contain%20negative%2Dweight%20edges
/// https://en.wikipedia.org/wiki/Breadth-first_search

fn calculate_len(map: &Vec<u8>, width: usize, height: usize, start: usize, end: usize) -> usize {
    let start = start - start / (width + 1);
    let end = end - end / (width + 1);

    let mut map = map.to_owned();

    map[start] = 0; // cast 'S' to 'a' on map
    map[end] = 25; // cast 'E' to 'z' on map

    let start = (start % width, start / width);
    let end = (end % width, end / width);

    if let Some(b_f_s) = pathfinding::directed::bfs::bfs(
        &end,
        |(x, y)| {
            let cur = map[y * width + x];
            SEARCH_FOR_NEXT_NODE
                .iter()
                .map(|(a, b)| (x.wrapping_add(*a), y.wrapping_add(*b)))
                .filter(|(x, y)| {
                    x < &width && y < &height && map[width * y + x] >= cur.saturating_sub(1)
                })
                .collect::<Vec<_>>()
        },
        |&p| p == start,
    ) {
        b_f_s.len() - 1
    } else {
        usize::MAX
    }
}

fn process_map(input: &str) -> (Vec<u8>, usize, usize) {
    let map: Vec<_> = input
        .bytes()
        .filter(|b| b != &b'\n')
        .map(|b| b.to_ascii_lowercase() - b'a')
        .collect();

    let width = input.bytes().position(|b| b == b'\n').unwrap();
    let height = map.len() / width;

    (map, width, height)
}

fn part1(input: &str) -> usize {
    let (map, width, height) = process_map(input);

    let start = input.bytes().position(|b| b == b'S').unwrap();
    let end = input.bytes().position(|b| b == b'E').unwrap();

    calculate_len(&map, width, height, start, end)
}

fn part2(input: &str) -> usize {
    let (map, width, height) = process_map(input);

    let starts = input
        .bytes()
        .enumerate()
        .filter_map(|(i, b)| {
            if b == b'S' || b == b'a' {
                Some(i)
            } else {
                None
            }
        })
        .into_iter()
        .collect::<Vec<usize>>();

    let end = input.bytes().position(|b| b == b'E').unwrap();

    starts
        .iter()
        .map(|&start| calculate_len(&map, width, height, start, end))
        .min()
        .unwrap()
}

pub fn run() -> String {
    let input = include_str!("input.txt");

    format!("Day 12.1: {} \t\t Day 12.2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = include_str!("input.sample.txt");

    #[test]
    fn first() {
        assert_eq!(part1(SAMPLE_INPUT), 31);
    }
    #[test]
    fn second() {
        assert_eq!(part2(SAMPLE_INPUT), 29);
    }
}
