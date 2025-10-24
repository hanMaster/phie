// SPDX-FileCopyrightText: Copyright (c) 2022 Yegor Bugayenko
// SPDX-License-Identifier: MIT

extern crate phie;

use phie::data::Data;
use phie::emu::{Emu, Opt};
use phie::error::Result;
use std::env;
use std::fs;
use std::str::FromStr;

fn emulate(phi_code: &str) -> Result<Data> {
    let mut emu = Emu::from_str(phi_code)?;
    emu.opt(Opt::LogSnapshots);
    emu.opt(Opt::StopWhenTooManyCycles);
    emu.opt(Opt::StopWhenStuck);
    Ok(emu.dataize().0)
}

pub fn run_emulator(filename: &str) -> Result<i16> {
    let phi_code = fs::read_to_string(filename)?;
    emulate(&phi_code)
}

pub fn execute_program(args: &[String]) -> Result<i16> {
    assert!(args.len() >= 2);
    let filename: &str = &args[1];
    let result: i16 = run_emulator(filename)?;
    if args.len() >= 3 {
        let correct = args[2].parse::<i16>()?;
        assert_eq!(result, correct);
    }
    Ok(result)
}

pub fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2);
    let result = execute_program(&args);
    match result {
        Ok(value) => println!("Executor result: {value}"),
        Err(error) => eprintln!("Executor error: {error}"),
    }
}

#[test]
fn test_execute_program_with_valid_args() {
    let args = vec![
        "program_name".to_string(),
        "tests/resources/written_test_example".to_string(),
        "84".to_string(),
    ];
    let result = execute_program(&args);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 84);
}

#[test]
#[should_panic]
fn test_execute_program_with_invalid_args() {
    let args = vec!["program_name".to_string()];
    execute_program(&args).unwrap();
}

#[test]
fn executes_file_example() {
    let result = run_emulator("tests/resources/written_test_example");
    assert!(result.is_ok());
    assert_eq!(84, result.unwrap());
}

#[test]
fn executes_fibonacci_file() {
    let result = run_emulator("tests/resources/written_fibonacci_test");
    assert!(result.is_ok());
    assert_eq!(21, result.unwrap());
}

#[test]
fn executes_sum_file() {
    let result = run_emulator("tests/resources/written_sum_test");
    assert!(result.is_ok());
    assert_eq!(84, result.unwrap());
}
