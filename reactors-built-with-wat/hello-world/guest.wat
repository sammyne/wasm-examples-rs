(component
  (core module $length-core-wasm
    (func (export "length") (param $ptr i32) (param $len i32) (result i32)
      local.get $len
    )
    (memory (export "mem") 1)
    (func (export "realloc") (param i32 i32 i32 i32) (result i32)
      i32.const 0    
    )
  )
  (core instance $length-instance (instantiate $length-core-wasm))
  (func (export "length") (param "input" string) (result u32)
    (canon lift
      (core func $length-instance "length")
      (memory $length-instance "mem")
      (realloc (func $length-instance "realloc"))
    )
  )
)
