mod utils {

    use ark_bn254::{Bn254, Fr, G1Projective};
    use ark_poly::univariate::DensePolynomial;
    use ark_ec::{PairingEngine, AffineCurve, ProjectiveCurve};
    use ark_ff::{One}
    use std::{
        convert::TryInto, fmt::Display, ops::{Add, Mul}, rc::Rc, str::FromStr, sync::Mutex
    };

    pub type G1Point = <Bn254 as PairingEngine>::G1Affine;
    pub type G2Point = <Bn254 as PairingEngine>::G2Affine;
    pub type Poly = DensePolynomial<Fr>;

    // pub struct KzgScheme<'a>(&'a Srs);
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct KzgCommitment(pub G1Point);

    impl KzgCommitment {
        pub fn inner(&self) -> &G1Point {
            &self.0
        }
    }
    pub struct PlonkProof {
        pub a: KzgCommitment,
        pub b: KzgCommitment,
        pub c: KzgCommitment,
        pub z: KzgCommitment,
        pub t1: KzgCommitment,
        pub t2: KzgCommitment,
        pub t3: KzgCommitment,
        pub eval_a: Fr,
        pub eval_b: Fr,
        pub eval_c: Fr,§
        pub eval_s1: Fr,
        pub eval_s2: Fr,
        pub eval_zw: Fr,
        pub eval_r: Fr,
        pub wxi: KzgCommitment,
        pub wxiw: KzgCommitment,
    }

