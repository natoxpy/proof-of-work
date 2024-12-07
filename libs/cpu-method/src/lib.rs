use std::time::Instant;

use rayon::prelude::*;

use data_encoding::HEXUPPER;
use ring::digest::{Context, SHA256};

// find if result is less than target
fn verify_nonce(result: &[u8], target: &[u8]) -> bool {
    if result.len() != target.len() {
        return false;
    }

    for (&res_byte, &tgt_byte) in result.iter().zip(target.iter()) {
        match res_byte.cmp(&tgt_byte) {
            std::cmp::Ordering::Greater => return false, // Result is larger, invalid
            std::cmp::Ordering::Less => return true,     // Result is smaller, valid
            std::cmp::Ordering::Equal => continue,       // Keep checking next bytes
        }
    }

    // All bytes are equal, result is valid
    true
}

pub fn solve_challenge_single_thread(prefix: &str, target_hex: &str) -> String {
    let mut nonce = 0;
    let mut hashed;
    let target = HEXUPPER.decode(target_hex.as_bytes()).unwrap();

    loop {
        let mut context = Context::new(&SHA256);
        let input = format!("{}{}", prefix, nonce);
        context.update(input.as_bytes());
        hashed = context.finish().as_ref().to_vec();

        let result = verify_nonce(&hashed, &target);

        if result {
            break;
        } else {
            nonce += 1;
        }
    }

    nonce.to_string()
}

fn batch_256_nonce(dims: &[u32; 2], offset: u32, prefix: &str) -> Vec<Vec<Vec<u8>>> {
    let len = dims[0] * dims[1];

    (0..dims[0])
        .into_par_iter()
        .map(|i| {
            (0..dims[1])
                .into_par_iter()
                .map(|y| {
                    let nonce = y * dims[0] + i + len * offset;

                    let mut context = Context::new(&SHA256);
                    context.update(format!("{}{}", prefix, nonce).as_bytes());
                    context.finish().as_ref().to_vec()
                })
                .collect::<Vec<Vec<u8>>>()
        })
        .collect::<Vec<Vec<Vec<u8>>>>()
}

#[derive(Debug, Clone, Copy)]
pub struct Work {
    pub nonce: u32,
    pub result: bool,
}

pub fn solve_challange_threaded(
    prefix: &str,
    target_hex: &str,
    threads: u32,
    threads_n: u32,
    max_offset: u32,
) -> Vec<Work> {
    let target = HEXUPPER.decode(target_hex.as_bytes()).unwrap();

    let total_len = threads * threads_n;

    for offset in 0..max_offset {
        let p = Instant::now();
        let mut batches = batch_256_nonce(&[threads, threads_n], offset, prefix);
        println!("{:?}", p.elapsed());

        let results = batches
            .par_iter_mut()
            .map(|batch| {
                let mut vals = Vec::new();

                for item in batch {
                    vals.push(verify_nonce(item, &target));
                }

                vals
            })
            .collect::<Vec<Vec<bool>>>();

        let result = results
            .iter()
            .enumerate()
            .flat_map(|(i, work_batch)| {
                work_batch.iter().enumerate().map(move |(y, result)| Work {
                    nonce: (y as u32) * threads + (i as u32) + total_len * offset,
                    result: *result,
                })
            })
            .filter(|work| work.result)
            .collect::<Vec<Work>>();

        if !result.is_empty() {
            return result;
        }
    }

    Vec::new()
}
