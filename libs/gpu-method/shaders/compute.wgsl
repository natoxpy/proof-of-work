@group(0) @binding(0) var<storage, read_write> data: array<atomic<u32>>; // Output data
@group(0) @binding(1) var<storage, read_write> context: array<u32>; // Output data

// [0]: u32 size of bytes given for hash and target data
// [1]: u32 how many hashes per thread
@group(0) @binding(2) var<storage, read_write> metadata: array<u32>; // input data
@group(0) @binding(3) var<storage, read_write> hash_data: array<u32>; // input data
@group(0) @binding(4) var<storage, read_write> target_data: array<u32>; // input data

const block_x: u32 = 1u;
const block_y: u32 = 1u;
const block_z: u32 = 1u;

const numThreadsPerWorkgroup = block_x * block_y * block_z;

// little indian to big indian 
fn le_to_be(value: u32) -> u32 {
    let byte0 = value & 0xFF;
    let byte1 = (value >> 8) & 0xFF;
    let byte2 = (value >> 16) & 0xFF;
    let byte3 = (value >> 24) & 0xFF;

    return byte0 << 24 | byte1 << 16 | byte2 << 8 | byte3;
}

// check if hash < target
fn validate(bite_index: u32) -> bool {
    let offset_index = bite_index * (metadata[0] / 4u);

    for (var i = 0u; i < metadata[0] / 4u; i++) {
        if le_to_be(hash_data[offset_index + i]) > le_to_be(target_data[i]) {
            return false;
        } else if le_to_be(hash_data[offset_index + i]) < le_to_be(target_data[i]) {
            return true;
        } else {
            break;
        }
    }

    return true;
}

@compute @workgroup_size(block_x, block_y, block_z)
fn main(
    @builtin(local_invocation_index) local_invocation_index: u32,
    @builtin(workgroup_id) workgroup_id: vec3<u32>,
    @builtin(num_workgroups) num_workgroups: vec3<u32>,
) {
    let total_hashes_todo = metadata[1];
    let total_workgroups = num_workgroups.x * num_workgroups.y * num_workgroups.z;
    let workgroup_index = workgroup_id.x + workgroup_id.y * num_workgroups.x + workgroup_id.z * num_workgroups.x * num_workgroups.y;
    let index = workgroup_index * numThreadsPerWorkgroup + local_invocation_index;

    for (var i = 0u; i < total_hashes_todo; i++) {
        let y = index + total_workgroups * i;
        let bite_index = y / 32u;
        let bit = 1u << (y % 32u);

        if validate(y) {
            atomicOr(&data[bite_index], bit); // Use a pointer to data[word_index]
        }

        if validate(y) {
            context[y] = 10u;
        }
    }
}

