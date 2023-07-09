```rust
let /* mut */ value: Type = /* data */;
let ptr *const Type = &value;
// let ptr /* : type * / = &value as *const Type;
let address: usize = ptr as usize;
let mut_ptr: *mut Type_or_2 = address as *mut Type_or_2;
// let mut_or_ptr: *const/mut Type_or_2 = &mut/? value;
// a *const can have &mut but not vic-versa;
// value must be mutable as well

// to read:
unsafe {
    *ptr; *mut_ptr;
}

// to write:
*pre/mut_ptr = /* data */;
```