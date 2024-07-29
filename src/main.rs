use colored::Colorize;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Posn {
    row: usize,
    col: usize,
}

impl Posn {
    fn new(row: usize, col: usize) -> Self {
        Posn { row, col }
    }
}
#[derive(Debug)]
struct GridWord {
    word: String,
    posns: Vec<Posn>,
}

impl Display for GridWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.word)
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
}

impl Direction {
    fn get_dirs() -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::LeftUp,
            Direction::LeftDown,
            Direction::RightUp,
            Direction::RightDown,
        ]
    }
}

impl Direction {
    fn dir_to_offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::LeftDown => (1, -1),
            Direction::LeftUp => (-1, -1),
            Direction::RightDown => (1, 1),
            Direction::RightUp => (-1, 1),
        }
    }
}

#[derive(Debug)]
struct Board {
    grid: Vec<Vec<char>>,
    words: Vec<String>,
    row_len: usize,
    col_len: usize,
    word_indices: HashSet<Posn>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.row_len {
            for j in 0..self.col_len {
                let posn = Posn::new(i, j);
                let ch: char = self.get_letter(posn);
                if self.word_indices.contains(&posn) {
                    write!(f, "{}", ch.to_string().red())?;
                } else {
                    write!(f, "{}", ch)?;
                }
            }
            writeln!(f)?;
        }

        writeln!(f)?;
        writeln!(f, "{:?}", self.words)?;

        Ok(())
    }
}

impl Board {
    fn new(grid: Vec<Vec<char>>, words: Vec<String>) -> Self {
        let word_indices: HashSet<Posn> = HashSet::new();
        let row_len = grid.len();
        let col_len = grid[0].len();

        Board {
            grid,
            words,
            row_len,
            col_len,
            word_indices,
        }
    }

    fn get_letter(&self, posn: Posn) -> char {
        self.grid[posn.row][posn.col]
    }

    fn get_neighbor_in_dir(&self, posn: Posn, dir: &Direction) -> Option<Posn> {
        let (row_off, col_off): (i32, i32) = dir.dir_to_offset();
        let new_row = posn.row as i32 + row_off;
        let new_col = posn.col as i32 + col_off;
        if new_row < 0
            || new_row >= self.row_len as i32
            || new_col < 0
            || new_col >= self.col_len as i32
        {
            None
        } else {
            Some(Posn::new(new_row as usize, new_col as usize))
        }
    }

    fn get_positions(&self) -> Vec<Posn> {
        (0..self.row_len)
            .flat_map(|r| {
                (0..self.col_len)
                    .map(|c| Posn::new(r, c))
                    .collect::<Vec<Posn>>()
            })
            .collect()
    }

    fn grid_words(&self) -> Vec<GridWord> {
        self.get_positions()
            .iter()
            .flat_map(|&head| {
                Direction::get_dirs()
                    .iter()
                    .flat_map(|dir| {
                        let mut word: String = self.get_letter(head).to_string();
                        let mut posns: Vec<Posn> = vec![head];
                        let mut grid_words: Vec<GridWord> = vec![];

                        let mut curr_neighbor = self.get_neighbor_in_dir(head, dir);
                        while let Some(curr) = curr_neighbor {
                            word.push(self.get_letter(curr));
                            posns.push(curr);
                            grid_words.push(GridWord {
                                word: word.clone(),
                                posns: posns.clone(),
                            });
                            curr_neighbor = self.get_neighbor_in_dir(curr, dir);
                        }
                        grid_words
                    })
                    .collect::<Vec<GridWord>>()
            })
            .collect()
    }

    fn solve(&mut self) {
        self.word_indices = self
            .grid_words()
            .iter()
            .filter(|grid_word| self.words.contains(&grid_word.word))
            .flat_map(|grid_word| grid_word.posns.clone())
            .collect();
    }
}

fn main() {
    let grid_content: String = fs::read_to_string("../grid.txt").expect("Unable to open grid.txt");
    let words_content: String =
        fs::read_to_string("../words.txt").expect("Unable to open words.txt");

    let grid: Vec<Vec<char>> = grid_content
        .lines()
        .map(|line| line.to_string().chars().collect::<Vec<char>>())
        .collect();

    let words: Vec<String> = words_content.lines().map(|line| line.to_string()).collect();

    let mut board: Board = Board::new(grid, words);
    println!("{}", board);

    board.solve();
    println!("{}", board);
}
