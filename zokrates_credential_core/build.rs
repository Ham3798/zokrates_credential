use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir).ancestors().nth(4).unwrap();
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    let target_profile_dir = Path::new(target_dir).join(profile);
    let target_zok_dir = target_profile_dir.join("zok");
    let target_issuer_dir = target_zok_dir.join("issuer");
    let target_prover_dir = target_zok_dir.join("prover");
    let target_verifier_dir = target_zok_dir.join("verifier");

    // 모든 필요한 디렉토리를 생성합니다.
    for dir in &[&target_issuer_dir, &target_prover_dir, &target_verifier_dir] {
        if !dir.exists() {
            fs::create_dir_all(dir).expect(&format!("Failed to create directory: {:?}", dir));
        }
    }

    let source_files = [
        ("zok/create_hash.zok", &target_zok_dir),
        ("zok/verify_credential.zok", &target_prover_dir),
        ("zok/verify_credential.zok", &target_verifier_dir),
        ("zok/create_signature.py", &target_issuer_dir),
    ];

    for (source_file_str, target_dir) in &source_files {
        let source_file = Path::new(source_file_str);
        if let Some(file_name) = source_file.file_name() {
            let target_file = target_dir.join(file_name);
            println!("Trying to copy from: {:?}", source_file);
            println!("Target file path: {:?}", target_file);

            match fs::copy(source_file, &target_file) {
                Ok(_) => println!("Successfully copied {:?} to {:?}", source_file, target_file),
                Err(e) => panic!("Failed to copy file: {:?}", e),
            }
        } else {
            panic!("Invalid source file path: {:?}", source_file);
        }
    }
}
