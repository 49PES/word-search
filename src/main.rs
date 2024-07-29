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

/// Board with 2D character vector for the grid and 1D String vector for the words
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
        let mut word_indices: HashSet<Posn> = HashSet::new();
        word_indices.insert(Posn { row: 1, col: 2 }); // For testing colors

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

    // fn get_positions(&self) -> Vec<Posn> {
    //     (0..self.row_len)
    //         .flat_map(|r| {
    //             (0..self.col_len)
    //                 .map(|c| Posn::new(r, c))
    //                 .collect::<Vec<Posn>>()
    //         })
    //         .collect()
    // }
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

    let board: Board = Board::new(grid, words);
    println!("{}", board);
}
