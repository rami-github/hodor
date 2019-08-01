// pub(crate) mod vdf;
// pub(crate) mod cubic_vdf;

use ff::*;

#[derive(PrimeField)]
#[PrimeFieldModulus = "3618502788666131213697322783095070105623107215331596699973092056135872020481"]
#[PrimeFieldGenerator = "7"]
pub struct Fr(FrRepr);