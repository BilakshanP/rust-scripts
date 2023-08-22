#![allow(unused)]

pub mod macros;

module!( pub base, hash, enc_dec, functions, data_structures, algorithms, mathematica );

type Time = [u8; 3];
type ToTime = [Time; 2];

#[derive(Clone)]
struct Subject {
    name: String,
    room: String,
    group: Option<u8>,
    teacher: String 
}

struct Day([Option<Subject>; 5]);

fn main() {
    for i in 1..=5 {
        println!("{}", get_time(i))
    }
}

#[allow(clippy::zero_prefixed_literal)]
fn period_to_time(n: usize) -> ToTime {
    [
        [[09, 00, 00], [10, 30, 00]],
        [[10, 30, 00], [12, 00, 01]],
        [[12, 00, 00], [01, 30, 01]],
        [[01, 30, 01], [03, 00, 01]],
        [[03, 00, 01], [04, 30, 01]]
    ][n]
}

fn apm(ap: u8) -> String {
    match ap {
        0 => "AM",
        1 => "PM",
        _ => unreachable!()
    }.to_owned()
}

fn format_time(to_time: ToTime) -> String {
    let h1: u8 = to_time[0][0];
    let m1: u8 = to_time[0][1];

    let h2: u8 = to_time[1][0];
    let m2: u8 = to_time[1][1];

    let ap1: String = apm(to_time[0][2]);
    let ap2: String = apm(to_time[1][2]);

    format!("{:0>2}:{:0>2} {} - {:0>2}:{:0>2} {}", h1, m1, ap1, h2, m2, ap2)
}

fn get_time(period: usize) -> String {
    format_time(period_to_time(period - 1))
}