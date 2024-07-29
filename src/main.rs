use std::fmt::Display;
use std::fs;

/// Board with 2D character vector for the grid and 1D String vector for the words
#[derive(Debug)]
struct Board {
    grid: Vec<Vec<char>>,
    words: Vec<String>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for ch in row {
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }

        writeln!(f)?;
        writeln!(f, "{:?}", self.words)?;

        Ok(())
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

    let board: Board = Board { grid, words };
    println!("{}", board);
}
