pub mod macros;

module!( pub base, hash, enc_dec, functions, data_structures, algorithms, mathematica );

// #[derive(Debug)]
// pub enum BaseUnit {
//     Length,
//     Mass,
//     Time,
//     ElectricCurrent,
//     Temperature,
//     AmountOfSubstance,
//     LuminousIntensity,
// }

// impl BaseUnit {
//     pub fn get_symbol(&self) -> &str {
//         match self {
//             BaseUnit::Length => "L",
//             BaseUnit::Mass => "M",
//             BaseUnit::Time => "T",
//             BaseUnit::ElectricCurrent => "I",
//             BaseUnit::Temperature => "Θ",
//             BaseUnit::AmountOfSubstance => "N",
//             BaseUnit::LuminousIntensity => "J",
//         }
//     }
// }

// #[derive(Debug, PartialEq)]
// pub enum UnitPrefix {
//     Yotta = 24,
//     Zetta = 21,
//     Exa = 18,
//     Peta = 15,
//     Tera = 12,
//     Giga = 9,
//     Mega = 6,
//     Kilo = 3,
//     Hecto = 2,
//     Deca = 1,
//     None = 0,
//     Deci = -1,
//     Centi = -2,
//     Milli = -3,
//     Micro = -6,
//     Nano = -9,
//     Pico = -12,
//     Femto = -15,
//     Atto = -18,
//     Zepto = -21,
//     Yocto = -24,
// }

// impl UnitPrefix {
//     pub fn get_symbol(&self) -> &str {
//         match self {
//             UnitPrefix::Yotta => "Y",
//             UnitPrefix::Zetta => "Z",
//             UnitPrefix::Exa => "E",
//             UnitPrefix::Peta => "P",
//             UnitPrefix::Tera => "T",
//             UnitPrefix::Giga => "G",
//             UnitPrefix::Mega => "M",
//             UnitPrefix::Kilo => "k",
//             UnitPrefix::Hecto => "h",
//             UnitPrefix::Deca => "da",
//             UnitPrefix::None => "",
//             UnitPrefix::Deci => "d",
//             UnitPrefix::Centi => "c",
//             UnitPrefix::Milli => "m",
//             UnitPrefix::Micro => "μ",
//             UnitPrefix::Nano => "n",
//             UnitPrefix::Pico => "p",
//             UnitPrefix::Femto => "f",
//             UnitPrefix::Atto => "a",
//             UnitPrefix::Zepto => "z",
//             UnitPrefix::Yocto => "y",
//         }
//     }
// }

// #[derive(Debug)]
// pub struct DimensionalUnit {
//     base: BaseUnit,
//     prefix: UnitPrefix,
//     power: i8,
// }

// #[derive(Debug)]
// pub struct Unit {
//     values: [DimensionalUnit; 7],
// }

// impl Default for Unit {
//     fn default() -> Self {
//         Unit {
//             values: [
//                 DimensionalUnit {
//                     base: BaseUnit::Length,
//                     prefix: UnitPrefix::None,
//                     power: 0,
//                 },
//                 DimensionalUnit {
//                     base: BaseUnit::Mass,
//                     prefix: UnitPrefix::None,
//                     power: 0,
//                 },
//                 DimensionalUnit {
//                     base: BaseUnit::Time,
//                     prefix: UnitPrefix::None,
//                     power: 0,
//                 },
//                 DimensionalUnit {
//                     base: BaseUnit::ElectricCurrent,
//                     prefix: UnitPrefix::None,
//                     power: 0,
//                 },
//                 DimensionalUnit {
//                     base: BaseUnit::Temperature,
//                     prefix: UnitPrefix::None,
//                     power: 0,
//                 },
//                 DimensionalUnit {
//                     base: BaseUnit::AmountOfSubstance,
//                     prefix: UnitPrefix::None,
//                     power: 0,
//                 },
//                 DimensionalUnit {
//                     base: BaseUnit::LuminousIntensity,
//                     prefix: UnitPrefix::None,
//                     power: 0,
//                 },
//             ],
//         }
//     }
// }

// impl Unit {
//     pub fn new() -> Self {
//         Unit::default()
//     }

//     pub fn new_from_raw(
//         length: DimensionalUnit,
//         mass: DimensionalUnit,
//         time: DimensionalUnit,
//         electric_current: DimensionalUnit,
//         temperature: DimensionalUnit,
//         amount_of_substance: DimensionalUnit,
//         luminous_intensity: DimensionalUnit,
//     ) -> Self {
//         Unit {
//             values: [
//                 length,
//                 mass,
//                 time,
//                 electric_current,
//                 temperature,
//                 amount_of_substance,
//                 luminous_intensity,
//             ],
//         }
//     }

