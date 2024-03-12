use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    // 필드명을 스네이크 케이스로 변경
    #[serde(rename = "@context")]
    context: Vec<String>,
    id: String,
    #[serde(rename = "type")]
    credential_type: Vec<String>,
    issuer: CredentialIssuer,
    issuance_date: DateTime<Utc>,          // 변경됨
    credential_subject: CredentialSubject, // 변경됨
    exp: i64,                              // `exp` 클레임 추가
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CredentialIssuer {
    id: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CredentialSubject {
    id: String,
    name: String,
    age: u8,
    student_number: String,
    alumni_of: AlumniOf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AlumniOf {
    id: String,
    name: String,
    department: String,
}

pub fn credential_sample() -> Claims {
    let my_claims = Claims {
        context: vec![
            "https://www.w3.org/2018/credentials/v1".to_owned(),
            "https://www.example.org/examples/v1".to_owned(),
        ],
        id: "http://chungnam.ac.kr/credentials/3732".to_owned(),
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
            name: "Socrates".to_owned(),
            age: 30,
            student_number: "201812345".to_owned(),
            alumni_of: AlumniOf {
                id: "did:example:c34fb4561237890".to_owned(),
                name: "Chungnam National University".to_owned(),
                department: "Information Security".to_owned(),
            },
        },
        exp: (Utc::now() + Duration::days(90)).timestamp(), // 90일 후 만료
    };
    my_claims
}

pub fn credentialSubject_sample() -> CredentialSubject {
    let my_claims = CredentialSubject {
        id: "did:example:abcdef1234567890".to_owned(),
        name: "Socrates".to_owned(),
        age: 30,
        student_number: "201812345".to_owned(),
        alumni_of: AlumniOf {
            id: "did:example:c34fb4561237890".to_owned(),
            name: "Chungnam National University".to_owned(),
            department: "Information Security".to_owned(),
        },
    };
    my_claims
}
