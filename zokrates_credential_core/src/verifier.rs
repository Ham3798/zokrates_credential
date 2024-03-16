use std::fs;
use std::process::Command;

pub fn get_proving_key(destination_path: &str) {
    // `proving.key` 파일의 현재 경로
    let source_path = "./zok/verifier/proving.key";
    // `proving.key` 파일을 이동할 목적지 경로
    let destination_path = &format!("{}/proving.key", destination_path);

    // 파일 복사
    fs::copy(source_path, destination_path).expect("Failed to copy proving.key file. Setup First");
    println!("File copied to: {}", destination_path); // 복사된 파일 위치 로깅
}

pub fn setup() {
    // `zokrates compile` 명령어 실행
    let compile_status = Command::new("zokrates")
        .current_dir("./zok/verifier") // 작업 디렉토리 설정
        .arg("compile")
        .arg("-i")
        .arg("verify_credential.zok")
        .status() // 명령어 실행
        .expect("Failed to execute zokrates compile");
    assert!(compile_status.success()); // 컴파일 성공 확인

    // `zokrates setup` 명령어 실행
    let compile_status = Command::new("zokrates")
        .current_dir("./zok/verifier") // 작업 디렉토리 설정
        .arg("setup")
        .status() // 명령어 실행
        .expect("Failed to execute zokrates setup");
    assert!(compile_status.success()); // setup 성공 확인

    // `zokrates export-verifier` 명령어 실행
    // verifier.sol 컨트렉트 생성
    let compile_status = Command::new("zokrates")
        .current_dir("./zok/verifier") // 작업 디렉토리 설정
        .arg("export-verifier")
        .status() // 명령어 실행
        .expect("Failed to execute zokrates setup");
    assert!(compile_status.success()); // setup 성공 확인
}

pub fn get_verify_contract(destination_path: &str) {
    // `verifier.sol` 파일의 현재 경로
    let source_path = "./zok/verifier/verifier.sol";
    // `verifier.sol` 파일을 이동할 목적지 경로
    let destination_path = &format!("{}/verifier.sol", destination_path);

    // 파일 복사
    fs::copy(source_path, destination_path).expect("Failed to copy verifier.sol file.");
    println!("File copied to: {}", destination_path); // 복사된 파일 위치 로깅
}
