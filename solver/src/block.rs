use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::collections::BTreeSet;
use std::fmt;

use anyhow::Result;

use crate::point::Point;

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

        let base = Point::new(min_x as i32, min_y as i32);
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

    pub fn get_blocks() -> Vec<Self> {
        [
            vec!["###", "#..", "#.."],
            vec!["#.#", "###"],
            vec!["##.", ".#.", ".##"],
            vec!["#.", "##", "#.", "#."],
            vec!["#.", "##", ".#", ".#"],
            vec!["##", ".#", ".#", ".#"],
            vec!["##.", "###"],
            vec!["###", "###"],
        ]
        .iter()
        .map(|b| Block::from_strs(&b))
        .collect::<Result<Vec<_>>>()
        .unwrap()
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
