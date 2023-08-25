// Day 7 of Rust , I learnt about memory. memory is stored using binary
//  ie Bits(0 or 1), Bit is the smallest uint of memory that can be stored.
// computer hardware is optimized to work with bytes
// 1 byte == 8 bits.

// all data in memory has a "memory address", these addresses are used to locate data in memory.
// addresses are always the same but data at the addresses changes, however while
// writing programs, we don't usually worry about addresses because variables takes care of these for us.

// memory Offsets can be used to locate item at a specific address.
// offsets always begin at 0 and they represent the number of bytes
// away from the original address, similar to adresses, we don't deal with
// offsets directly, instead we deal with "indexes" and the compiler calculates
// how many bytes away we are from the original address based on the index
// essencially in programs, variable names automatically takes care of the
// address and the index will automatically get mapped to an offset by the compiler.

fn main() {
    println!("Hello, world!");
}
