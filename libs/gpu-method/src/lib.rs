use buffer::ComputeBuffer;
use comptu::*;
use data_encoding::HEXUPPER;
use hashes::create_hashes;
use load_shader::load_shader;
use std::{sync::Arc, time::Instant};
mod hashes;
mod load_shader;

pub fn get_nonce(data: Vec<u8>, offset: u32) -> Option<u32> {
    for (x, n) in data.iter().copied().enumerate() {
        if n == 0 {
            continue;
        }

        let bits: u32 = (0..7)
            .map(|bit| u8::pow(2, bit) & n)
            .filter(|x| x > &0)
            .map(|m| u8::ilog(m, 2))
            .collect::<Vec<u32>>()
            .first()
            .copied()
            .unwrap();

        // println!(
        //     "{} : {} : {:?}",
        //     offset,
        //     n,
        //     bits // (0..7).map(|bit| u8::pow(2, bit) & n).collect::<Vec<u8>>()
        // );

        return Some((x as u32) * 8 + offset + bits);
    }

    None
}

pub async fn test() {
    let prefix = "VXMwW2qPfW2gkCNSl1i708NJkDghtAyU";
    let target_hex = "000000FF00000000000000000000000000000000000000000000000000000000";
    let target = HEXUPPER.decode(target_hex.as_bytes()).unwrap();

    // let hash = doo(prefix, 67661163);
    // let offset = 67661123 + 3;
    let workgroups = [8, 8, 8];
    let work_per_thread = 1; // number hashes responsable for single thread

    let dim_len = workgroups[0] * workgroups[1] * workgroups[2] * work_per_thread;

    let attemps = 1;
    let time = Instant::now();

    let context = Arc::new(Context::new().await);
    let shader = Arc::new(load_shader(
        "./libs/gpu-method/shaders/compute.wgsl",
        &context,
    ));

    //67_661_164

    for atps in 0..attemps {
        let it = Instant::now();
        let offset = 67661163 - 30 + atps * dim_len;

        let hashes_t = Instant::now();

        let hashes = create_hashes(prefix, dim_len, offset);

        println!("hashes {:?} : {}", hashes_t.elapsed(), offset);

        let bt = Instant::now();
        // 32 bits
        let output_buffer = StorageBinding::new(&context, ComputeBuffer::from(vec![0; 1]), true);
        let context_buffer =
            StorageBinding::new(&context, ComputeBuffer::from(vec![0; dim_len]), true);

        // [0]: hash vec<u8> length
        let metadata = StorageBinding::new(
            &context,
            ComputeBuffer::from(vec![32, work_per_thread as u32]),
            false,
        );
        let input_hash = StorageBinding::new(&context, ComputeBuffer::new(hashes), false);
        let input_target = StorageBinding::new(&context, ComputeBuffer::new(target.clone()), false);

        println!("storage binding {:?}", bt.elapsed());

        let compute = ComputeContext {
            compute_module: shader.clone(),
            storage_bindings: vec![metadata, input_hash, input_target],
            write_bindings: vec![output_buffer, context_buffer],
            workgroups: [
                workgroups[0] as u32,
                workgroups[1] as u32,
                workgroups[2] as u32,
            ],
        };

        let t = Instant::now();
        let output = compute.compute(context.clone()).await;

        println!("compute {:?}", t.elapsed());

        println!("gpu {:?}", output.first().unwrap().data.clone());
        let nonce = get_nonce(output.first().unwrap().data.clone(), offset as u32);

        println!("nonce {:?} : {}", nonce, offset);
        println!("loop {:?}\n", it.elapsed());

        println!(
            "context {:?}",
            Vec::<u32>::from(output.get(1).unwrap().clone())
        );

        if nonce.is_some() {
            break;
        }
    }

    println!("{:?}", time.elapsed());
}

/*
pub async fn test() {
    let prefix = "VXMwW2qPfW2gkCNSl1i708NJkDghtAyU";
    let target_hex = "000000FF00000000000000000000000000000000000000000000000000000000";
    let target = HEXUPPER.decode(target_hex.as_bytes()).unwrap();

    // let hash = doo(prefix, 67661163);

    // let offset = 67661123 + 3;
    let dimension = [255, 128, 128];
    let dim_len = dimension[0] * dimension[1] * dimension[2];
    let attemps = 15;
    let time = Instant::now();

    for atps in attemps..attemps {
        let offset = atps * dim_len + 1;
        let context = Context::new().await;
        let shader = load_shader("./libs/gpu-method/shaders/compute.wgsl", &context);
        let hashes = create_hashes(prefix, dim_len, offset)
            .iter()
            .flatten()
            .copied()
            .collect();

        // 32 bits
        let output_buffer =
            StorageBinding::new(&context, ComputeBuffer::from(vec![0; dim_len / 32]), true);
        let context_buffer =
            StorageBinding::new(&context, ComputeBuffer::from(vec![0; dim_len]), true);

        // [0]: hash vec<u8> length
        let metadata = StorageBinding::new(&context, ComputeBuffer::from(vec![32, 16]), false);
        let input_hash = StorageBinding::new(&context, ComputeBuffer::new(hashes), false);
        let input_target = StorageBinding::new(&context, ComputeBuffer::new(target.clone()), false);

        let compute = ComputeContext {
            compute_module: shader,
            storage_bindings: vec![metadata, input_hash, input_target],
            write_bindings: vec![output_buffer, context_buffer],
            workgroups: [
                dimension[0] as u32,
                dimension[1] as u32,
                dimension[2] as u32,
            ],
        };

        let output = compute.compute(Arc::new(context)).await;

        println!(
            "context {:?}",
            Vec::<u32>::from(output.get(1).unwrap().clone())
        );

        // let nonce = get_nonce(output.first().unwrap().data.clone(), offset as u32);
        // println!("nonce {:?}:{}", nonce, offset);

        // if nonce.is_some() {
        //     break;
        // }
    }

    println!("{:?}", time.elapsed());

    // println!(
    //     "gpu {:?}",
    //     Vec::<u32>::from(output.first().unwrap().clone())
    // );

    // // println!("output {:?}", &output.first().unwrap().data);
    // println!(
    //     "context {:?}",
    //     Vec::<u32>::from(output.get(1).unwrap().clone())
    // );
}
*/
