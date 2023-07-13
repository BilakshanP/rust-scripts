```rust
use std::mem::{size_of, align_of};

// size_of::<T>();
// align_of::<T>();

//  u8: 1, 1;
// u16: 2, 2;
// u32: 4, 4;

/*                                                           */
    type T1 = u8;
    type T2 = u16;
    type T3 = u32;

    type T = (T1, T2, T3);

    println!("Tuple<T1, T2, T3> -> Align: {}", ao::<T>());
    println!("Tuple<T1, T2, T3> -> Size:  {}", so::<T>())

/*                                                           */

```