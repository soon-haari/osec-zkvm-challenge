# Exploiting zkVMs via Fiat-Shamir bug (Task from OtterSec)

[X post](https://x.com/ah_p_uh/status/2029932399602336079?s=20)

This is a writeup for the both of the [OtterSec's zkVM post challenge](https://osec.io/blog/2026-03-03-zkvms-unfaithful-claims#challenges). Note that my exploit isn't the most optimal or clean, but I was focused on solving them as quick as possible.

## Jolt
- [handout](https://osec.io/posts/zkvms-unfaithful-claims/handout_jolt.tar.gz)
- `jolt.chal.osec.io:8960`


In the post, it is mentioned that `opening_claims` are important factors that's needed in the internal sumchecks, however not added into transcript, so the checking values can be manually fixed from `opening_claims`. And the post hints that they are linear, so solving linear equation on a field would solve it.
```rs
JoltProof {
    commitments: Vec<Commitment>,           // Polynomial commitments to trace
    opening_claims: Map<OpeningId, Claim>,  // <- THE VULNERABLE VALUES
    proofs: Map<Stage, SumcheckProof>,      // Sumcheck and opening proofs
    ...
}
```

Internally there exists 5 sumcheck checks, and one dory check. As an attacker, you can easily pass the dory check by fixing the transcript manually a little bit, or fixing the prover part to ignore any mischecks. I selected fixing the transcript.

During the verification, I replaced the following checks:
```rs
println!("a = {:?}", output_claim);
println!("b = {:?}", expected_output_claim);

if output_claim != expected_output_claim {
    // return Err(ProofVerifyError::SumcheckVerificationError);
}       
```
And by changing some values of `opening_claims` in the final part, I can check if a or b is fixed, and if a or b is linearly changing from my edit of opening_claims. With enough searches, I could gather enough space to pass all 5 checks manually. Used SageMath for solving linear equations on the needed curve field.


For fixing the transcript, this is the only needed part.

```rs
    pub fn reduce_and_prove<ProofTranscript: Transcript, PCS: CommitmentScheme<Field = F>>(
        &mut self,
        mut polynomials: HashMap<CommittedPolynomial, MultilinearPolynomial<F>>,
        mut opening_hints: HashMap<CommittedPolynomial, PCS::OpeningProofHint>,
        pcs_setup: &PCS::ProverSetup,
        transcript: &mut ProofTranscript,
    ) -> ReducedOpeningProof<F, PCS, ProofTranscript> {
        tracing::debug!(
            "{} sumcheck instances in batched opening proof reduction",
            self.sumchecks.len()
        );

        transcript.append_bytes(b"fix1asdfasdfasdfasdf");

...
    pub fn reduce_and_verify<ProofTranscript: Transcript, PCS: CommitmentScheme<Field = F>>(
        &mut self,
        pcs_setup: &PCS::VerifierSetup,
        commitment_map: &mut HashMap<CommittedPolynomial, PCS::Commitment>,
        reduced_opening_proof: &ReducedOpeningProof<F, PCS, ProofTranscript>,
        transcript: &mut ProofTranscript,
    ) -> Result<(), ProofVerifyError> {
        #[cfg(test)]
        if let Some(prover_openings) = &self.prover_opening_accumulator {
            assert_eq!(prover_openings.len(), self.len());
        }

        // transcript.append_bytes(b"logloglog");
```
Get the trascript state from verfier's `"logloglog"` part, and copy that into prover's `"fix1asdfasdfasdfasdf"` part.

- `Congrats! Send this to crypto@osec.io: osec{1m_d01ng_10_000_c4lcu14t10ns_p3r_s3c0nd,_4nd_th3y're_4ll_wr0ng}`

---

## Nexus
- [handout](https://osec.io/posts/zkvms-unfaithful-claims/handout_nexus.tar.gz)
- `nexus.chal.osec.io:8950`

Unlike Jolt, this was relying on a single check. Reading the post, `claimed_sum` has the only constraint of sum of it being zero, and it isn't fed to the transcript as well. We can immediately thinking of one way exploiting this, just add k to one value and subtract k to another, and check if it's linearly done.

I first added the following part to `vm/src/trace.rs`'s `k_trace` function with some help from LLM.
```rs
    // Must match verify_expected encoding exactly.
    let mut out = postcard::to_stdvec_cobs(&Some(true)).expect("encode Some(true)");
    let padded_len = (out.len() + 3) & !3;
    out.resize(padded_len, 0x00);

    if let Some(layout) = view.memory_layout {
        view.output_memory = out
            .iter()
            .enumerate()
            .map(|(i, b)| emulator::PublicOutputEntry {
                address: layout.public_output_start() + i as u32,
                value: *b,
            })
            .collect();
    }
```

This allows to prove any invalid proofs, with invalid witnesses, the only error is `Err(ProvingError::ConstraintsNotSatisfied)`.

Simply edit out stwo's `crates/stwo/src/prover/mod.rs`.
```rs
    {
        // return Err(ProvingError::ConstraintsNotSatisfied);
    }
```

And in function `verify` in `crates/stwo/src/core/verifier.rs` edit to the following so that the only check is changing linearly well.

```rs
    let a = composition_oods_eval;
    let b = components.eval_composition_polynomial_at_point(
            oods_point,
            &proof.sampled_values,
            random_coeff,
        );

    println!("{}", a);
    println!("{}", b);
```

This is a single check, so don't even need linear equation here. However, this use a very unique QM31 field, so the arithmatics has to be done on that field.

Check out `nexus-zkvm/ex.sage`.

- `Congrats! Send this to crypto@osec.io: osec{y34h_1_kn0w_l1n34r_4lg3br4: y=a*x+b}`