fn main() {
    let forest = include_str!("input.txt")
        .lines()
        .map(|line| line.bytes().map(|c| c - '0' as u8).collect())
        .collect();

    println!("Part1: {}", part1(&forest));
    println!("Part2: {}", part2(&forest));
}

fn part1(forest: &Vec<Vec<u8>>) -> u16 {
    let mut total_visible = 0;
    let depth = forest.len();
    let width = forest[0].len();
    for row in 0..depth {
        for col in 0..width {
            let tree_height = forest[row][col];
            if (0..row).all(|row_above| forest[row_above][col] < tree_height)
                || ((row + 1)..depth).all(|row_below| forest[row_below][col] < tree_height)
                || (0..col).all(|col_left| forest[row][col_left] < tree_height)
                || ((col + 1)..width).all(|col_right| forest[row][col_right] < tree_height)
            {
                total_visible += 1;
            }
        }
    }
    total_visible
}

fn part2(forest: &Vec<Vec<u8>>) -> usize {
    let mut max_score = 0;
    let depth = forest.len();
    let width = forest[0].len();
    for row in 0..depth {
        for col in 0..width {
            let tree_height = forest[row][col];
            let dist_above = (0..row)
                .rev()
                .position(|row_above| forest[row_above][col] >= tree_height)
                .and_then(|score| Some(score + 1))
                .unwrap_or(row);
            let dist_below = ((row + 1)..depth)
                .position(|row_below| forest[row_below][col] >= tree_height)
                .and_then(|score| Some(score + 1))
                .unwrap_or(if depth > row { depth - row - 1 } else { 0 });
            let dist_left = (0..col)
                .rev()
                .position(|col_left| forest[row][col_left] >= tree_height)
                .and_then(|score| Some(score + 1))
                .unwrap_or(col);
            let dist_right = ((col + 1)..width)
                .position(|col_right| forest[row][col_right] >= tree_height)
                .and_then(|score| Some(score + 1))
                .unwrap_or(if width > col { width - col - 1 } else { 0 });
            let tree_score = dist_above * dist_below * dist_left * dist_right;
            // println!(
            //     "{}, {}: {}, {}, {}, {}; {}",
            //     row, col, dist_above, dist_below, dist_left, dist_right, tree_score
            // );
            max_score = max_score.max(tree_score);
        }
    }
    max_score
}
