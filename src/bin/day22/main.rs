#![feature(iter_intersperse)]

use std::{
    borrow::BorrowMut,
    fmt::Display,
    str::{Chars, FromStr},
};

use lib::common_startup::startup;
use log::{debug, info, trace};
use Cell::*;
use Direction::*;
use Instruction::*;

fn main() {
    let cli = startup();
    let input = if cli.sample {
        include_str!("sample_input.txt")
    } else {
        include_str!("input.txt")
    };

    let (board_str, instructions_str) = input.split_once("\n\n").unwrap();

    let board: Board = board_str.parse().unwrap();
    let instructions = InstructionIter::new(instructions_str);
    debug!("{}", board);

    info!("Part1: {}", part1(&board, &instructions));
    // info!("Part2: {}", part2());
}

fn part1(board: &Board, instructions: &InstructionIter) -> isize {
    let mut state = PathState::new(board);
    for instruction in instructions.clone() {
        state.follow_instruction(instruction);
    }
    (state.pos.0 + 1) * 1000 + (state.pos.1 + 1) * 4 + state.dir as isize
}

fn part2() {}

struct PathState {
    pos: (isize, isize),
    dir: Direction,
    board: Board,
}

impl PathState {
    fn new(board: &Board) -> Self {
        let board = board.clone();
        let pos = (
            0,
            board.board[0].iter().position(|c| *c == Space).unwrap() as isize,
        );
        let dir = Right;
        PathState { board, pos, dir }
    }

    fn follow_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Move(n) => self.pos = self.board.take_steps(self.pos, self.dir, n),
            TurnLeft => {
                self.dir = match self.dir {
                    Up => Left,
                    Down => Right,
                    Left => Down,
                    Right => Up,
                }
            }
            TurnRight => {
                self.dir = match self.dir {
                    Up => Right,
                    Down => Left,
                    Left => Up,
                    Right => Down,
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct InstructionIter {
    instruction_chars: Chars<'static>,
    last_char: Option<char>,
    flag: bool,
}

impl InstructionIter {
    fn new(instruction_str: &'static str) -> Self {
        InstructionIter {
            instruction_chars: instruction_str.trim().chars(),
            last_char: None,
            flag: false,
        }
    }
}

impl Iterator for InstructionIter {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.flag {
            return None;
        }

        match self.last_char {
            Some('L') => {
                self.last_char = None;
                return Some(TurnLeft);
            }
            Some('R') => {
                self.last_char = None;
                return Some(TurnRight);
            }
            Some(c) => panic!("Unexpected char: {}", c),
            None => (),
        }

        let mut digits = vec![];

        while let Some(c) = self.instruction_chars.next() {
            trace!("{}", c);
            if !c.is_digit(10) {
                self.last_char = Some(c);
                break;
            }
            digits.push(c);
        }

        if digits.len() > 0 {
            Some(Move(
                digits.into_iter().collect::<String>().parse().unwrap(),
            ))
        } else {
            self.flag = true;
            None
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
    Move(u8),
    TurnLeft,
    TurnRight,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up = 3,
    Down = 1,
    Left = 2,
    Right = 0,
}

impl Direction {
    fn inc(&self) -> (isize, isize) {
        match self {
            Up => (-1, 0),
            Down => (1, 0),
            Left => (0, -1),
            Right => (0, 1),
        }
    }
}

#[derive(Clone, Debug)]
struct Board {
    board: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Board {
    fn take_steps(&self, pos: (isize, isize), dir: Direction, dist: u8) -> (isize, isize) {
        let mut new_pos = pos;
        for _ in 0..dist {
            if let Some(step_pos) = self.take_step(new_pos, dir) {
                new_pos = step_pos;
            } else {
                break;
            }
        }
        new_pos
    }

    fn take_step(&self, pos: (isize, isize), dir: Direction) -> Option<(isize, isize)> {
        let inc = dir.inc();
        let mut new_pos = (pos.0 + inc.0, pos.1 + inc.1);

        if new_pos.0 < self.height as isize
            && new_pos.0 >= 0
            && new_pos.1 < self.width as isize
            && new_pos.1 >= 0
        {
            match self.board[new_pos.0 as usize][new_pos.1 as usize] {
                Space => {
                    return Some(new_pos);
                }
                Wall => {
                    return None;
                }
                Buff => (),
            };
        }

        new_pos = match dir {
            Up => (self.height as isize - 1, new_pos.1),
            Down => (0, new_pos.1),
            Left => (new_pos.0, self.width as isize - 1),
            Right => (new_pos.0, 0),
        };

        loop {
            match self.board[new_pos.0 as usize][new_pos.1 as usize] {
                Space => return Some(new_pos),
                Wall => return None,
                Buff => (),
            }
            new_pos = (new_pos.0 + inc.0, new_pos.1 + inc.1);
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.board
                .iter()
                .map(|row| row
                    .into_iter()
                    .map(|cell| format!("{}", cell))
                    .collect::<String>()
                    .trim_end()
                    .to_owned())
                .intersperse("\n".to_owned())
                .collect::<String>()
        )
    }
}

impl FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().map(|line| line.len()).max().unwrap();
        let height = s.lines().count();
        let mut board = vec![vec![Buff; width]; height];
        for (row, row_str) in s.lines().enumerate() {
            for (col, cell_char) in row_str.chars().enumerate() {
                board[row][col] = Cell::new(cell_char);
            }
        }
        Ok(Board {
            board,
            width,
            height,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Space,
    Wall,
    Buff,
}

impl Cell {
    fn new(cell_char: char) -> Self {
        match cell_char {
            '.' => Space,
            '#' => Wall,
            ' ' => Buff,
            _ => panic!("Unexpected char"),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Space => ".",
            Wall => "#",
            Buff => " ",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test;

    #[test]
    fn parse_board() {
        let test_str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.";
        let board: Board = test_str.parse().unwrap();
        assert_eq!(format!("{}", board), test_str);
    }

    #[test]
    fn parse_instructions() {
        let instructions = InstructionIter::new("10R5L5R");
        assert_eq!(
            instructions.collect::<Vec<_>>(),
            vec![Move(10), TurnRight, Move(5), TurnLeft, Move(5), TurnRight]
        );
    }
}
