use lazy_static::lazy_static;
use num_traits::Num;
use std::ops::Add;
use num_bigint::BigInt;
use crate::field::Fq;

lazy_static! {
    static ref JUBJUB_Q: BigInt = BigInt::from_str_radix("21888242871839275222246405745257275088548364400416034343698204186575808495617", 10).unwrap();
    static ref JUBJUB_E: BigInt = BigInt::from_str_radix("21888242871839275222246405745257275088614511777268538073601725287587578984328", 10).unwrap();
    static ref JUBJUB_C: BigInt = BigInt::from(8u32);
    static ref JUBJUB_L: BigInt = (&*JUBJUB_E) / (&*JUBJUB_C);
    static ref JUBJUB_A: BigInt = BigInt::from(168700u32);
    static ref JUBJUB_D: BigInt = BigInt::from(168696u32);
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    x: Fq,
    y: Fq,
}

impl Point {
    fn is_valid(&self) -> bool {
        let lhs = Fq::new(JUBJUB_A.clone()) * self.x.clone() * self.x.clone() + self.y.clone() * self.y.clone();
        let rhs = Fq::one() + Fq::new(JUBJUB_D.clone()) * self.x.clone() * self.x.clone() * self.y.clone() * self.y.clone();
        lhs == rhs
    }

    fn generator() -> Self {
        Self {
            x: Fq::new(BigInt::parse_bytes(b"16540640123574156134436876038791482806971768689494387082833631921987005038935", 10).unwrap()),
            y: Fq::new(BigInt::parse_bytes(b"20819045374670962167435360035096875258406992893633759881276124905556507972311", 10).unwrap()),
        }
    }
    
    fn scalar_mul(&self, scalar: &BigInt) -> Self {
        let mut result = Self::infinity(); // 무한대 포인트로 시작
        let mut base = self.clone();

        let mut scalar_bits = scalar.to_bytes_be().1;
        scalar_bits.reverse();

        for bit in scalar_bits.iter() {
            if *bit == 1 {
                result = result + base.clone();
            }
            base = base.clone() + base.clone();
        }

        result
    }

    fn infinity() -> Self {
        Self {
            x: Fq::zero(),
            y: Fq::one(),
        }
    }
    
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.x == Fq::zero() && self.y == Fq::zero() {
            return other;
        }
        if other.x == Fq::zero() && other.y == Fq::zero() {
            return self;
        }
        let u1 = self.x.clone();
        let v1 = self.y.clone();
        let u2 = other.x;
        let v2 = other.y;

        let u3 = (u1.clone() * v2.clone() + v1.clone() * u2.clone()) / (Fq::one() + Fq{ n: JUBJUB_D.clone() } * u1.clone() * u2.clone() * v1.clone() * v2.clone());
        let v3 = (v1.clone() * v2.clone() - Fq{ n: JUBJUB_A.clone() } * u1.clone() * u2.clone()) / (Fq::one() - Fq{ n: JUBJUB_D.clone() } * u1 * u2 * v1 * v2);
        Self { x: u3, y: v3 }
    }
}
