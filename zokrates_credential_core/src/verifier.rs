#[cfg(test)]
mod tests {
    use std::fs;
    use std::process::Command;

    fn move_proving_key() {
        // `proving.key` 파일의 현재 경로
        let source_path = "./zok/verifier/proving.key";
        // `proving.key` 파일을 이동할 목적지 경로
        let destination_path = "./zok/prover/proving.key";

        // 파일 이동
        fs::rename(source_path, destination_path).expect("Failed to move proving.key file");
    }

    #[test]
    fn zokrates_setup() {
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

        // `proving.key` 파일을 prover에게 전달
        move_proving_key();
    }

    #[test]
    fn zokrates_create_verify_contract() {
        // `zokrates export-verifier` 명령어 실행
        // verifier.sol 컨트렉트 생성
        let compile_status = Command::new("zokrates")
            .current_dir("./zok/verifier") // 작업 디렉토리 설정
            .arg("export-verifier")
            .status() // 명령어 실행
            .expect("Failed to execute zokrates setup");
        assert!(compile_status.success()); // setup 성공 확인
    }
}
