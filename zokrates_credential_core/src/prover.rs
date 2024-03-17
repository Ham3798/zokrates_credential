use serde_json::{Map, Value};
use std::{
    fs::{self, File},
    io::{self, BufRead},
};

fn load_credential(credential_path: &str) -> Vec<String> {
    // 파일로부터 JSON 데이터 읽기
    let data = fs::read_to_string(credential_path).expect("Unable to read file");

    // 읽은 데이터를 JSON으로 파싱
    let hashes: Map<String, Value> = serde_json::from_str(&data).expect("Unable to parse JSON");

    // JSON 객체에서 value 값만 추출하여 Vec<String>에 담아 리턴
    hashes
        .iter()
        .filter_map(|(_, value)| match value {
            Value::String(val) => Some(val.clone()), // 값이 문자열인 경우
            Value::Number(num) => Some(num.to_string()), // 값이 숫자인 경우, 숫자를 문자열로 변환
            _ => None, // 그 외의 경우는 None을 반환하여 filter_map에서 제외
        })
        .collect()
}

fn load_signature(signature_path: &str) -> Result<Vec<String>, io::Error> {
    // 파일 경로로부터 파일을 엽니다.
    let file = File::open(signature_path)?;
    let reader = io::BufReader::new(file);

    // 파일의 첫 번째 줄을 읽습니다.
    let mut lines = reader.lines();
    if let Some(Ok(line)) = lines.next() {
        // 공백으로 값을 분리하여 Vec<String>에 저장합니다.
        let values = line
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        Ok(values)
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "The file is empty or not accessible",
        ))
    }
}

use std::process::Command;

pub fn setup() {
    // `zokrates compile` 명령어 실행
    let compile_status = Command::new("zokrates")
        .current_dir("./zok/prover") // 작업 디렉토리 설정
        .arg("compile")
        .arg("-i")
        .arg("verify_credential.zok")
        .status() // 명령어 실행
        .expect("Failed to execute zokrates compile");
    assert!(compile_status.success()); // 컴파일 성공 확인
}

fn create_witness_for_verify_credential(credential_path: &str, signature_path: &str) {
    // credential_hash_param load
    let credential_hash_param = load_credential(credential_path);
    let signature_param = load_signature(signature_path).unwrap();

    // 세 가지 파라미터를 하나의 Vec<String>으로 합치기
    let mut param = Vec::new();
    param.extend(credential_hash_param);
    param.extend(signature_param);

    let mut args = vec!["compute-witness".into(), "-a".into()];
    args.extend(param.into_iter());
    args.push("--verbose".into());
    args.push("--json".into());

    let compute_witness_status = Command::new("zokrates")
        .current_dir("./zok/prover") // 작업 디렉토리 설정
        .args(&args)
        .status() // 명령어 실행
        .expect("Failed to execute zokrates compute-witness");

    assert!(compute_witness_status.success()); // compute-witness 성공 확인
}

pub fn create_proof(
    credential_path: &str,
    signature_path: &str,
    proving_key_path: &str,
    destination_path: &str,
) {
    create_witness_for_verify_credential(credential_path, signature_path);

    // `generate-proof` 명령어에 필요한 인자를 새로운 방식으로 준비합니다.
    let args: Vec<String> = vec![
        "generate-proof".into(),
        "--proving-key-path".into(),
        proving_key_path.into(), // 프루빙 키 경로
        "--witness".into(),
        "witness".into(), // compute-witness에서 생성된 증인 파일의 이름
        "--proof-path".into(),
        format!("{}/proof.json", destination_path).into(), // 생성된 증명을 저장할 경로
    ];

    // `zokrates generate-proof` 명령어를 수정된 인자로 실행합니다.
    let generate_proof_status = Command::new("zokrates")
        .current_dir("./zok/prover") // 작업 디렉토리 설정
        .args(&args)
        .status() // 명령어 실행
        .expect("Failed to execute zokrates generate-proof");

    assert!(generate_proof_status.success()); // generate-proof 성공 확인
}
