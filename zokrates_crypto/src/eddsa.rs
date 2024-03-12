use std::ops::Add;

use sha2::{Sha256, Digest};
use num_bigint::{BigInt, Sign};
use crate::{babyjubjub::{Point, JUBJUB_E, JUBJUB_L}, field::Fq};

// 메시지를 스칼라로 해싱하는 함수
fn hash_to_scalar<R: AsRef<[u8]>>(args: R) -> BigInt {
    let mut hasher = Sha256::new();
    hasher.update(args.as_ref());
    let result = hasher.finalize();
    BigInt::from_bytes_be(Sign::Plus, &result)
}

// PrivateKey와 PublicKey 구조체 정의
#[derive(Debug, Clone, PartialEq)]
pub struct PrivateKey {
    k: Fq,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PublicKey {
    a: Point,
}

impl PrivateKey {
    // 임의의 개인 키 생성
    pub fn new(k: Fq) -> Self {
        Self { k }
    }

    // 개인 키에서 공개 키 생성
    pub fn to_public_key(&self) -> PublicKey {
        PublicKey {
            a: Point::generator().scalar_mul(&self.k.n)
        }
    }

    // 메시지 서명
    pub fn sign(&self, msg: &[u8]) -> (Point, BigInt) {
        let r = hash_to_scalar(&self.k.n.to_bytes_be()) % &*JUBJUB_L;
        let r_point = Point::generator().scalar_mul(&r);
        let t = hash_to_scalar([r_point.to_bytes().as_slice(), self.to_public_key().a.to_bytes().as_slice(), msg].concat());
        let s = (r + (self.k.n * t)) % &*JUBJUB_E;
        (r_point, s)
    }
}

impl PublicKey {
    // 서명 검증
    pub fn verify(&self, sig: &(Point, BigInt), msg: &[u8]) -> bool {
        let (r_point, s) = sig;
        let t = hash_to_scalar([r_point.to_bytes().as_slice(), self.a.to_bytes().as_slice(), msg].concat());

        let lhs = Point::generator().scalar_mul(&s);
        let rhs = r_point.add(self.a.scalar_mul(&t));

        lhs == rhs
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::Fq;
    use rand::{thread_rng, RngCore};

    #[test]
    fn test_sign_verify() {
        // Generate a random message
        let mut rng = thread_rng();
        let mut msg = [0u8; 32];
        rng.fill_bytes(&mut msg);

        // Hardcoded private key for consistent testing
        let key = Fq::new(BigInt::from(1997011358982923168928344992199991480689546837621580239342656433234255379025u128));

        let sk = PrivateKey::new(key);
        let sig = sk.sign(&msg);

        let pk = sk.to_public_key();
        assert!(pk.verify(&sig, &msg), "Signature verification failed");
    }
}
