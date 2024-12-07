use rayon::iter::{IntoParallelIterator, ParallelIterator};
use ring::digest::SHA256;

pub fn create_hashes(prefix: &str, n: usize, offset: usize) -> Vec<u8> {
    (0..n)
        .into_par_iter()
        .flat_map(|i| {
            let mut context = ring::digest::Context::new(&SHA256);
            context.update(format!("{}{}", prefix, i + offset).as_bytes());
            context.finish().as_ref().to_vec()
        })
        .collect()

    // (0..n)
    //     .into_par_iter()
    //     .map(|i| {
    //         let mut context = ring::digest::Context::new(&SHA256);
    //         context.update(format!("{}{}", prefix, i + offset).as_bytes());
    //         context.finish().as_ref().to_vec()
    //     })
    //     .collect::<Vec<Vec<u8>>>()
    //     .concat()
}
