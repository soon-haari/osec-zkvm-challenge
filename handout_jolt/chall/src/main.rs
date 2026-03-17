// use std::path::PathBuf;
// use std::io::Read;
use jolt_sdk::{JoltProof, Serializable, host::Program};
use jolt_sdk::serialize_and_print_size;

fn print_hex(bytes: &[u8]) {
    println!("{}", bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>());
}

pub fn main_() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let target_dir = "/tmp/jolt-guest-targets";
    let elf_path = format!("{target_dir}/guest-flt/riscv64imac-unknown-none-elf/release/guest");

    if !std::path::Path::new(&elf_path).exists() {
        // first time setup, compile guest and create urs
        let mut program = guest::compile_flt(&target_dir);
        let _ = guest::preprocess_prover_flt(&mut program);

        println!("setup done, exiting");
        return Ok(());
    }

    let mut program = Program::new("guest");
    program.elf = Some(elf_path.clone().into());

    print_hex(&std::fs::read(elf_path)?);
    print_hex(&std::fs::read("./dory_urs_24_variables.urs")?);

    let mut lines = std::io::stdin().lines();
    let a: u64 = lines.next().expect("EOF")?.parse()?;
    let b: u64 = lines.next().expect("EOF")?.parse()?;
    let c: u64 = lines.next().expect("EOF")?.parse()?;
    let proof_hex = lines.next().expect("EOF")?;
    let proof_bytes = (0..proof_hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&proof_hex[i..i + 2], 16))
        .collect::<Result<Vec<u8>, _>>()?;
    let proof = JoltProof::deserialize_from_bytes(&proof_bytes)?;

    let prover_preprocessing = guest::preprocess_prover_flt(&mut program);
    let verifier_preprocessing = guest::verifier_preprocessing_from_prover_flt(&prover_preprocessing);

    let verify_flt = guest::build_verifier_flt(verifier_preprocessing);

    let output = Some(true); // please provide a counter example of flt
    let panic = false;
    if verify_flt(a, b, c, output, panic, proof) {
        println!("{}", std::fs::read_to_string("/flag.txt")?);
    } else {
        println!("invalid proof");
    }
    Ok(())
}

use std::fs::File;
use ark_serialize::CanonicalDeserialize;
type Proof =
    jolt_sdk::JoltProof<jolt_sdk::F, jolt_core::poly::commitment::dory::DoryCommitmentScheme, jolt_core::transcripts::Blake2bTranscript>;

pub fn main() {
    let target_dir = "/tmp/jolt-guest-targets";
    let mut program = guest::compile_flt(target_dir);

    let prover_preprocessing = guest::preprocess_prover_flt(&mut program);
    let verifier_preprocessing = guest::verifier_preprocessing_from_prover_flt(&prover_preprocessing);

    // let prove_flt = guest::build_prover_flt(program, prover_preprocessing);
    let verify_flt = guest::build_verifier_flt(verifier_preprocessing);


    /* 
    let (_output, proof, _io_device) = prove_flt(1u64, 1u64, 1u64);

    serialize_and_print_size(
        "jerryproof",
        "./proof.bin",
        &proof,
    ).expect("proof save fail");
    */
    

    let mut pf = File::open("./proof.bin").expect("open proof.bin");
    let proof: Proof = Proof::deserialize_compressed(&mut pf).expect("deserialize proof");


    println!("verifying...");

    let output = Some(true);
    let panic = false;
    let is_valid = verify_flt(1u64, 1u64, 1u64, output, panic, proof);

    println!("valid: {is_valid}");
}