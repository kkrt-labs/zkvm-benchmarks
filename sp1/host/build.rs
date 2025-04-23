use sp1_build::build_program_with_args;

fn main() {
    build_program_with_args("../fibonacci-guest", Default::default());
    build_program_with_args("../sha2-guest", Default::default());
    build_program_with_args("../ecdsa-guest", Default::default());
    build_program_with_args("../transfer-eth-guest", Default::default());
}
