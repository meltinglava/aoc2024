#![feature(iterator_try_reduce)]

use nom::{error::Error, Err as NomErr};

mod day01;
mod day02;
mod day03;

extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

aoc_lib! { year = 2024 }

/// Convert `nom::Err<Error<&str>>` to `nom::Err<Error<String>>`
pub fn convert_error_to_owned(e: NomErr<Error<&str>>) -> NomErr<Error<String>> {
    match e {
        NomErr::Incomplete(needed) => NomErr::Incomplete(needed),
        NomErr::Error(err) => NomErr::Error(Error {
            input: err.input.to_owned(),
            code: err.code,
        }),
        NomErr::Failure(err) => NomErr::Failure(Error {
            input: err.input.to_owned(),
            code: err.code,
        }),
    }
}
