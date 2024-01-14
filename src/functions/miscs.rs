// fn main() {
//     let mut r = PRG::new(0);

//     let mut p = || {
//         println!("{}", r.generate());
//     };

//     p(); p(); p(); p();
// }

#[allow(clippy::upper_case_acronyms)]
struct PRG {
    state: u8
}

impl PRG {
    fn new(seed: u8) -> Self {
        PRG { state: seed }
    }

    fn generate(&mut self) -> u8 {
        self.state = 17_u8.wrapping_mul(self.state).wrapping_add(31_u8);
        self.state
    }
}