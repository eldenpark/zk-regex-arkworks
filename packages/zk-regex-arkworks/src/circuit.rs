// use ark_bls12_381::Bls12_381;
// use ark_bls12_381::Fr;
// use ark_ff::PrimeField;
// use ark_groth16::{
//     create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof, Proof,
//     VerifyingKey,
// };
// use ark_r1cs_std::{alloc::AllocVar, boolean::Boolean, eq::EqGadget, fields::fp::FpVar, R1CSVar};
// use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
// use rand::thread_rng;

// #[derive(Clone)]
// struct Asterisk1RegexCircuit<F: PrimeField> {
//     pub msg: Vec<F>,
// }

fn print_states<F: PrimeField>(states: &Vec<Boolean<F>>) {
    for (idx, s) in states.iter().enumerate() {
        println!("states, idx: {}, val: {:?}", idx, s.value());
    }
}

// fn asterisk1_regex<F: PrimeField>(msg: Vec<FpVar<F>>) -> Result<Boolean<F>, SynthesisError> {
//     let num_bytes = msg.len();
//     let mut states = vec![Boolean::constant(false); num_bytes + 1];

//     // Initial state
//     states[0] = Boolean::constant(true);

//     print_states(&states);

//     for i in 0..num_bytes {
//         println!("round: {}", i);

//         // Wrapping the constants in FpVar before comparison
//         let is_x = msg[i].is_eq(&FpVar::Constant(F::from(120u128)))?;
//         let is_a = msg[i].is_eq(&FpVar::Constant(F::from(97u128)))?;
//         let is_b = msg[i].is_eq(&FpVar::Constant(F::from(98u128)))?;

//         let next_state = if i == 0 {
//             // First character must be 'x'
//             is_x.clone()
//         } else {
//             // Update state for 'a*' or 'b'
//             Boolean::or(
//                 &Boolean::and(&states[i], &is_a)?,
//                 &Boolean::and(&states[i - 1], &is_b)?,
//             )?
//         };

//         states[i + 1] = Boolean::or(&states[i + 1], &next_state)?;

//         print_states(&states);
//     }

//     // print_states(&states);

//     // Check if the final state is accepted (last state must be 'b')
//     let is_accepted = Boolean::and(
//         &states[num_bytes],
//         &msg.last()
//             .unwrap()
//             .is_eq(&FpVar::Constant(F::from(98u128)))?,
//     )?;

//     Ok(is_accepted)
// }

// impl<F: PrimeField> ConstraintSynthesizer<F> for Asterisk1RegexCircuit<F> {
//     fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
//         println!("self.msg: {:#?}", self.msg);

//         // let msg_vars: Vec<FpVar<F>> = self
//         //     .msg
//         //     .iter()
//         //     .map(|val| FpVar::new_input(cs.clone(), || Ok(*val)).unwrap())
//         //     .collect();

//         let mut msg_vars = vec![];
//         for val in self.msg {
//             let c = FpVar::new_input(cs.clone(), || Ok(val)).unwrap();
//             msg_vars.push(c);
//         }

//         println!("msg_vars len: {}", msg_vars.len());

//         // println!("msg_vars: {:#?}", msg_vars);

//         let is_accepted = asterisk1_regex(msg_vars)?;

//         // Enforce that the regex must be accepted (equals true)
//         is_accepted.enforce_equal(&Boolean::constant(true))?;

//         Ok(())
//     }
// }

// pub fn run() -> Result<(), SynthesisError> {
//     let msg = vec![
//         Fr::from(120u128), // 'x'
//         Fr::from(97u128),  // 'a'
//         Fr::from(98u128),  // 'b'
//     ];

//     // Create a random circuit input (e.g., "xab")
//     let circuit = Asterisk1RegexCircuit { msg: msg.clone() };

//     // Generate the proving and verifying keys
//     let rng = &mut thread_rng();
//     let params = {
//         let circuit = circuit.clone(); // Clone the circuit
//         generate_random_parameters::<Bls12_381, _, _>(circuit, rng).unwrap()
//     };

//     let pvk = prepare_verifying_key(&params.vk);

//     // Generate the proof
//     let proof = create_random_proof(circuit, &params, rng).unwrap();

//     // Prepare the public inputs (none in this case)
//     let public_inputs = msg;

//     // println!("proof: {:?}", proof);

//     // Verify the proof
//     let verified = verify_proof(&pvk, &proof, &public_inputs).unwrap();

