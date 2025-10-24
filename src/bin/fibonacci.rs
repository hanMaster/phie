// SPDX-FileCopyrightText: Copyright (c) 2022 Yegor Bugayenko
// SPDX-License-Identifier: MIT

extern crate phie;

use phie::data::Data;
use phie::emu::{Emu, Opt};
use phie::error::Result;
use std::env;

pub fn fibo(x: Data) -> Result<Data> {
    let mut emu: Emu = format!(
        "
        ν0(𝜋) ↦ ⟦ 𝜑 ↦ ν2(𝜋) ⟧
        ν1(𝜋) ↦ ⟦ Δ ↦ 0x{:04X} ⟧
        ν2(𝜋) ↦ ⟦ 𝜑 ↦ ν3(ξ), 𝛼0 ↦ ν1(𝜋) ⟧
        ν3(𝜋) ↦ ⟦ 𝜑 ↦ ν13(𝜋) ⟧
        ν5(𝜋) ↦ ⟦ Δ ↦ 0x0002 ⟧
        ν6(𝜋) ↦ ⟦ λ ↦ int-sub, ρ ↦ 𝜋.𝜋.𝛼0, 𝛼0 ↦ ν5(𝜋) ⟧
        ν7(𝜋) ↦ ⟦ Δ ↦ 0x0001 ⟧
        ν8(𝜋) ↦ ⟦ λ ↦ int-sub, ρ ↦ 𝜋.𝜋.𝛼0, 𝛼0 ↦ ν7(𝜋) ⟧
        ν9(𝜋) ↦ ⟦ 𝜑 ↦ ν3(ξ), 𝛼0 ↦ ν8(𝜋) ⟧
        ν10(𝜋) ↦ ⟦ 𝜑 ↦ ν3(ξ), 𝛼0 ↦ ν6(𝜋) ⟧
        ν11(𝜋) ↦ ⟦ λ ↦ int-add, ρ ↦ ν9(𝜋), 𝛼0 ↦ ν10(𝜋) ⟧
        ν12(𝜋) ↦ ⟦ λ ↦ int-less, ρ ↦ 𝜋.𝛼0, 𝛼0 ↦ ν5(𝜋) ⟧
        ν13(𝜋) ↦ ⟦ λ ↦ bool-if, ρ ↦ ν12(𝜋), 𝛼0 ↦ ν7(𝜋), 𝛼1 ↦ ν11(𝜋) ⟧
        ",
        x
    )
    .parse()?;
    emu.opt(Opt::LogSnapshots);
    emu.opt(Opt::StopWhenTooManyCycles);
    emu.opt(Opt::StopWhenStuck);
    Ok(emu.dataize().0)
}

pub fn main() -> Result<()> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let input = args[1].parse()?;
    let cycles = args[2].parse()?;
    let mut total = 0;
    let mut f = 0;
    for _ in 0..cycles {
        f = fibo(input)?;
        total += f;
    }
    println!("{}-th Fibonacci number is {}", input, f);
    println!("Sum of results is {}", total);
    Ok(())
}

#[cfg(test)]
use simple_logger::SimpleLogger;

#[test]
fn calculates_fibonacci() {
    SimpleLogger::new().init().unwrap();
    assert_eq!(21, fibo(7).unwrap())
}
