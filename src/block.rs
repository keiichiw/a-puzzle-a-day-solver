use std::fmt;

use anyhow::Result;

use crate::point::Point;

#[derive(Debug, Clone, PartialEq, Eq)]
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

        for i in 0..b.len() {
            for (j, c) in b[i].chars().enumerate() {
                if c == '#' {
                    ps.push(Point::new(i as i32, j as i32));
                }
            }
        }

        Self::sort_ps(&mut ps);
        Ok(Self { ps })
    }

    // Rotate 90 degrees.
    pub fn rot(&self) -> Self {
        let mut ps = self.ps.iter().map(|p| p.rot()).collect::<Vec<_>>();
        Self::sort_ps(&mut ps);
        Self { ps }
    }

    fn sort_ps(ps: &mut [Point]) {
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
    fn test_rot() {
        #[rustfmt::skip]
        let v = vec![
            "##..",
            ".###",
        ];

        let b = Block::from_strs(&v).unwrap();
        let b2 = b.rot().rot().rot().rot();
        assert_eq!(b, b2);
    }
}
