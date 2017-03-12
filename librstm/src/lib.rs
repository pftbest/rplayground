#![no_std]
#![feature(asm)]
#![feature(lang_items)]
#![feature(const_fn)]
#![feature(compiler_builtins_lib)]

extern crate compiler_builtins;

pub mod vectors;
pub mod startup;
pub mod print;
pub mod stdio;
mod rdi;
