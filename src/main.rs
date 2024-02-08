use itertools::Itertools;
use std::env::args;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d20;
mod d21;
mod d22;
mod d23;
mod d24;
mod d25;

fn main() {
    let mut args = args().skip(1).map(|x| x.parse());

    let Some((Ok(day), Ok(part))) = args.next_tuple() else {
        panic!("Provide an integer day and part as arguments.")
    };

    let solution = match (day, part) {
        (1, 1) => d01::p1(),
        (1, 2) => d01::p2(),
        (2, 1) => d02::p1(),
        (2, 2) => d02::p2(),
        (3, 1) => d03::p1(),
        (3, 2) => d03::p2(),
        (4, 1) => d04::p1(),
        (4, 2) => d04::p2(),
        (5, 1) => d05::p1(),
        (5, 2) => d05::p2(),
        (6, 1) => d06::p1(),
        (6, 2) => d06::p2(),
        (7, 1) => d07::p1(),
        (7, 2) => d07::p2(),
        (8, 1) => d08::p1(),
        (8, 2) => d08::p2(),
        (9, 1) => d09::p1(),
        (9, 2) => d09::p2(),
        (10, 1) => d10::p1(),
        (10, 2) => d10::p2(),
        (11, 1) => d11::p1(),
        (11, 2) => d11::p2(),
        (12, 1) => d12::p1(),
        (12, 2) => d12::p2(),
        (13, 1) => d13::p1(),
        (13, 2) => d13::p2(),
        (14, 1) => d14::p1(),
        (14, 2) => d14::p2(),
        (15, 1) => d15::p1(),
        (15, 2) => d15::p2(),
        (16, 1) => d16::p1(),
        (16, 2) => d16::p2(),
        (17, 1) => d17::p1(),
        (17, 2) => d17::p2(),
        (18, 1) => d18::p1(),
        (18, 2) => d18::p2(),
        (19, 1) => d19::p1(),
        (19, 2) => d19::p2(),
        (20, 1) => d20::p1(),
        (20, 2) => d20::p2(),
        (21, 1) => d21::p1(),
        (21, 2) => d21::p2(),
        (22, 1) => d22::p1(),
        (22, 2) => d22::p2(),
        (23, 1) => d23::p1(),
        (23, 2) => d23::p2(),
        (24, 1) => d24::p1(),
        (24, 2) => d24::p2(),
        (25, 1) => d25::p1(),
        (25, 2) => d25::p2(),
        _ => panic!("There is no problem for day {day}, part {part}."),
    };

    println!("The solution to day {day}, part {part} is:");
    println!("{solution}");
}
