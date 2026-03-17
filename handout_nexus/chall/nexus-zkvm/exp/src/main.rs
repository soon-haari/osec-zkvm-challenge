use nexus_sdk::{
    compile::{cargo::CargoPackager, Compile, Compiler},
    stwo::seq::{Proof, Stwo},
    KnownExitCodes, Prover, Verifiable,
};

const PACKAGE: &str = "guest";

fn print_hex(bytes: &[u8]) {
    println!(
        "{}",
        bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    );
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let elf_path = "/Users/minsun/KU/osec/handout_nexus/chall/nexus-zkvm/elfelf";
    let prover = Stwo::new_from_file(&elf_path)?;
    let elf = prover.elf.clone();
    let ad = prover.ad.clone();

    let public_input = (1u64, 1u64, 1u64);
    let (_view, proof) = prover.prove_with_input::<(), (u64, u64, u64)>(&(), &public_input)?;

    let proof_bytes = postcard::to_stdvec(&proof)?;
    // println!("Proof bytes (hex):");
    print_hex(&proof_bytes);


    let public_output = Some(true);

    let roundtrip_proof: Proof = postcard::from_bytes(&proof_bytes)?;

    roundtrip_proof.verify_expected(
        &public_input,
        KnownExitCodes::ExitSuccess as u32,
        &public_output,
        &elf,
        &ad,
    )?;

    println!("Proof verified!");
    Ok(())
}
