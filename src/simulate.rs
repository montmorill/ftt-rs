use crate::{
    define::{Block, FttMap, Position},
    error::{Error, Result},
};
use Direction::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            'w' => Ok(Up),
            's' => Ok(Down),
            'a' => Ok(Left),
            'd' => Ok(Right),
            char => Err(Error::Param(format!("Direction cannot be {char}!"))),
        }
    }
}

const DELTA_ROW: [i128; 4] = [-1, 1, 0, 0];
const DELTA_COLUMN: [i128; 4] = [0, 0, -1, 1];
pub const DIRECTIONS: [Direction; 4] = [Up, Down, Left, Right];

fn next(pos: Position, dir: Direction) -> Position {
    let (row, col) = pos;
    let delta_row = DELTA_ROW[dir as usize];
    let delta_col = DELTA_COLUMN[dir as usize];
    let next_row = (row as i128 + delta_row) as usize;
    let next_col = (col as i128 + delta_col) as usize;
    (next_row, next_col)
}

/// Simulate once, returns if operation is legal
pub fn simulate(
    ftt: &mut FttMap,
    dir: Direction,
) -> bool {
    let (mut next_row, mut next_col) = next(ftt.player, dir);

    let obstacles = [Some(Block::Wall), Some(Block::Sand)];
    if obstacles.contains(&ftt.map[next_row][next_col]) {
        return false;
    }

    let mut flag = true;
    while flag && ftt.player != ftt.target {
        let (row, col) = ftt.player;
        ftt.player = (next_row, next_col);

        if ftt.map[next_row][next_col] == Some(Block::Cobweb) {
            flag = false;
        }

        (next_row, next_col) = next(ftt.player, dir);

        if ftt.map[row][col] == Some(Block::Piston) {
            ftt.map[row][col] = Some(Block::Wall);
        }

        if obstacles.contains(&ftt.map[next_row][next_col]) {
            flag = false;
        }

        for dir in DIRECTIONS {
            let (row, col) = next(ftt.player, dir);
            if ftt.map[row][col] == Some(Block::Sand) {
                ftt.map[row][col] = None;
            }
        }
    }

    true
}
