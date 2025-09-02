%builtins output

func main{
    output_ptr: felt*,
}() {
    alloc_locals;

    tempvar n;
    %{ ids.n = program_input[0] %}
    let result: felt = fib(0, 1, n);

    assert [output_ptr] = result;
    let output_ptr = output_ptr + 1;
    return ();
}

func fib(a, b, n) -> (res: felt) {
    jmp fib_body if n != 0;
    tempvar result = b;
    return (b,);

    fib_body:
    tempvar y = a + b;
    return fib(b, y, n - 1);
}
