use colored::Colorize;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Posn {
    row: usize,
    col: usize,
}

/// Board with 2D character vector for the grid and 1D String vector for the words
#[derive(Debug)]
struct Board {
    grid: Vec<Vec<char>>,
    words: Vec<String>,
    word_indices: HashSet<Posn>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                let ch: char = self.grid[i][j];
                let posn = Posn { row: i, col: j };
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
        let mut indices: HashSet<Posn> = HashSet::new();
        indices.insert(Posn { row: 1, col: 2 });
        Board {
            grid,
            words,
            word_indices: indices,
        }
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

    let board: Board = Board::new(grid, words);
    println!("{}", board);
}
