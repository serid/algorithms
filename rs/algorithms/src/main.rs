#![allow(dead_code)]

mod debruijn;
mod list;
mod scheduler;
mod comptime;
mod cont;

fn main() {
    cont::run();
}
