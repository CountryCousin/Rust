// Day 36 of Rust, All data has memory address and these addresses determine the
// location of data in memory.
// Offsets can be used to access adjacent addresses(these are also called inexes/indices)
// Two ways memory is manages in Rust are Stack and the Heap
// In the  stack, data is placed sequentially, there is limited available space(so must know the data size beforehand),
// all variables created in a program are stored in the stack(not all data). stacks are
// fast to work with because they use Offsets to access data, its only possible to remove or add
// data from the top of stack.

// In the Heap, Data is placed algorithmically, its slower than the stack but with unlimited thoeretical space
// The heap uses pointers to point to where the memory is, pointers are fixed size depending on the achtecture of the computer.
// 64-bit pc have pionter size of 64-bits.Rust data type for pionter is usize.
// vectors, HashMaps and dynamically sized collections are stored on the heap

struct Entry {
    id: i32,
}
fn main() {
    let data = Entry { id: 5 }; //creating an instance of Entry
    let data_ptr: Box<Entry> = Box::new(); //"Box<>" means we are putting the data on the heap.
    let data_stack = *data_ptr; //moves the data back to the stack using "*"
}
