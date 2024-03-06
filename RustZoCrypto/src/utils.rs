use std::fs::File;
use std::io::{Write, Result};
use hex;
use num_bigint::BigInt;

// babyjubjub.rs와 field.rs 모듈이 이미 정의되어 있다고 가정합니다.
use crate::babyjubjub::{Point, PublicKey};

// `to_bytes` 함수와 비슷한 기능을 수행하는 러스트 함수입니다.
fn to_bytes<T: Into<Vec<u8>>>(data: T) -> Vec<u8> {
    data.into()
}

// 서명, 공개키, 메시지를 받아 ZoKrates CLI 형식으로 파일에 쓰는 함수입니다.
fn write_signature_for_zokrates_cli(pk: &PublicKey, sig: &(Point, BigInt), msg: &[u8], path: &str) -> Result<()> {
    let (sig_r, sig_s) = sig;
    let mut file = File::create(path)?;

    // 서명 값, 공개키, 메시지를 문자열로 변환합니다.
    let args = format!(
        "{} {} {} {} {}",
        sig_r.x, sig_r.y, sig_s,
        pk.p.x.n, pk.p.y.n,
    );
    
    // 메시지를 16진수 문자열로 변환한 뒤, 공백으로 구분된 8자리 10진수 문자열로 변환합니다.
    let msg_hex = hex::encode(msg);
    let m0 = &msg_hex[..64];
    let m1 = &msg_hex[64..];
    let b0: Vec<String> = m0.as_bytes()
                            .chunks(8)
                            .map(|chunk| u64::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16).unwrap().to_string())
                            .collect();
    let b1: Vec<String> = m1.as_bytes()
                            .chunks(8)
                            .map(|chunk| u64::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16).unwrap().to_string())
                            .collect();
    let args = args + " " + &b0.join(" ") + " " + &b1.join(" ");

    // 파일에 쓰기
    file.write_all(args.as_bytes())?;

    Ok(())
}
