use std::{fs::File, io::BufReader};

use ark_bn254::Fr;
use ark_ff::PrimeField;
use ark_std::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use serde_json;
use std::str::FromStr;
use zkboo_recursion::{
    native::{poseidon254_native::Poseidon254Native, sha256_native::Sha256Native, NativeBackend},
    *,
};

type F = F256<ark_bn254::Fr>;

fn bench_many_mul_circuit(c: &mut Criterion) {
    let mut circuit_builder = CircuitBuilder::new();
    let input = circuit_builder.inputs(1)[0];
    let mut muled = input;
    for _ in 0..4194304 {
        muled = circuit_builder.mul(&muled, &muled);
    }
    let circuit = circuit_builder.output(&[muled]);
    let mut rng = ark_std::test_rng();

    let secpar = 100;
    println!("prover starts");
    let mut group = c.benchmark_group("2^22 multiplication circuit");
    group.sample_size(10);
    group.bench_function(
        "generate zkboo proof for 2^22 multiplication circuit",
        |b| {
            b.iter(|| {
                let prove_time = start_timer!(|| "Proving");
                let _ = zkboo_prove::<F, Sha256Native<F>, _>(
                    secpar,
                    &mut rng,
                    vec![],
                    &circuit,
                    &[F::one()],
                )
                .unwrap();
                end_timer!(prove_time);
            })
        },
    );

    let proof =
        zkboo_prove::<F, Sha256Native<F>, _>(secpar, &mut rng, vec![], &circuit, &[F::one()])
            .unwrap();
    group.bench_function(
        "verify the zkboo proof for 2^22 multiplication circuit",
        |b| {
            b.iter(|| {
                let verify_time = start_timer!(|| "Verifying");
                let is_valid = proof
                    .verify_whole(secpar, vec![], &circuit, &[F::one()])
                    .unwrap();
                end_timer!(verify_time);
                assert!(is_valid);
            })
        },
    );
}

criterion_group!(benches, bench_many_mul_circuit,);
criterion_main!(benches);