use ark_ec::AffineCurve;

pub mod verifier {
    use ark_bn254::{Bn254, FqParameters, Fr, FrParameters};
    use ark_ec::short_weierstrass_jacobian::GroupAffine;
    use ark_ec::*;
    use ark_ff::{Field, Fp256, One, PrimeField, UniformRand, Zero};
    // use ark_ff::*;
    pub use crate::utils::utils::{get_plonk_proof, PlonkProof};
    use ark_poly::univariate::DensePolynomial;
    use ark_poly::{domain, Polynomial};
    use std::fmt::{Debug, DebugMap, Display};
    use std::marker::PhantomData;
    use std::ops::{Add, Mul, Neg, Sub};
    use std::str::FromStr;
    use num_bigint::*;

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

        let pi: Fp256<FrParameters> = Fr::from_str(
            "10021071990350671093045688305445916367264617343457315103161905320545395462791",
        )
        .unwrap();
    
        let n = Fr::from_str("2048").unwrap(); 

        let lagrange = calculateLagrange(n, xi);

        println!("Lagrange {:?}", lagrange);
        

        let proof: PlonkProof = get_plonk_proof();

        let r0 = calcualteR0(aplha, aplha2, beta, gamma, proof, lagrange, pi);

        print!("final r0 {}", r0.to_string());


        //         let _pD:= add(pMem, pD)
        //         let gamma := mload(add(pMem, pGamma))
        //         let mIn := mload(0x40)
        //         mstore(0x40, add(mIn, 256)) // d1, d2, d3 & d4 (4*64 bytes)

        //         g1_setC(_pD, Qcx, Qcy)
        //         g1_mulAccC(_pD, Qmx, Qmy, mulmod(calldataload(pEval_a), calldataload(pEval_b), q))
        //         g1_mulAccC(_pD, Qlx, Qly, calldataload(pEval_a))
        //         g1_mulAccC(_pD, Qrx, Qry, calldataload(pEval_b))
        //         g1_mulAccC(_pD, Qox, Qoy, calldataload(pEval_c))            

        //         let betaxi := mload(add(pMem, pBetaXi))
        //         let val1 := addmod(
        //             addmod(calldataload(pEval_a), betaxi, q),
        //             gamma, q)

        //         let val2 := addmod(
        //             addmod(
        //                 calldataload(pEval_b),
        //                 mulmod(betaxi, k1, q),
        //                 q), gamma, q)

        //         let val3 := addmod(
        //             addmod(
        //                 calldataload(pEval_c),
        //                 mulmod(betaxi, k2, q),
        //                 q), gamma, q)

        //         let d2a := mulmod(
        //             mulmod(mulmod(val1, val2, q), val3, q),
        //             mload(add(pMem, pAlpha)),
        //             q
        //         )

        //         let d2b := mulmod(
        //             mload(add(pMem, pEval_l1)),
        //             mload(add(pMem, pAlpha2)),
        //             q
        //         )

        //         // We'll use mIn to save d2
        //         g1_calldataSet(add(mIn, 192), pZ)
        //         g1_mulSet(
        //             mIn,
        //             add(mIn, 192),
        //             addmod(addmod(d2a, d2b, q), mload(add(pMem, pU)), q))


        //         val1 := addmod(
        //             addmod(
        //                 calldataload(pEval_a),
        //                 mulmod(mload(add(pMem, pBeta)), calldataload(pEval_s1), q),
        //                 q), gamma, q)

        //         val2 := addmod(
        //             addmod(
        //                 calldataload(pEval_b),
        //                 mulmod(mload(add(pMem, pBeta)), calldataload(pEval_s2), q),
        //                 q), gamma, q)
    
        //         val3 := mulmod(
        //             mulmod(mload(add(pMem, pAlpha)), mload(add(pMem, pBeta)), q),
        //             calldataload(pEval_zw), q)
    

        //         // We'll use mIn + 64 to save d3
        //         g1_mulSetC(
        //             add(mIn, 64),
        //             S3x,
        //             S3y,
        //             mulmod(mulmod(val1, val2, q), val3, q))

        //         // We'll use mIn + 128 to save d4
        //         g1_calldataSet(add(mIn, 128), pT1)

        //         g1_mulAccC(add(mIn, 128), calldataload(pT2), calldataload(add(pT2, 32)), mload(add(pMem, pXin)))
        //         let xin2 := mulmod(mload(add(pMem, pXin)), mload(add(pMem, pXin)), q)
        //         g1_mulAccC(add(mIn, 128), calldataload(pT3), calldataload(add(pT3, 32)) , xin2)
                
        //         g1_mulSetC(add(mIn, 128), mload(add(mIn, 128)), mload(add(mIn, 160)), mload(add(pMem, pZh)))

