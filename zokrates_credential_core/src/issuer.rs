#[cfg(test)]
mod tests {
    use crate::credential::{AlumniOf, Claims, CredentialIssuer, CredentialSubject};
    use chrono::{DateTime, Duration, Utc};
    use serde_json::{json, to_string_pretty, to_value, Map, Value};
    use sha2::{Digest, Sha256};
    use std::{
        fmt::format, fs::{self, File}, io::{BufReader, Write}, path::Path
    };

    fn create_credential(credential_id: &str, name: &str, age: u8, student_number: &str, department: &str) {
        let credential = Claims {
            context: vec![
                "https://www.w3.org/2018/credentials/v1".to_owned(),
                "https://www.example.org/examples/v1".to_owned(),
            ],
            id: format!("http://chungnam.ac.kr/credentials/{}", credential_id).to_owned(),
            credential_type: vec![
                "VerifiableCredential".to_owned(),
                "AlumniCredential".to_owned(),
            ],
            issuer: CredentialIssuer {
                id: "https://infosec.chungnam.ac.kr".to_owned(),
                name: "Chungnam National University Information Security Lab".to_owned(),
            },
            issuance_date: Utc::now(),
            credential_subject: CredentialSubject {
                id: "did:example:abcdef1234567890".to_owned(),
                name: name.to_owned(),
                age,
                student_number: student_number.to_owned(),
                alumni_of: AlumniOf {
                    id: "did:example:c34fb4561237890".to_owned(),
                    name: "Chungnam National University".to_owned(),
                    department: department.to_owned(),
                },
            },
            exp: (Utc::now() + Duration::days(90)).timestamp(),
        };

        let credential_subject = CredentialSubject {
            id: "did:example:abcdef1234567890".to_owned(),
            name: name.to_owned(),
            age,
            student_number: student_number.to_owned(),
            alumni_of: AlumniOf {
                id: "did:example:c34fb4561237890".to_owned(),
                name: "Chungnam National University".to_owned(),
                department: department.to_owned(),
            },
        };

        let credential_serialized = to_value(&credential).expect("Failed to serialize credential");
        let credential_subject_serialized =
            to_value(&credential_subject).expect("Failed to serialize credential");

        let mut hashes = serde_json::Map::new(); // JSON 객체를 위한 맵 생성

        if let Value::Object(obj) = credential_serialized {
            for (key, value) in obj.iter() {
                let value_str = value.to_string();
                // 값에 대한 SHA-256 해시 계산
                let mut hasher = Sha256::new();
                hasher.update(value_str.as_bytes());
                let hashed = hasher.finalize();
                // 바이트 배열을 16진수 문자열로 변환
                let hashed_decimal_str = hashed
                    .iter()
                    .flat_map(|&byte| format!("{}", byte).chars().collect::<Vec<_>>()) // 각 바이트를 10진수 문자열로 변환하고, 문자들을 flat_map을 사용해 하나의 이터레이터로 만듭니다.
                    .filter(|&c| c.is_digit(10)) // 0부터 9까지의 숫자인지 확인합니다.
                    .take(38) // 최초 38개의 문자만 선택합니다.
                    .collect::<String>(); // 필터링된 문자들을 문자열로 합칩니다.

                // 키와 해시된 값을 맵에 추가
                hashes.insert(key.clone(), json!(hashed_decimal_str));
            }
        }

        // `serialized`가 객체인 경우, 그 키와 값을 순회
        if let Value::Object(obj) = credential_subject_serialized {
            for (key, value) in obj.iter() {
                match value {
                    Value::Number(num) if num.is_i64() => {
                        // 숫자가 정수형일 경우, 직접 값을 추가
                        hashes.insert(key.clone(), json!(num));
                    }
                    _ => {
                        // 정수형이 아닌 경우, 값을 문자열로 변환하고 해시화
                        let value_str = value.to_string();
                        let mut hasher = Sha256::new();
                        hasher.update(value_str.as_bytes());
                        let hashed = hasher.finalize();
                        let hashed_decimal_str = hashed
                            .iter()
                            .flat_map(|&byte| format!("{}", byte).chars().collect::<Vec<_>>()) // 각 바이트를 10진수 문자열로 변환하고, 문자들을 flat_map을 사용해 하나의 이터레이터로 만듭니다.
                            .filter(|&c| c.is_digit(10)) // 0부터 9까지의 숫자인지 확인합니다.
                            .take(38) // 최초 38개의 문자만 선택합니다.
                            .collect::<String>(); // 필터링된 문자들을 문자열로 합칩니다.

                        // 키와 해시된 값을 맵에 추가
                        hashes.insert(key.clone(), json!(hashed_decimal_str));
                    }
                }
            }
        }

        // JSON 객체를 문자열로 변환
        let json_hashes = serde_json::to_string(&hashes).expect("Failed to serialize hashes");
        println!("{}", json_hashes);

        // JSON 객체를 문자열로 변환
        let json_hashes = serde_json::to_string(&hashes).expect("Failed to serialize hashes");
        println!("{}", json_hashes);

        // JSON 객체를 문자열로 변환 (가독성을 위해 예쁘게 인쇄)
        let json_hashes_pretty = to_string_pretty(&hashes).expect("Failed to serialize hashes");

        // 디렉토리 생성
        let dir_path = format!("./zok/issuer/{}", credential_id);
        fs::create_dir_all(&dir_path).expect("Failed to create directory");

        // 파일에 JSON 데이터 쓰기
        let file_path = format!("./zok/issuer/{}/credential.json", credential_id);
        let mut file = File::create(file_path).expect("Failed to create file");
        file.write_all(json_hashes_pretty.as_bytes())
            .expect("Failed to write to file");
    }

