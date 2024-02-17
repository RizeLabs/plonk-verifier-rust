pub mod verifier {
    use ark_bn254::{Bn254, FqParameters, Fr, FrParameters};
    use ark_ec::*;
    use ark_ff::{Field, Fp256, One, PrimeField, UniformRand, Zero};
    // use ark_ff::*;
    pub use crate::utils::utils::{get_plonk_proof, PlonkProof};
    use ark_poly::univariate::DensePolynomial;
    use ark_poly::{domain, Polynomial};
    use std::fmt::{Debug, DebugMap, Display};
    use std::ops::{Add, Mul, Neg, Sub};
    use std::str::FromStr;

    pub fn verify() {

        // challenges
        let aplha: Fp256<FrParameters> = Fr::from_str(
            "20524487875464908209490178628685531130495322118498633336472062372490596458160",
        )
        .unwrap();
        let aplha2: Fp256<FrParameters> = Fr::from_str(
            "15078006696392234695360259740636700679685160725546870868598180534190235322590",
        )
        .unwrap();
        let beta: Fp256<FrParameters> = Fr::from_str(
            "1469297811652786173524431317518899500255817294137003269865683238937785575151",
        )
        .unwrap();
        let betaXi: Fp256<FrParameters> = Fr::from_str(
            "13225259735795124208355754745106974264820190639360930913938372355710361556434",
        )
        .unwrap();
        let gamma: Fp256<FrParameters> = Fr::from_str(
            "18662762454804078530469268494873062022326292981887766436251536958276002157418",
        )
        .unwrap();
        let u: Fp256<FrParameters> = Fr::from_str(
            "3671131478064498243238023262552279287106793140894919933179355516438710425648",
        )
        .unwrap();
        let v1: Fp256<FrParameters> = Fr::from_str(
            "14498287487861080416419858029046690078416135504177055334726844512695965479306",
        )
        .unwrap();
        let v2: Fp256<FrParameters> = Fr::from_str(
            "18486859084993980290861474858117854364521133753017300100785278076947352879482",
        )
        .unwrap();
        let v3: Fp256<FrParameters> = Fr::from_str(
            "14123602248794384244454650572711232922479511827410910736881997840343398040432",
        )
        .unwrap();
        let v4: Fp256<FrParameters> = Fr::from_str(
            "2148331607749528302422858560444633850556901391050132284183052763054829516667",
        )
        .unwrap();
        let v5: Fp256<FrParameters> = Fr::from_str(
            "4136526678804187529711616303688208869122242242984196786246124372892070082407",
        )
        .unwrap();
        let xi: Fp256<FrParameters> = Fr::from_str(
            "2036501310948870752400564319467871188178099508325597424996516092094167193038",
        )
        .unwrap();
        let xin: Fp256<FrParameters> = Fr::from_str(
            "18100393929293372189165175191067012844444248477558768048865905094957039702828",
        )
        .unwrap();
        let zh: Fp256<FrParameters> = Fr::from_str(
            "18100393929293372189165175191067012844444248477558768048865905094957039702827",
        )
        .unwrap();
    
        let n = Fr::from_str("2048").unwrap(); 

        let lagrange = calculateLagrange(n, zh);

        println!("Lagrange {:?}", lagrange);
        

        let proof: PlonkProof = get_plonk_proof();
        print!("{:?}", proof);
    }

    pub fn calculateLagrange(n: Fp256<FrParameters> , zh: Fp256<FrParameters>) -> Fp256<FrParameters> {
        let w = Fr::one();

        let denom = n * (zh - w);
        let domain: u64 = 2048;
        let numerator = w * (zh.pow([domain]) - w);
        let lagrange = numerator.mul(denom.inverse().unwrap());

        lagrange
    }
}
