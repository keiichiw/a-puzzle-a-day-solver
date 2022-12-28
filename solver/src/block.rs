use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

use anyhow::{bail, Result};

use crate::point::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PuzzleType {
    /// DragonFjord's [A-Puzzle-A-Day](https://www.dragonfjord.com/product/a-puzzle-a-day/).
    DragonFjord = 0,
    /// JarringWords's [Calendar Puzzle](https://www.etsy.com/jp/listing/1032608229/).
    JarringWords = 1,
    /// Tetromino Type [Puzzle containing quad pieces](https://puzzleparadise.net/listing/puzzle-calendar-solve-for-each-day-of-the-year-cherry-pieces-and-walnut-border/107535)
    Tetromino = 2,
    /// Weekday Type [Puzzle containing weekday pieces](https://www.amazon.com/gp/product/B09NVQSY7Q)
    WeekDay = 3,
}

impl FromStr for PuzzleType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "d" | "D" | "dragonfjord" | "DragonFjord" => Ok(Self::DragonFjord),
            "j" | "J" | "jarringwords" | "JarringWords" => Ok(Self::JarringWords),
            "t" | "T" | "tetromino" | "Tetromino" => Ok(Self::Tetromino),
            "w" | "W" | "weekday" | "WeekDay" => Ok(Self::WeekDay),
            _ => bail!("'{}' is invalid puzzle type", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Block {
    pub ps: Vec<Point>,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let min_x = self.ps.iter().map(|p| p.x).min().unwrap();
        let max_x = self.ps.iter().map(|p| p.x).max().unwrap();
        let min_y = self.ps.iter().map(|p| p.y).min().unwrap();
        let max_y = self.ps.iter().map(|p| p.y).max().unwrap();

        let base = Point::new(min_x, min_y);
        let height = max_x - min_x + 1;
        let width = max_y - min_y + 1;
        let mut board = vec![vec!['.'; width as usize]; height as usize];

        for p in &self.ps {
            board[(p.x - base.x) as usize][(p.y - base.y) as usize] = '#';
        }

        for line in board {
            writeln!(f, "{:?}", line)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum Piece {
    HexRect,   // Hexomino, Rectangle
    PentL,     // Pentomino, L
    PentN,     // Pentomino, N
    PentP,     // Pentomino, P
    PentT,     // Pentomino, T
    PentU,     // Pentomino, U
    PentV,     // Pentomino, V
    PentY,     // Pentomino, Y
    PentZ,     // Pentomino, Z
    TetSquare, // Tetromino, Square
    TetL,      // Tetromino, L
    TetT,      // Tetromino, T
    TetI,      // Tetromino, I
    TetZ,      // Tetromino, Z
    WeekZ,     // Weekday, Z
}

impl From<Piece> for Block {
    fn from(p: Piece) -> Self {
        const BLOCK_SETS: [&[&str]; 15] = [
            &["###", "###"],           // Hexomino, Rectangle
            &["##", ".#", ".#", ".#"], // Pentomino, L
            &["#.", "##", ".#", ".#"], // Pentomino, N
            &["##.", "###"],           // Pentomino, P
            &["###", ".#.", ".#."],    // Pentomino, T
            &["#.#", "###"],           // Pentomino, U
            &["###", "#..", "#.."],    // Pentomino, V
            &["#.", "##", "#.", "#."], // Pentomino, Y
            &["##.", ".#.", ".##"],    // Pentomino, Z
            &["##", "##"],             // Tetromino, Square
            &["##", ".#", ".#"],       // Tetromino, L
            &["###", ".#."],           // Tetromino, T
            &["####"],                 // Tetromino, I
            &[".##", "##."],           // Tetromino, Z
            &["###.", "..##"],         // WeekDay, Z
        ];
        Self::from_strs(BLOCK_SETS[p as usize]).unwrap()
    }
}

impl Block {
    pub fn from_strs(b: &[&str]) -> Result<Self> {
        let mut ps = vec![];

        for (i, line) in b.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    ps.push(Point::new(i as i32, j as i32));
                }
            }
        }

        Self::normalize(&mut ps);
        Ok(Self { ps })
    }

    // Rotate 90 degrees.
    pub fn rot(&self) -> Self {
        let mut ps = self.ps.iter().map(|p| p.rot()).collect::<Vec<_>>();
        Self::normalize(&mut ps);
        Self { ps }
    }

    fn flip(&self) -> Self {
        let mut ps = self.ps.iter().map(|p| p.flip()).collect::<Vec<_>>();
        Self::normalize(&mut ps);
        Self { ps }
    }

    pub fn possible_dirs(&self, allow_flip: bool) -> BTreeSet<Self> {
        let mut s = BTreeSet::new();
        let mut b = self.clone();
        for _ in 0..4 {
            s.insert(b.clone());
            b = b.rot();
        }
        if allow_flip {
            b = b.flip();
            for _ in 0..4 {
                s.insert(b.clone());
                b = b.rot();
            }
        }
        s
    }

    pub fn get_blocks(typ: PuzzleType) -> Vec<Self> {
        use Piece::*;
        let pieces = match typ {
            PuzzleType::DragonFjord => {
                // DragonFjord uses `PentZ`
                vec![HexRect, PentL, PentN, PentP, PentU, PentV, PentY, PentZ]
            }
            PuzzleType::JarringWords => {
                // JarringWords uses `PentT`
                vec![HexRect, PentL, PentN, PentP, PentU, PentV, PentY, PentT]
            }
            PuzzleType::Tetromino => {
                // Tetromino uses a bunch of tetrominoes
                vec![
                    HexRect, PentV, PentU, PentP, TetSquare, TetL, TetT, TetZ, TetI,
                ]
            }
            PuzzleType::WeekDay => {
                // WeekDay uses `PentL`
                vec![
                    PentP, PentL, PentZ, TetZ, PentT, PentU, PentV, TetI, TetL, WeekZ,
                ]
            }
        };
        pieces.iter().map(|p| Self::from(*p)).collect::<Vec<_>>()
    }

    fn normalize(ps: &mut [Point]) {
        ps.sort();
        let min_x = ps.iter().map(|p| p.x).min().unwrap();
        let min_y = ps.iter().map(|p| p.y).min().unwrap();
        for p in ps.iter_mut() {
            p.x -= min_x;
            p.y -= min_y;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rot_z() {
        #[rustfmt::skip]
        let z = vec![
            "###",
            ".#.",
            "###",
        ];
        let b = Block::from_strs(&z).unwrap();
        assert_ne!(b, b.rot());
        assert_eq!(b, b.rot().rot());
    }

    #[test]
    fn test_rot_plus() {
        #[rustfmt::skip]
        let plus = vec![
            ".#.",
            "###",
            ".#.",
        ];

        let mut cur = Block::from_strs(&plus).unwrap();
        for _ in 0..4 {
            let next = cur.rot();
            assert_eq!(cur, next);
            cur = next;
        }
    }

    #[test]
    fn test_rot_bar() {
        #[rustfmt::skip]
        let bar = Block::from_strs(&[
            "#",
            "#",
            "#"
        ]).unwrap();
        #[rustfmt::skip]
        let minus = Block::from_strs(&[
            "###",
        ]).unwrap();

        assert_ne!(bar, minus);
        assert_eq!(bar.rot(), minus);
        assert_eq!(bar.rot().rot(), bar);
    }

    #[test]
    fn test_bar_padding() {
        #[rustfmt::skip]
        let bar1 = Block::from_strs(&[
            "#",
            "#",
            "#"
        ]).unwrap();
        #[rustfmt::skip]
        let bar2 = Block::from_strs(&[
            "...#...",
            "...#",
            "...#..."
        ]).unwrap();

        assert_eq!(bar1, bar2);
    }
}