    pub fn get_plonk_proof() -> PlonkProof {

        let a_x_p =  <G1Point as AffineCurve>::BaseField::from_str("3391331107546193050127490912878524732339221319611096053981693850015879041085").unwrap();
        let a_y_p = <G1Point as AffineCurve>::BaseField::from_str("6451615961075051326185606036856749805800164346541076346184303537381960838734").unwrap();
        
        let a_affine = G1Projective::new(a_x_p, a_y_p, <G1Projective as ProjectiveCurve>::BaseField::one()).into_affine();

        let a_commitment = KzgCommitment(a_affine);

        let b_x_p =  <G1Point as AffineCurve>::BaseField::from_str("21683537853259339251063152245237531702090647847126293038809031327008963203108").unwrap();
        let b_y_p = <G1Point as AffineCurve>::BaseField::from_str("18391047229041192027119387365071094242052630625649388692858480541036553880351").unwrap();
        
        let b_affine: ark_ec::short_weierstrass_jacobian::GroupAffine<ark_bn254::g1::Parameters> = G1Projective::new(b_x_p, b_y_p, <G1Projective as ProjectiveCurve>::BaseField::one()).into_affine();
        let b_commitment = KzgCommitment(b_affine);

        let c_x_p =  <G1Point as AffineCurve>::BaseField::from_str("11621292064649230445391937986030104439133575577360444671422851910837301411829").unwrap();
        let c_y_p = <G1Point as AffineCurve>::BaseField::from_str("12846527394780535006077377608327231671887851965940114019717015962888986407415").unwrap();
        
        let c_affine = G1Projective::new(c_x_p, c_y_p, <G1Projective as ProjectiveCurve>::BaseField::one()).into_affine();

        let c_commitment = KzgCommitment(a_affine);

        let z_x_p = <G1Point as AffineCurve>::BaseField::from_str("9662530300557940141476783491796212064762163872492397029727967414740213081250").unwrap();
        let z_y_p = <G1Point as AffineCurve>::BaseField::from_str("2230166349457580308212762490897620014521593936716050864065785773322261326858").unwrap();

        let z_affine = G1Projective::new(z_x_p, z_y_p, <G1Projective as ProjectiveCurve>::BaseField::one()).into_affine();
        let z_commitment = KzgCommitment(z_affine);

        let wxi_x_p = <G1Point as AffineCurve>::BaseField::from_str("16492338865768995854154662306809174646444657225640052473237223839472028908904").unwrap();
        let wxi_y_p = <G1Point as AffineCurve>::BaseField::from_str("21038495490098114078539448908688428463282035962450468815928523871851326375578").unwrap();

        let wxi_affine = G1Projective::new(wxi_x_p, wxi_y_p, <G1Projective as ProjectiveCurve>::BaseField::one()).into_affine();
        let wxi_commitment = KzgCommitment(wxi_affine);

        let wxi_x_p = <G1Point as AffineCurve>::BaseField::from_str("16492338865768995854154662306809174646444657225640052473237223839472028908904").unwrap();
        let wxi_y_p = <G1Point as AffineCurve>::BaseField::from_str("21038495490098114078539448908688428463282035962450468815928523871851326375578").unwrap();

        let wxi_affine = G1Projective::new(wxi_x_p, wxi_y_p, <G1Projective as ProjectiveCurve>::BaseField::one()).into_affine();
        let wxi_commitment = KzgCommitment(wxi_affine);

        let wxiw_x_p = <G1Point as AffineCurve>::BaseField::from_str("2780525989726764018420379210379839759994073162261397943283103257756511856860").unwrap();
        let wxiw_y_p = <G1Point as AffineCurve>::BaseField::from_str("12923982292430325868323127343515309985083240380358225279418756761288606931447").unwrap();

        let wxiw_affine = G1Projective::new(wxi_x_p, wxi_y_p, <G1Projective as ProjectiveCurve>::BaseField::one()).into_affine();
        let wxiw_commitment = KzgCommitment(wxi_affine);

        let z_affine = G1Projective::new(z_x_p, z_y_p, <G1Projective as ProjectiveCurve>::BaseField::one()).into_affine();
        let z_commitment = KzgCommitment(z_affine);

        let z_x_p = <G1Point as AffineCurve>::BaseField::from_str("9662530300557940141476783491796212064762163872492397029727967414740213081250").unwrap();
        let z_y_p = <G1Point as AffineCurve>::BaseField::from_str("2230166349457580308212762490897620014521593936716050864065785773322261326858").unwrap();

        let z_affine = G1Projective::new(z_x_p, z_y_p, <G1Projective as ProjectiveCurve>::BaseField::one()).into_affine();
        let z_commitment = KzgCommitment(z_affine);

        let eval_zw = Fr::from_str("18783639038834474674067895191355971310694141864269021285850927873317795382077").unwrap();

        let t1_x_p = <G1Point as AffineCurve>::BaseField::from_str("13976153128590576937397889869875821651092117150069518539559298883533980395680").unwrap();
        let t1_y_p = <G1Point as AffineCurve>::BaseField::from_str("11018714681232781839102084684877965925499276423335639813473370288005567457661").unwrap();

        let t2_x_p = <G1Point as AffineCurve>::BaseField::from_str("19763526060651013734725678968159864877425295863829308322002659641339268836352").unwrap();
        let t2_y_p = <G1Point as AffineCurve>::BaseField::from_str("4665078093642836254430224528352784659803758008562344660512316265311681020080").unwrap();

        let t3_x_p = <G1Point as AffineCurve>::BaseField::from_str("19808840592623791914719484994224028955450551831956248163361229498834932936048").unwrap();
        let t3_y_p = <G1Point as AffineCurve>::BaseField::from_str("2233260682003588131111649695626313810047664148795308977544617839230576778268").unwrap();

        let t1_affine = G1Projective::new(t1_x_p, t1_y_p, <G1Projective as ProjectiveCurve>::BaseField::one()).into_affine();
        let t2_affine = G1Projective::new(t2_x_p, t2_y_p, <G1Projective as ProjectiveCurve>::BaseField::one()).into_affine();
        let t3_affine = G1Projective::new(t3_x_p, t3_y_p, <G1Projective as ProjectiveCurve>::BaseField::one()).into_affine();

        let t1_commitment = KzgCommitment(t1_affine);
        let t2_commitment = KzgCommitment(t2_affine);
        let t3_commitment = KzgCommitment(t3_affine);

        let proof: PlonkProof = PlonkProof {
            a: a_commitment,
            b: b_commitment,
            c: c_commitment,
            z: z_commitment,
            t1: t1_commitment,
            t2: t2_commitment,
            t3: t3_commitment,
            eval_a: Fr::from_str("4302624146366450652487434222703471747066376001951794309125803650188206585261").unwrap(),
            eval_b: Fr::from_str("17074639252741075737925596332580730307358217990958820868307768383290159985506").unwrap(),
            eval_c: Fr::from_str("915701340966044793273109906212178530360818667100526556142788382544113554046").unwrap(),
            eval_s1: Fr::from_str("4932226675697194216764262696292010946770188623631360976373609344562530495050").unwrap(),
            eval_s2: Fr::from_str("1834885551332428838300727375535561819990018043056048479258162325986116088327").unwrap(),
            eval_zw: eval_zw,
            eval_r: Fr::from_str("11410710969449562470071038294456377378562621755076252052836692477638805717495").unwrap(),
            wxi: wxi_commitment,
            wxiw: wxiw_commitment,
        };
        proof
    }

    pub fn get_vk() {
        
    }
}
