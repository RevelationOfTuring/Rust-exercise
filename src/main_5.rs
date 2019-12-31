/*
Slices are similar to arrays, but their size is not known at compile time.
Instead, a slice is a two-word object, the first word is a pointer to the data, and the second word is the length of the slice.
The word size is the same as usize, determined by the processor architecture eg 64 bits on an x86-64.
Slices can be used to borrow a section of an array, and have the type signature &[T].
*/


fn main_5() {
    let arr = [1024; 100];
    // convert array to slice: &arr is a slice
    analyze_slice(&arr);
    println!("{:?}", arr.get(99));
}

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}