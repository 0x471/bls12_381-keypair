use lambdaworks_math::cyclic_group::IsGroup;
use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::BLS12381Curve;
use lambdaworks_math::elliptic_curve::traits::IsEllipticCurve;
use lambdaworks_math::traits::{AsBytes, ByteConversion};
use lambdaworks_math::unsigned_integer::element::U256;
use std::u64;

fn main() {
    // Great explanation of BLS12-381: https://hackmd.io/@benjaminion/bls12-381#Swapping-G1-and-G2
    // Generating public key from a private key: `public_key = generator_point * private_key`

    const PRIVATE_KEY: u64= 0x6C616D6264617370;

    // Get the Generator point
    let generator_point = BLS12381Curve::generator();

    // Scalar multiplication
    let public_key = generator_point.operate_with_self(PRIVATE_KEY);

    // Big-endian :)
    let public_key_from_bytes =
        U256::from_bytes_be(&public_key.as_bytes()).expect("Failed to convert public_key from bytes to uint256");

    println!("0x{}", public_key_from_bytes.to_hex().to_lowercase());
    // 0xefc2d10ad531cebf2b8c7b4325bc93ed91e6477d260304c1f9ecc7ba0e6f5711
}