//     pub fn print(&self) {
//         let sup = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];

//         print!("[");

//         for (i, value) in self.values.iter().enumerate() {
//             let base = value.base.get_symbol();
//             let prefix = value.prefix.get_symbol();
//             let power = value.power;
//             let power = if power == 1 {
//                 "".to_string()
//             } else {
//                 format!("{}", power)
//             };
//             let power = if power.is_empty() {
//                 power
//             } else {
//                 power
//                     .chars()
//                     .map(|c| sup[c.to_digit(10).unwrap() as usize])
//                     .collect()
//             };
//             print!("{}{}{} ", base, prefix, power);
//         }

//         println!("]");
//     }
// }

// fn main() {
//     let unit = Unit::new();
//     unit.print();

//     let length = DimensionalUnit {
//         base: BaseUnit::Length,
//         prefix: UnitPrefix::Kilo,
//         power: 1,
//     };

//     let mass = DimensionalUnit {
//         base: BaseUnit::Mass,
//         prefix: UnitPrefix::Mega,
//         power: 2,
//     };

//     let time = DimensionalUnit {
//         base: BaseUnit::Time,
//         prefix: UnitPrefix::None,
//         power: 0,
//     };

//     let electric_current = DimensionalUnit {
//         base: BaseUnit::ElectricCurrent,
//         prefix: UnitPrefix::None,
//         power: 0,
//     };

//     let temperature = DimensionalUnit {
//         base: BaseUnit::Temperature,
//         prefix: UnitPrefix::None,
//         power: 0,
//     };

//     let amount_of_substance = DimensionalUnit {
//         base: BaseUnit::AmountOfSubstance,
//         prefix: UnitPrefix::None,
//         power: 0,
//     };

//     let luminous_intensity = DimensionalUnit {
//         base: BaseUnit::LuminousIntensity,
//         prefix: UnitPrefix::None,
//         power: 0,
//     };

//     let unit = Unit::new_from_raw(
//         length,
//         mass,
//         time,
//         electric_current,
//         temperature,
//         amount_of_substance,
//         luminous_intensity,
//     );

//     unit.print();
// }

// fn steps(n: i32) -> i32 {
//     match n {
//         0..=2 => n,
//         3 => 4,
//         _ => steps(n - 1) + steps(n - 2) + steps(n - 3),
//     }
// }

fn get_all_points(p1: (i32, i32), p2: (i32, i32)) -> Vec<(i32, i32)> {
    let mut points = vec![];
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let dx = x2 - x1;
    let dy = y2 - y1;
    let steps = if dx.abs() > dy.abs() {
        dx.abs()
    } else {
        dy.abs()
    };
    let x_inc = dx as f64 / steps as f64;
    let y_inc = dy as f64 / steps as f64;
    let mut x = x1 as f64;
    let mut y = y1 as f64;
    points.push((x.round() as i32, y.round() as i32));
    for _ in 0..steps {
        x += x_inc;
        y += y_inc;
        points.push((x.round() as i32, y.round() as i32));
    }
    points
}

fn get_all_points_dda(p1: (i32, i32), p2: (i32, i32)) -> Vec<(i32, i32)> {
    let mut points = vec![];
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let dx = x2 - x1;
    let dy = y2 - y1;
    let steps = if dx.abs() > dy.abs() {
        dx.abs()
    } else {
        dy.abs()
    };
    let x_inc = dx as f64 / steps as f64;
    let y_inc = dy as f64 / steps as f64;
    let mut x = x1 as f64;
    let mut y = y1 as f64;
    points.push((x.round() as i32, y.round() as i32));
    for _ in 0..steps {
        x += x_inc;
        y += y_inc;
        points.push((x.round() as i32, y.round() as i32));
    }
    points
}

fn main() {
    let p1 = (0, 0);
    let p2 = (77, 3);

    let points = get_all_points(p1, p2);
    let points_dda = get_all_points_dda(p1, p2);

    println!("{:?}", points);
    println!("{:?}", points_dda);

    // compare both
    // for (i, (p1, p2)) in points.iter().zip(points_dda.iter()).enumerate() {
    //     if p1 != p2 {
    //         println!("{}: {:?} != {:?}", i, p1, p2);
    //     }
    // }
}
