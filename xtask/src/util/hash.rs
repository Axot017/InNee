use std::path::Path;

use sha3::Digest;

pub fn are_files_the_same(first: &Path, second: &Path) -> bool {
    if !(first.is_file() && second.is_file()) {
        return false;
    }

    let first_hash = hash_file(first);
    let second_hash = hash_file(second);

    first_hash == second_hash
}

fn hash_file(path: &Path) -> Vec<u8> {
    let mut file = std::fs::File::open(path).unwrap();
    let mut hasher = sha3::Sha3_256::new();
    std::io::copy(&mut file, &mut hasher).unwrap();

    hasher.finalize().to_vec()
}
