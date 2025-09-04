# Benchmarks for Cairo Zero

We test the performance of Cairo Zero with the following programs:

- Fibonacci
- SHA256

## SHA256

For SHA256, we use a pythonic hint to fill the input values. Because the LC CairoVM doesn't support execution of pythonic hints natively, we use a custom hint processor that's implemented as part of the `cairo-addons` crate in the `keth` repository. We can simply use the `prove-cairo` command of `keth` to prove any cairo-zero program. This pipeline is only available through a python CLI, and thus, that's why we invoke it through a shell command.
