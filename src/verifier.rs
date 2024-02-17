pub mod verifier {
    use ark_bn254::{Bn254, FqParameters, Fr, FrParameters, G1Projective, g1::Parameters};
    use ark_ec::short_weierstrass_jacobian::GroupAffine;
    use ark_ec::*;
    use ark_ff::{Field, Fp256, One, PrimeField, UniformRand, Zero};
    // use ark_ff::*;
    pub use crate::utils::utils::{get_plonk_proof, PlonkProof, KzgCommitment};
    use ark_poly::univariate::DensePolynomial;
    use ark_poly::{domain, Polynomial};
    use std::fmt::{Debug, DebugMap, Display};
    use std::ops::{Add, Mul, Neg, Sub};
    use std::str::FromStr;
    use num_bigint::*;

    pub type G1Point = <Bn254 as PairingEngine>::G1Affine;
    pub type G2Point = <Bn254 as PairingEngine>::G2Affine;

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

        // let pi: Fp256<FrParameters> = Fr::from_str(
        //     "10021071990350671093045688305445916367264617343457315103161905320545395462791",
        // )
        // .unwrap();
    
        let n = Fr::from_str("2048").unwrap(); 

        let lagrange = calculateLagrange(n, xi);

        println!("Lagrange {:?}", lagrange);
        

        let proof: PlonkProof = get_plonk_proof();

        let pi = calculate_pi(lagrange, proof);

        let r0 = calcualteR0(aplha, aplha2, beta, gamma, proof, lagrange, pi);

        let f = calculate_f(proof, v1, v2, v3, v4, v5);
        println!("f x{:?}", f.x.to_string());
        println!("f y{:?}", f.y.to_string());

        print!("final r0 {}", r0.to_string());
        // print!("{:?}", proof);
    }

    fn calculate_pi(lagrange: Fp256<FrParameters>, proof: PlonkProof) -> Fp256<FrParameters> {
        let PlonkProof {
            pi: pub_input,
            ..
        } = proof;

        let pi_input = Fr::zero();

        let pi = pi_input.sub(lagrange.mul(pub_input));
        // println!("pi {:?}", pi.to_string());
        pi
    }

    pub fn calculate_f(proof: PlonkProof, v1: Fp256<FrParameters>, v2: Fp256<FrParameters>, v3: Fp256<FrParameters>, v4: Fp256<FrParameters>, v5: Fp256<FrParameters>) -> GroupAffine<Parameters> {
        let PlonkProof {
            a: a,
            b: b,
            c: c,
            ..
        } = proof;

        let a_affine = a.0;
        let b_affine = b.0;
        let c_affine = c.0;

        let s_x_1 = <G1Point as AffineCurve>::BaseField::from_str("2277685636083563024253879452693986130212942936235758785876153581019640880319").unwrap();
        let s_y_1 = <G1Point as AffineCurve>::BaseField::from_str("5558146521438681597961812116362946523808729442181555954974750217085655765563").unwrap();
        
        let s1_affine = G1Projective::new(s_x_1, s_y_1, <G1Projective as ProjectiveCurve>::BaseField::one()).into_affine();

        let s_x_2 = <G1Point as AffineCurve>::BaseField::from_str("21421714290183048746230047877229262977674171892814788767166398067614207270732").unwrap();
        let s_y_2 = <G1Point as AffineCurve>::BaseField::from_str("18351947949312641279139525707675648861898823980801914700748293475468468405778").unwrap();
        
        let s2_affine = G1Projective::new(s_x_2, s_y_2, <G1Projective as ProjectiveCurve>::BaseField::one()).into_affine();

        let d_x = <G1Point as AffineCurve>::BaseField::from_str("12666435046384658404409583525473450230172226690730467901543922284322472961475").unwrap();
        let d_y = <G1Point as AffineCurve>::BaseField::from_str("13658003628221264547878886863410062940256997495045129927343096614540833534333").unwrap();
        
        let d_affine = G1Projective::new(d_x, d_y, <G1Projective as ProjectiveCurve>::BaseField::one()).into_affine();


        let in_complete_f = a_affine.mul(v1).add(b_affine.mul(v2).add(c_affine.mul(v3).add(s1_affine.mul(v4).add(s2_affine.mul(v5))))).into_affine();        

        d_affine.add(in_complete_f)

        // let f = (v1.mul(a)).add(v2.mul(b)).add(v3.mul(c)).add(v4.mul(s1)).add(v5.mul(s2)).add(zw);
        // f

    }

    pub fn calcualteR0(alpha: Fp256<FrParameters>, alpha2: Fp256<FrParameters>, beta: Fp256<FrParameters>, gamma: Fp256<FrParameters>, proof: PlonkProof, lagrange: Fp256<FrParameters>, pi: Fp256<FrParameters>) -> Fp256<FrParameters> {
        let PlonkProof {
            eval_a: a,
            eval_b: b,
            eval_c: c,
            eval_s1: s1,
            eval_s2: s2,
            eval_zw: zw,
            ..
        } = proof;

        let e1 = pi;
        println!("e1 {:?}", e1.to_string());
        // let e1b = BigInt::from_str(s1.to_string().as_str()).unwrap();
        let e2 = lagrange.mul(alpha2);
        println!("e2 {:?}", e2.to_string());
        
        let e3a = ((beta.mul(s1)).add(a)).add(gamma);

        println!("e3a {:?}", e3a.to_string());
        let e3b = (beta.mul(s2).add(b)).add(gamma);
        println!("e3b {:?}", e3b.to_string());
        let e3c = c.add(gamma);
        println!("e3c {:?}", e3c.to_string());
        let e3 = alpha.mul(zw.mul(e3c.mul(e3a.mul(e3b))));
        println!("e3 {:?}", e3.to_string());
        let ri  = e1.sub(e2);
        println!("ri {:?}", ri.to_string());
        let r0 = ri.sub(e3);

        r0
    }

    pub fn calculateLagrange(n: Fp256<FrParameters> , zh: Fp256<FrParameters>) -> Fp256<FrParameters> {
        let w = Fr::one();

        let denom = n * (zh - w);
        let domain: u64 = 2048;
        let numerator = w * (zh.pow([domain]) - w);
        let lagrange = numerator.mul(denom.inverse().unwrap());
        // let val = lagrange

        print!("Lagrange {:?}", lagrange.to_string());

        lagrange
    }
}
