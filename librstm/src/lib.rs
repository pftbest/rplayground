#![no_std]
#![feature(asm)]
#![feature(lang_items)]
#![feature(const_fn)]

extern crate compiler_builtins_snapshot;

pub mod vectors;
pub mod startup;
pub mod print;
pub mod stdio;
mod rdi;
