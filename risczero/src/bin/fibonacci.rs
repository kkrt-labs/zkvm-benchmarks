// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::time::Instant;

use risc0_zkvm::serde::to_vec;
use risc0_zkvm::{
    get_prover_server, sha::Digest, ExecutorEnv, ExecutorImpl, ProverOpts, VerifierContext,
};
use utils::{bench::benchmark_v2, bench::Metrics, metadata::FIBONACCI_INPUTS};

pub fn main() {
    benchmark_v2(
        benchmark_fib,
        &FIBONACCI_INPUTS,
        "../.outputs/benchmark/fib_risczero.csv",
    );
}

fn benchmark_fib(input: u32) -> Metrics {
    let mut metrics = Metrics::new(input as usize);
    const ELF: &[u8] = risc0_benchmark_methods::FIBONACCI_ELF;
    let image_id: Digest = risc0_benchmark_methods::FIBONACCI_ID.into();
    let input = to_vec(&input).unwrap();

    let env = ExecutorEnv::builder().write_slice(&input).build().unwrap();
    let mut exec = ExecutorImpl::from_elf(env, &ELF).unwrap();
    let start = Instant::now();
    let session = exec.run().unwrap();
    metrics.exec_duration = start.elapsed();
    metrics.cycles = session.user_cycles;

    let prover = get_prover_server(&ProverOpts::succinct()).unwrap();
    let ctx = VerifierContext::default();

    let start = Instant::now();
    let receipt = prover.prove_session(&ctx, &session).unwrap().receipt;
    metrics.proof_duration = start.elapsed();
    metrics.proof_bytes = receipt.inner.succinct().unwrap().seal_size();

    let start = Instant::now();
    receipt.verify(image_id).unwrap();
    metrics.verify_duration = start.elapsed();

    metrics
}
