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

use risc0_zkvm::serde::to_vec;
use risc0_zkvm::{get_prover_server, ExecutorEnv, ExecutorImpl, ProverOpts, VerifierContext};

use crate::Job;

pub fn new_jobs() -> Vec<Job> {
    let mut jobs = Vec::new();
    for iterations in [10u32, 100u32, 1000u32, 10000u32] {
        jobs.push(Job::new(
            format!("fibonacci-{iterations}"),
            risc0_benchmark_methods::FIBONACCI_ELF,
            risc0_benchmark_methods::FIBONACCI_ID.into(),
            to_vec(&iterations).unwrap(),
            iterations as usize,
        ));
    }
    jobs
}

pub fn profile() {
    println!("Start profiling fibonacci(100)...");
    let n = vec![100u32];
    let env = ExecutorEnv::builder().write_slice(&n).build().unwrap();
    let mut exec = ExecutorImpl::from_elf(env, &risc0_benchmark_methods::FIBONACCI_ELF).unwrap();
    let session = exec.run().unwrap();

    let prover = get_prover_server(&ProverOpts::succinct()).unwrap();
    let ctx = VerifierContext::default();
    let receipt = prover.prove_session(&ctx, &session).unwrap().receipt;
    println!("Finish proving!");
}
