use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Credential {
    pub claim: Claims,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    // 필드명을 스네이크 케이스로 변경
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    pub id: String,
    #[serde(rename = "type")]
    pub credential_type: Vec<String>,
    pub issuer: CredentialIssuer,
    pub issuance_date: DateTime<Utc>,
    pub credential_subject: CredentialSubject,
    pub exp: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CredentialIssuer {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CredentialSubject {
    pub id: String,
    pub name: String,
    pub age: u8,
    pub student_number: String,
    pub alumni_of: AlumniOf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AlumniOf {
    pub id: String,
    pub name: String,
    pub department: String,
}
