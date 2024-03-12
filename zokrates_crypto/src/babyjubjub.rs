use lazy_static::lazy_static;
use num_traits::Num;
use std::ops::Add;
use num_bigint::BigInt;
use crate::field::Fq;

lazy_static! {
    pub static ref JUBJUB_Q: BigInt = BigInt::from_str_radix("21888242871839275222246405745257275088548364400416034343698204186575808495617", 10).unwrap();
    pub static ref JUBJUB_E: BigInt = BigInt::from_str_radix("21888242871839275222246405745257275088614511777268538073601725287587578984328", 10).unwrap();
    pub static ref JUBJUB_C: BigInt = BigInt::from(8u32);
    pub static ref JUBJUB_L: BigInt = (&*JUBJUB_E) / (&*JUBJUB_C);
    pub static ref JUBJUB_A: BigInt = BigInt::from(168700u32);
    pub static ref JUBJUB_D: BigInt = BigInt::from(168696u32);
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    x: Fq,
    y: Fq,
}

impl Point {
    pub fn is_valid(&self) -> bool {
        let lhs = Fq::new(JUBJUB_A.clone()) * self.x.clone() * self.x.clone() + self.y.clone() * self.y.clone();
        let rhs = Fq::one() + Fq::new(JUBJUB_D.clone()) * self.x.clone() * self.x.clone() * self.y.clone() * self.y.clone();
        lhs == rhs
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.x.n.to_bytes_be().1);
        bytes.extend_from_slice(&self.y.n.to_bytes_be().1);
        bytes
    }

    pub fn generator() -> Self {
        Self {
            x: Fq::new(BigInt::parse_bytes(b"16540640123574156134436876038791482806971768689494387082833631921987005038935", 10).unwrap()),
            y: Fq::new(BigInt::parse_bytes(b"20819045374670962167435360035096875258406992893633759881276124905556507972311", 10).unwrap()),
        }
    }
    
    pub fn scalar_mul(&self, scalar: &BigInt) -> Self {
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

    pub fn infinity() -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigInt;
    use crate::field::Fq;

    fn point_g() -> Point {
        Point::generator()
    }

    fn point_g_dbl() -> Point {
        Point {
            x: Fq::new(BigInt::parse_bytes(b"17324563846726889236817837922625232543153115346355010501047597319863650987830", 10).unwrap()),
            y: Fq::new(BigInt::parse_bytes(b"20022170825455209233733649024450576091402881793145646502279487074566492066831", 10).unwrap()),
        }
    }

    #[test]
    fn test_double_via_add() {
        let g = point_g();
        let g_dbl = g.clone() + g.clone(); // Assuming Add is properly implemented for Point
        assert_eq!(g_dbl, point_g_dbl());
    }

    #[test]
    fn test_cyclic() {
        let g = point_g();
        let e_plus_one = &*JUBJUB_E + BigInt::from(1);
        assert_eq!(g.scalar_mul(&e_plus_one), g);
    }

    #[test]
    fn test_mult_2() {
        let g = point_g();
        let g_mult2 = g.scalar_mul(&BigInt::from(2));
        assert_eq!(g_mult2, point_g_dbl());
    }

    #[test]
    fn test_lower_order_p() {
        let lp = Point {
            x: Fq::new(BigInt::parse_bytes(b"4342719913949491028786768530115087822524712248835451589697801404893164183326", 10).unwrap()),
            y: Fq::new(BigInt::parse_bytes(b"4826523245007015323400664741523384119579596407052839571721035538011798951543", 10).unwrap()),
        };
        let lp_c = lp.scalar_mul(&*JUBJUB_C);
        assert_eq!(lp_c, Point::infinity());
        let lp_l = lp.scalar_mul(&*JUBJUB_L);
        assert_eq!(lp_l, lp);
    }

    // Additional tests like test_multiplicative, test_associativity, and test_identities
    // would follow a similar pattern, adapting the Python logic to Rust's syntax and your crate's API.
}

