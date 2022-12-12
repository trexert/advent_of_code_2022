use std::collections::HashMap;

use lib::dijkstra;

fn main() {
    let mut start = None;
    let mut end = None;
    let grid: Vec<Vec<u8>> = include_str!("input.txt")
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.bytes()
                .enumerate()
                .map(|(col, byte)| match byte as char {
                    'S' => {
                        start = Some((row as u8, col as u8));
                        'a' as u8
                    }
                    'E' => {
                        end = Some((row as u8, col as u8));
                        'z' as u8
                    }
                    something_else => something_else as u8,
                })
                .collect()
        })
        .collect();

    println!("Part1: {}", part1(&grid, start.unwrap(), end.unwrap()));

    let all_starts: Vec<(u8, u8)> = grid
        .iter()
        .enumerate()
        .flat_map(|(row, row_content)| {
            row_content
                .iter()
                .enumerate()
                .filter_map(move |(col, cell)| {
                    if *cell == 'a' as u8 {
                        Some((row as u8, col as u8))
                    } else {
                        None
                    }
                })
        })
        .collect();

    println!("Part2: {}", part2(&grid, &all_starts, end.unwrap()));
}

fn part1(grid: &Vec<Vec<u8>>, start: (u8, u8), end: (u8, u8)) -> u64 {
    let (_, result) = dijkstra::solve_uniform_edges(&build_map(grid), start, end).unwrap();
    result
}

fn part2(grid: &Vec<Vec<u8>>, starts: &Vec<(u8, u8)>, end: (u8, u8)) -> u64 {
    let map = build_map(grid);
    starts
        .iter()
        .filter_map(|start| {
            dijkstra::solve_uniform_edges(&map, *start, end).and_then(|result| Some(result.1))
        })
        .min()
        .unwrap()
}

fn build_map(grid: &Vec<Vec<u8>>) -> HashMap<(u8, u8), Vec<(u8, u8)>> {
    let rows = grid.len();
    let cols = grid[0].len();
    assert!(grid.iter().all(|row| row.len() == cols));

    let mut map = HashMap::new();
    for row in 0..rows {
        for col in 0..cols {
            let mut accessible = Vec::with_capacity(4);
            let current_height = grid[row][col];

            if row > 0 && current_height >= grid[row - 1][col] - 1 {
                accessible.push((row as u8 - 1, col as u8))
            }
            if row < rows - 1 && current_height >= grid[row + 1][col] - 1 {
                accessible.push((row as u8 + 1, col as u8))
            }
            if col > 0 && current_height >= grid[row][col - 1] - 1 {
                accessible.push((row as u8, col as u8 - 1))
            }
            if col < cols - 1 && current_height >= grid[row][col + 1] - 1 {
                accessible.push((row as u8, col as u8 + 1))
            }
            map.insert((row as u8, col as u8), accessible);
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use test;

    #[test]
    fn test_build_map() {
        let grid = vec![vec![1, 2], vec![4, 3], vec![5, 6]];
        let expected_result: HashMap<(u8, u8), Vec<(u8, u8)>> = vec![
            ((0, 0), vec![(0, 1)]),
            ((0, 1), vec![(1, 1), (0, 0)]),
            ((1, 0), vec![(0, 0), (2, 0), (1, 1)]),
            ((1, 1), vec![(0, 1), (1, 0)]),
            ((2, 0), vec![(1, 0), (2, 1)]),
            ((2, 1), vec![(1, 1), (2, 0)]),
        ]
        .into_iter()
        .collect();
        assert_eq!(expected_result, build_map(&grid));
    }
}
