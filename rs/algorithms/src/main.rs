#![allow(dead_code)]

mod debruijn;
mod list;
mod scheduler;
mod comptime;

fn main() {
    debruijn::run();
}
