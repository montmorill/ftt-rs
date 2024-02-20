use crate::define::FttMap;
use crate::simulate::{simulate, Direction, DIRECTIONS};
use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

/// Search for one of shortest possible solutions
pub fn search(
    ftt: FttMap,
    max_steps: usize,
    debug: bool,
) -> Option<Vec<Direction>> {
    let timer = Instant::now();
    let mut total = 0;
    let mut depth = 0;
    let mut hash_set: HashSet<FttMap> = HashSet::from([ftt.clone()]);
    let mut queue = VecDeque::from([(ftt, vec![])]);
    while let Some((ftt, mut steps)) = queue.pop_front() {
        total += 1;
        if steps.len() > depth {
            depth = steps.len();
            let elapsed = timer.elapsed();
            if debug {
                println!(
                    "depth: {depth} \ttotal: {total} \telapsed: {elapsed:?}"
                );
            }
        }

        for dir in DIRECTIONS {
            let mut ftt = ftt.clone();
            if simulate(&mut ftt, dir) {
                steps.push(dir);
                if ftt.player == ftt.target {
                    let elapsed = timer.elapsed();
                    if debug {
                        println!(
                            "steps: {} \ttotal: {} \telapsed: {:?}",
                            depth + 1,
                            total,
                            elapsed
                        );
                    }
                    return Some(steps.to_vec());
                }
                if steps.len() <= max_steps && !hash_set.contains(&ftt) {
                    hash_set.insert(ftt.clone());
                    queue.push_back((ftt, steps.clone()));
                }
                steps.pop();
            }
        }
    }
    None
}
