(module
    (memory $0 16)
    (func $store (export "store") (param $N i32) (result i64)
        ;; Store '10' to address $N
        (i64.store
            (local.get $N)
            (i64.const 10)
        )
        ;; Load the value from address $N and return it
        (i64.load
            (local.get $N)
        )
    )
)
