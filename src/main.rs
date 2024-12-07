// use std::time::Instant;
// use cpu_method::solve_challange_threaded;
use gpu_method::test;

#[tokio::main]
async fn main() {
    test().await;
    // let prefix = "VXMwW2qPfW2gkCNSl1i708NJkDghtAyU";
    // let target_hex = "000000FF00000000000000000000000000000000000000000000000000000000";

    // // let time = Instant::now();
    // let mut result = solve_challange_threaded(prefix, target_hex, 16, 65_536, 255);

    // println!("{:?}", time.elapsed());
    // println!("{:#?}", result.pop().unwrap().nonce);
}
