use ark_ec::*;
use ark_ff::*;
use ark_bn254::{Bn254, Fr};
use ark_poly::univariate::DensePolynomial;
use ark_poly::Polynomial;
use std::fmt::{Debug, Display};
use std::ops::{Add, Mul, Neg, Sub};

fn main() {
    println!("Hello, world!");



    // function batch_evaluate_lagrange_poly_out_of_domain(
    //     uint256[] memory poly_nums, 
    //     uint256 domain_size, 
    //     PairingsBn254.Fr memory omega, 
    //     PairingsBn254.Fr memory at
    // ) internal view returns (PairingsBn254.Fr[] memory res) {
    //     PairingsBn254.Fr memory one = PairingsBn254.new_fr(1);
    //     PairingsBn254.Fr memory tmp_1 = PairingsBn254.new_fr(0);
    //     PairingsBn254.Fr memory tmp_2 = PairingsBn254.new_fr(domain_size);
    //     PairingsBn254.Fr memory vanishing_at_z = at.pow(domain_size);
    //     vanishing_at_z.sub_assign(one);
    //     // we can not have random point z be in domain
    //     require(vanishing_at_z.value != 0);
    //     PairingsBn254.Fr[] memory nums = new PairingsBn254.Fr[](poly_nums.length);
    //     PairingsBn254.Fr[] memory dens = new PairingsBn254.Fr[](poly_nums.length);
    //     // numerators in a form omega^i * (z^n - 1)
    //     // denoms in a form (z - omega^i) * N
    //     for (uint i = 0; i < poly_nums.length; i++) {
    //         tmp_1 = omega.pow(poly_nums[i]); // power of omega
    //         nums[i].assign(vanishing_at_z);
    //         nums[i].mul_assign(tmp_1);
            
    //         dens[i].assign(at); // (X - omega^i) * N
    //         dens[i].sub_assign(tmp_1); 
    //         dens[i].mul_assign(tmp_2); // mul by domain size
    //     }
        
    //     PairingsBn254.Fr[] memory partial_products = new PairingsBn254.Fr[](poly_nums.length);
    //     partial_products[0].assign(PairingsBn254.new_fr(1));
    //     for (uint i = 1; i < dens.length; i++) {
    //         partial_products[i].assign(dens[i-1]);
    //         partial_products[i].mul_assign(partial_products[i-1]);
    //     }
    
    //     tmp_2.assign(partial_products[partial_products.length - 1]);
    //     tmp_2.mul_assign(dens[dens.length - 1]);
    //     tmp_2 = tmp_2.inverse(); // tmp_2 contains a^-1 * b^-1 (with! the last one)
        
    //     for (uint i = dens.length - 1; i < dens.length; i--) {
    //         tmp_1.assign(tmp_2); // all inversed
    //         tmp_1.mul_assign(partial_products[i]); // clear lowest terms
    //         tmp_2.mul_assign(dens[i]);
    //         dens[i].assign(tmp_1);
    //     }
        
    //     for (uint i = 0; i < nums.length; i++) {
    //         nums[i].mul_assign(dens[i]);
    //     }

    //     return nums;
    // }

    let poly_nums = vec![1, 2, 3, 4, 5];
    let domain_size = 10;
    let omega = Fr::from(2u64);
    let at = Fr::from(3u64);

    let one = Fr::one();
    let mut tmp_1 = Fr::zero();
    let mut tmp_2 = Fr::from(domain_size);
    let vanishing_at_z = at.pow(domain_size);
    vanishing_at_z.sub_assign(&one);

    let mut nums = vec![Fr::zero(); poly_nums.len()];
    let mut dens = vec![Fr::zero(); poly_nums.len()];
    for i in 0..poly_nums.len() {
        tmp_1 = omega.pow([poly_nums[i]]);
        nums[i] = vanishing_at_z.clone();
        nums[i].mul_assign(&tmp_1);
        
        dens[i] = at.clone();
        dens[i].sub_assign(&tmp_1);
        dens[i].mul_assign(&tmp_2);
    }

    let mut partial_products = vec![Fr::zero(); poly_nums.len()];
    partial_products[0] = Fr::one();
    for i in 1..dens.len() {
        partial_products[i] = dens[i-1].clone();
        partial_products[i].mul_assign(&partial_products[i-1]);
    }

    tmp_2 = partial_products[partial_products.len() - 1].clone();
    tmp_2.mul_assign(&dens[dens.len() - 1]);
    tmp_2 = tmp_2.inverse().unwrap();

    for i in (0..dens.len()).rev() {
        tmp_1 = tmp_2.clone();
        tmp_1.mul_assign(&partial_products[i]);
        tmp_2.mul_assign(&dens[i]);
        dens[i] = tmp_1;
    }

    for i in 0..nums.len() {
        nums[i].mul_assign(&dens[i]);
    }

    println!("{:?}", nums);

}
