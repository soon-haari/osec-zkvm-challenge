use std::path::PathBuf;

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
    let elf_path: PathBuf = match std::env::var("ELF_PATH") {
        Ok(elf_path) => elf_path.into(),
        Err(_) => {
            let mut prover_compiler = Compiler::<CargoPackager>::new(PACKAGE);
            let elf_path = prover_compiler.build()?;
            println!("{}", elf_path.to_str().unwrap());
            return Ok(());
        }
    };

    print_hex(&std::fs::read(&elf_path)?);

    let prover = Stwo::new_from_file(&elf_path)?;
    let elf = prover.elf.clone();

    let mut lines = std::io::stdin().lines();
    let a: u64 = lines.next().expect("EOF")?.parse()?;
    let b: u64 = lines.next().expect("EOF")?.parse()?;
    let c: u64 = lines.next().expect("EOF")?.parse()?;
    let proof_hex = lines.next().expect("EOF")?;
    let proof_bytes = (0..proof_hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&proof_hex[i..i + 2], 16))
        .collect::<Result<Vec<u8>, _>>()?;
    let proof: Proof = postcard::from_bytes(&proof_bytes)?;

    let public_input = (a, b, c);
    let public_output = Some(true); // please provide a counter example of flt

    proof
        .verify_expected(
            &public_input,
            KnownExitCodes::ExitSuccess as u32,
            &public_output,
            &elf,
            &prover.ad,
        )
        .expect("Failed to verify proof");
    println!("Proof verified!");
    println!("{}", std::fs::read_to_string("/flag.txt")?);
    Ok(())
}
