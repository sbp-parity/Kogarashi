use confidential_transfer::ConfidentialTransferCircuit;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::rngs::StdRng;
use rand::SeedableRng;
use zero_elgamal::EncryptedNumber;
use zero_jubjub::*;
use zero_kzg::KeyPair;
use zero_pairing::TatePairing;
use zero_plonk::prelude::*;
use zkstd::common::{CurveGroup, Group};

#[allow(unused_must_use)]
fn circuit(c: &mut Criterion) {
    let mut group = c.benchmark_group("circuit");

    let mut rng = StdRng::seed_from_u64(8349u64);
    let n = 14;
    let label = b"bench";
    group.bench_function("setup", |b| {
        b.iter(|| KeyPair::<TatePairing>::setup(n, BlsScalar::random(&mut rng)));
    });

    let mut pp = KeyPair::<TatePairing>::setup(n, BlsScalar::random(&mut rng));
    let (prover, verifier) =
        Compiler::compile::<ConfidentialTransferCircuit, TatePairing>(&mut pp, label)
            .expect("failed to compile circuit");
    let generator = JubjubExtended::ADDITIVE_GENERATOR;
    let alice_private_key = JubjubScalar::random(&mut rng);
    let bob_private_key = JubjubScalar::random(&mut rng);
    let alice_public_key = generator * alice_private_key;
    let bob_public_key = generator * bob_private_key;
    let alice_balance = 1500;
    let transfer_amount_b = 800;
    let alice_after_balance = JubjubScalar::from(700_u64);
    let alice_original_randomness = JubjubScalar::from(789_u64);
    let randomness = JubjubScalar::from(123_u64);
    let alice_encrypted_balance =
        EncryptedNumber::encrypt(alice_private_key, alice_balance, alice_original_randomness);
    let alice_transfer_amount =
        EncryptedNumber::encrypt(alice_private_key, transfer_amount_b, randomness);
    let transfer_amount_scalar = JubjubScalar::from(transfer_amount_b as u64);
    let bob_left_encrypted_transfer_amount =
        (generator * transfer_amount_scalar) + (bob_public_key * randomness);

    let circuit = ConfidentialTransferCircuit::new(
        JubjubAffine::from(alice_public_key),
        JubjubAffine::from(bob_public_key),
        alice_encrypted_balance,
        alice_transfer_amount,
        JubjubAffine::from(bob_left_encrypted_transfer_amount),
        alice_private_key,
        transfer_amount_scalar,
        alice_after_balance,
        randomness,
    );

    let (proof, public_inputs) = prover.prove(&mut rng, &circuit).expect("failed to prove");

    group.bench_function("gen_proof", |b| {
        b.iter(|| {
            black_box(prover.prove(&mut rng, black_box(&circuit)));
        })
    });

    group.bench_function("verify_proof", |b| {
        b.iter(|| {
            black_box(verifier.verify(black_box(&proof), black_box(&public_inputs)));
        })
    });
}

criterion_group!(
    name = bench_confidential_transfer;
    config = Criterion::default().sample_size(10);
    targets = circuit
);
criterion_main!(bench_confidential_transfer);
