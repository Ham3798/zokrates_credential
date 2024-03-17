extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::ops::{Add, Div, Mul, Neg, Sub};

const FIELD_MODULUS: &str =
    "21888242871839275222246405745257275088548364400416034343698204186575808495617";

fn inv(a: &BigInt) -> BigInt {
    a.modpow(
        &(-BigInt::one()),
        &BigInt::parse_bytes(FIELD_MODULUS.as_bytes(), 10).unwrap(),
    )
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fq {
    pub n: BigInt,
}

impl Fq {
    pub fn new<T: Into<BigInt>>(val: T) -> Fq {
        let modulus = BigInt::parse_bytes(FIELD_MODULUS.as_bytes(), 10).unwrap();
        let n = val.into() % &modulus;
        Fq { n }
    }

    pub fn one() -> Fq {
        Fq::new(BigInt::one())
    }

    pub fn zero() -> Fq {
        Fq::new(BigInt::zero())
    }
}

// 연산자 오버로딩을 위한 구현들입니다.
impl Add for Fq {
    type Output = Fq;

    fn add(self, other: Fq) -> Fq {
        Fq::new(self.n + other.n)
    }
}

impl Sub for Fq {
    type Output = Fq;

    fn sub(self, other: Fq) -> Fq {
        Fq::new(self.n - other.n)
    }
}

impl Mul for Fq {
    type Output = Fq;

    fn mul(self, other: Fq) -> Fq {
        Fq::new(self.n * other.n)
    }
}

impl Div for Fq {
    type Output = Fq;

    fn div(self, other: Fq) -> Fq {
        Fq::new(self.n * inv(&other.n))
    }
}

impl Neg for Fq {
    type Output = Fq;

    fn neg(self) -> Self::Output {
        let modulus = BigInt::parse_bytes(FIELD_MODULUS.as_bytes(), 10).unwrap();
        let neg_n = (modulus.clone() - self.n) % &modulus;
        Fq::new(neg_n)
    }
}