        //         mstore(add(add(mIn, 64), 32), mod(sub(qf, mload(add(add(mIn, 64), 32))), qf))
        //         mstore(add(mIn, 160), mod(sub(qf, mload(add(mIn, 160))), qf))
        //         g1_acc(_pD, mIn)
        //         g1_acc(_pD, add(mIn, 64))
        //         g1_acc(_pD, add(mIn, 128))

        // calculateD()

        // print!("{:?}", proof);
    }


   
    

    pub fn calculateD(gamma: Fp256<FqParameters>, betaxi: Fp256<FqParameters>, k2: Fp256<FqParameters>, eval_l1: Fp256<FqParameters>, alpha: Fp256<FqParameters>, alpha2: Fp256<FqParameters>, u: Fp256<FqParameters>, verifybetaXi: Fp256<FqParameters>){
        let qc: GroupAffine<ark_bn254::g1::Parameters> = GroupAffine::new(Fp256::from_str("0").unwrap(),
        Fp256::from_str("0").unwrap(),
         true);
        let eval_a_into_eval_b: Fp256<FqParameters> = Fp256::from_str("1645188846976919578283751811856989119143482059083800477641627476904775990270").unwrap();
        let eval_a: Fp256<FqParameters> = Fp256::from_str("7619444648548762352688989264071365525087666293572605752963973137331466620379").unwrap();
        let eval_b: Fp256<FqParameters> = Fp256::from_str("12564993388515609407621530932388481577961227603586802807221481569176168238260").unwrap();
        let eval_c: Fp256<FqParameters> = Fp256::from_str("6511986115001766925734365330664692166783761208764259458159980563836620574767").unwrap();
        let qm: GroupAffine<ark_bn254::g1::Parameters> = GroupAffine::new(Fp256::from_str("20835273517253247507278161354140085192179560558424391762960775729600393482750").unwrap(),
        Fp256::from_str("16191201213275001001200617578554070333626688786050641588918630575263395623273").unwrap(),
         false);
        let qm_ab = qm.mul(eval_a_into_eval_b);
        
        let ql: GroupAffine<ark_bn254::g1::Parameters> = GroupAffine::new(Fp256::from_str("20835273517253247507278161354140085192179560558424391762960775729600393482750").unwrap(),
        Fp256::from_str("16191201213275001001200617578554070333626688786050641588918630575263395623273").unwrap(),
         false);

        let qr: GroupAffine<ark_bn254::g1::Parameters> = GroupAffine::new(Fp256::from_str("6900030744989144129848893583598672235257204177548311761347544245788955028280").unwrap(),
        Fp256::from_str("8155125105494137927083991839474623324411895145542585614480259473774672439508").unwrap(),
         false);


        let ql_a = ql.mul(eval_a);

        let qr_b = qr.mul(eval_b);
        let qo: GroupAffine<ark_bn254::g1::Parameters> = GroupAffine::new(Fp256::from_str("15946180093115511093353920492758773804069483402874922499479809500987551267911").unwrap(),
        Fp256::from_str("10782711402358324053795706160377115050675566507577901529557399547946751276930").unwrap(),
         false);

        let qo_c = qo.mul(eval_c);



        let d = qc.add(qm_ab.into()).add(ql_a.into()).add(qr_b.into()).add(qo_c.into());
        //todo values should be in q field
        let val1 = (eval_a.add(betaxi)).add(gamma);

        let val2 = (eval_b.add(betaxi.mul(Fp256::from(1)))).add(gamma);

        let val3 = gamma.add(eval_c.add(betaxi.mul(k2)));

        let d2a = val1.mul(val2.mul(val3)).mul(alpha);

        let d2b = eval_l1.mul(alpha2);

        let d2 = d2a.add(d2b).add(u);

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
        let e2 = lagrange.mul(alpha2);
        let e3a = beta.mul(s1).add(gamma);
        let e3b = (beta.mul(s2).add(b)).add(gamma);
        let e3c = c.add(gamma);
        let e3 = alpha.mul(zw.mul(e3c.mul(e3a.mul(e3b))));
        let r0 = e3.sub(e1.sub(e2));

        r0

        // let exp1 = alpha.mul(a.add(beta.mul(s1).add(gamma)));
        // let exp2 = b.add(beta.mul(s2).add(gamma));
        // let exp3 = c.add(gamma).mul(zw);

        // let final_r0 = pi.sub(lagrange.mul(alpha2).sub(exp1.mul(exp2.mul(exp3))));

        // final_r0
        // need to do PI(zh) - L1(zh)*alpha2 - final_exp 
        // ps waiting for PI(zh) and L1(zh) calculation 
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
