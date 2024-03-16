use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    // OUT_DIR로부터 target 디렉토리의 경로를 추출합니다.
    let target_dir = Path::new(&out_dir).ancestors().nth(4).unwrap();

    // 빌드 모드에 따라 정확한 디렉토리를 선택합니다.
    let profile = if cfg!(debug_assertions) { "debug" } else { "release" };
    let target_profile_dir = Path::new(target_dir).join(profile);

    // 'zok' 폴더를 추가합니다.
    let target_zok_dir = target_profile_dir.join("zok");

    // 'zok/issuer' 폴더 경로를 추가합니다.
    let target_issuer_dir = target_zok_dir.join("issuer");

    // 'zok/issuer' 디렉토리가 존재하지 않으면 생성합니다.
    if !target_issuer_dir.exists() {
        fs::create_dir_all(&target_issuer_dir).expect("Failed to create zok/issuer directory");
    }

    let source_files = [
        ("zok/create_hash.zok", &target_zok_dir),
        ("zok/verify_credential.zok", &target_zok_dir),
        ("zok/create_signature.py", &target_issuer_dir), // 이 파일만 다른 위치로 복사됩니다.
    ];

    for (source_file_str, target_dir) in &source_files {
        let source_file = Path::new(source_file_str);
        if let Some(file_name) = source_file.file_name() {
            // 파일을 지정된 디렉토리 내부에 복사합니다.
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
