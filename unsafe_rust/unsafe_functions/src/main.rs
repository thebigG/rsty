use std::slice;

fn main() {
    let mut list = vec![1, 2, 3, 4, 5];
    let wild_slice = split_at_mut(list.as_mut_slice(), 3);

    println!("{:?}", wild_slice);
}

/// The idea is that we create  a safe abstraction for our unsafe code, which is dealing with raw wild pointers
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]){
    let len = values.len();
    let  ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe{
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut( ptr.add(mid), len - mid)
        )
    }
}