    fn load_credential_hash(credential_id: &str) -> Vec<String> {
        // 파일로부터 JSON 데이터 읽기
        let file_path = format!("./zok/issuer/{}/credential.json", credential_id);
        let data = fs::read_to_string(file_path).expect("Unable to read file");

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

    use std::process::Command;

    fn compile_create_hash_zok() {
        // `zokrates compile` 명령어 실행
        let compile_status = Command::new("zokrates")
            .current_dir("./zok/issuer") // 작업 디렉토리 설정
            .arg("compile")
            .arg("-i")
            .arg("create_hash.zok")
            .status() // 명령어 실행
            .expect("Failed to execute zokrates compile");
        assert!(compile_status.success()); // 컴파일 성공 확인

    }

    fn create_witness_for_eddsa_signature_memo(credential_id: &str) {
        // credential_hash_param load
        let credential_hash_param = load_credential_hash(credential_id);
        let mut args = vec!["compute-witness".into(), "-a".into()];
        args.extend(credential_hash_param.into_iter());
        args.push("--verbose".into());
        args.push("--json".into());

        print!("args : {:?}", args);

        let compute_witness_status = Command::new("zokrates")
            .current_dir("./zok/issuer") // 작업 디렉토리 설정
            .args(&args)
            .status() // 명령어 실행
            .expect("Failed to execute zokrates compute-witness");

        assert!(compute_witness_status.success()); // compute-witness 성공 확인
    }

    fn load_zokrates_witness() -> Vec<String> {
        let path = Path::new("./zok/issuer/witness.json");
        // 파일을 열고 BufReader에 바인딩합니다.
        let file = File::open(path).expect("Failed to open file");
        let reader = BufReader::new(file);

        // JSON 데이터를 파싱합니다.
        let v: Map<String, Value> = serde_json::from_reader(reader).unwrap();

        // 특정 키(`~out_0` 부터 `~out_15`)에 해당하는 값들을 순서대로 추출합니다.
        let mut values = Vec::new();

        for i in 0..16 {
            let key = format!("~out_{}", i);
            if let Some(Value::String(val)) = v.get(&key) {
                values.push(val.clone());
            }
        }
        values
    }

    fn create_signature_and_pk(credential_id: &str) {
        // witness 값 로드
        let witness_values = load_zokrates_witness();

        // witness 값 JSON 파일로 저장 (예: witness_values.json)
        let witness_file_path = "./zok/issuer/witness_values.json";
        let mut file = File::create(witness_file_path).expect("Unable to create witness file");
        let witness_json =
            serde_json::to_string(&witness_values).expect("Unable to serialize witness values");
        writeln!(file, "{}", witness_json).expect("Unable to write witness values");

        // Python 스크립트 실행
        // signature 및 publi key 생성 스크립트.
        // 현재는 기존 조크라테스 크립토를 그대로 사용하나, 추후 조크라테스 크립토를 RustZoCrypto로 마이그레이션 후 대체할 예정.
        let output = Command::new("python3")
            .current_dir("./zok/issuer")
            .arg("create_signature.py")
            .output()
            .expect("Failed to execute python script");

        // 실행 성공 여부 확인
        assert!(output.status.success(), "Python script execution failed");

        Issuance_to_prover(credential_id);
    }

    // 파일 이동을 위한 함수
    fn move_file_to_credential_id_folder(credential_id: &str, file_name: &str) {
        let source_path = format!("./zok/issuer/{}", file_name);
        let destination_path = format!("./zok/issuer/{}/{}", credential_id, file_name);

        fs::rename(&source_path, &destination_path)
            .expect(&format!("Failed to move {} to ./zok/prover", file_name));
    }

    fn Issuance_to_prover(credential_id: &str) {
        move_file_to_credential_id_folder(credential_id, "signature");
        move_file_to_credential_id_folder(credential_id, "credential.json");
    }

    fn create_signature(credential_id: &str) {
        create_witness_for_eddsa_signature_memo(credential_id);
        create_signature_and_pk(credential_id);
        Issuance_to_prover(credential_id);
    }

    #[test]
    fn create_credential_test() {
        create_credential("3732", "Socrates", 30, "201902769", "Information Security");

    }

    #[test]
    fn compile_create_hash_zok_test() {
        compile_create_hash_zok();
    }

    #[test]
    fn create_signature_test() {
        create_signature("3732");
    }
   
    
}