//     println!("Proof verification: {}", verified); // Should print true

//     Ok(())
// }
use ark_bls12_381::Bls12_381;
use ark_bls12_381::Fr;
use ark_ff::PrimeField;
use ark_groth16::{
    create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof, Proof,
    VerifyingKey,
};
use ark_r1cs_std::{alloc::AllocVar, boolean::Boolean, eq::EqGadget, fields::fp::FpVar, R1CSVar};
use ark_relations::r1cs::{
    ConstraintSynthesizer, ConstraintSystem, ConstraintSystemRef, SynthesisError,
};
use rand::thread_rng;

#[derive(Clone)]
struct Asterisk1RegexCircuit<F: PrimeField> {
    pub msg: Vec<F>,
}

fn asterisk1_regex<F: PrimeField>(msg: Vec<FpVar<F>>) -> Result<Boolean<F>, SynthesisError> {
    let num_bytes = msg.len();
    let mut states = vec![Boolean::constant(false); num_bytes + 1];

    // Initial state
    states[0] = Boolean::constant(true);

    print_states(&states);

    for i in 0..num_bytes {
        let is_x = msg[i].is_eq(&FpVar::Constant(F::from(120u128)))?; // x
        let is_a = msg[i].is_eq(&FpVar::Constant(F::from(97u128)))?; // a
        let is_b = msg[i].is_eq(&FpVar::Constant(F::from(98u128)))?; // b

        // DFA Transitions
        if i == 0 {
            states[i + 1] = Boolean::or(&states[i], &is_x)?;
        } else {
            let state1 = Boolean::or(&Boolean::and(&states[i], &is_a)?, &is_x)?;
            let state2 = Boolean::and(&states[i], &is_b)?;
            states[i + 1] = Boolean::or(&state1, &state2)?;
        }

        println!("\n round: {}", i);
        print_states(&states);
    }

    // Final state must be the accepting state 2
    let is_accepted = Boolean::and(
        &states[num_bytes],
        &msg.last()
            .unwrap()
            .is_eq(&FpVar::Constant(F::from(98u128)))?,
    )?;

    Ok(is_accepted)
}

impl<F: PrimeField> ConstraintSynthesizer<F> for Asterisk1RegexCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        println!("start \n\n");

        let msg_vars: Vec<FpVar<F>> = self
            .msg
            .iter()
            .map(|val| FpVar::new_input(cs.clone(), || Ok(*val)).unwrap())
            .collect();

        let is_accepted = asterisk1_regex(msg_vars)?;

        // Enforce that the regex must be accepted (equals true)
        is_accepted.enforce_equal(&Boolean::constant(true))?;

        Ok(())
    }
}

// xa*b
pub fn run() -> Result<(), SynthesisError> {
    let input = vec![
        Fr::from(120u128), // 'x'
        Fr::from(120u128), // 'x'
        Fr::from(120u128), // 'x'
        Fr::from(97u128),  // 'a'
        Fr::from(97u128),  // 'b'
    ];

    // Create a random circuit input (e.g., "xab")
    let circuit = Asterisk1RegexCircuit { msg: input.clone() };

    // Generate the proving and verifying keys
    let rng = &mut thread_rng();
    let params = {
        let circuit = circuit.clone(); // Clone the circuit
        generate_random_parameters::<Bls12_381, _, _>(circuit, rng).unwrap()
    };

    let pvk = prepare_verifying_key(&params.vk);

    // Generate the proof
    let proof = create_random_proof(circuit, &params, rng).unwrap();

    let public_inputs = input.clone();

    // Verify the proof
    let verified = verify_proof(&pvk, &proof, &public_inputs).unwrap();

    println!("Proof verification: {}", verified); // Should print true

    Ok(())
}

// fn main() -> Result<(), SynthesisError> {
//     let cs = ConstraintSystem::<Fr>::new_ref();

//     // Example message (replace with your actual input)
//     let msg: Vec<FpVar<Fr>> = vec![
//         FpVar::new_input(cs.clone(), || Ok(Fr::from(120u128)))?,
//         FpVar::new_input(cs.clone(), || Ok(Fr::from(97u128)))?,
//         FpVar::new_input(cs.clone(), || Ok(Fr::from(98u128)))?,
//     ];

//     let is_accepted = asterisk1_regex(msg)?;

//     println!("Regex accepted: {:?}", is_accepted.value()?);

//     Ok(())
// }
