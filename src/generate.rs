use crate::define::{Block, FttMap, Position};
use crate::error::{Error, Result};
use rand::Rng;
use rand::{rngs::StdRng, SeedableRng};

/// Generate a game map
pub fn generate(
    width: usize,
    height: usize,
    placed_blocks: &Vec<(Block, f64)>,
    seed: Option<u64>,
) -> Result<(FttMap, u64)> {
    if placed_blocks.iter().map(|x| x.1).sum::<f64>() > 1.0 {
        let msg = "The sum of probabilities cannot be greater than 1.";
        return Err(Error::Param(msg.to_string()));
    }

    let seed = seed.unwrap_or(rand::thread_rng().gen());
    let mut rng = StdRng::seed_from_u64(seed);
    let mut map = Vec::new();
    let mut empty_blocks = Vec::new();

    for row in 0..height {
        if [0, height - 1].contains(&row) {
            map.push(vec![Some(Block::Wall); width]);
            continue;
        }
        map.push(Vec::new());
        for col in 0..width {
            if [0, width - 1].contains(&col) {
                map[row].push(Some(Block::Wall));
                continue;
            }
            map[row].push(None);
            empty_blocks.push((row, col));
            let mut threshold = rng.gen_range(0.0..=1.0);
            for (block, probability) in placed_blocks {
                if threshold < *probability {
                    map[row][col] = Some(block.clone());
                    empty_blocks.pop();
                    break;
                }
                threshold -= *probability;
            }
        }
    }

    let mut allocated = Vec::new();

    let mut allocate = || -> Position {
        if empty_blocks.is_empty() {
            loop {
                let row = rng.gen_range(1..height - 1);
                let col = rng.gen_range(1..width - 1);
                let position = (row, col);
                dbg!(allocated.clone(), position);
                if allocated.contains(&position) {
                    continue;
                }
                map[row][col] = None;
                allocated.push(position);
                break position;
            }
        } else {
            let idx = rng.gen_range(0..empty_blocks.len());
            empty_blocks.remove(idx)
        }
    };

    Ok((
        FttMap {
            player: allocate(),
            target: allocate(),
            map,
        },
        seed,
    ))
}
