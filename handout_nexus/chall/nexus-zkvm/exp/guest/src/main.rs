#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

#[nexus_rt::main]
#[nexus_rt::public_input(a, b, c)]
fn main(a: u64, b: u64, c: u64) -> Option<bool> {
    if a == 0 || b == 0 || c == 0 {
        return None;
    }
    fn pow3(x: u64) -> Option<u64> {
        x.checked_mul(x)?.checked_mul(x)
    }
    // Check that a^3 + b^3 = c^3
    Some(pow3(a)?.checked_add(pow3(b)?)? == pow3(c)?)
}