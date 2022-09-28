use std::{cmp::min, mem::size_of};

unsafe fn memcpy<'a, T: Copy>(dest: &'a mut [u8], src: &[T]) -> &'a [u8] {
    let len = min(dest.len(), src.len() * size_of::<T>());
    let bytes = std::slice::from_raw_parts(src.as_ptr() as _, len);

    dest[..len].copy_from_slice(bytes);
    return &dest[..len];
}

fn main() {
    type Arr = [i32; 4];

    let ints: Arr = [1, 2, 3, 4];
    let mut buf = [0; size_of::<Arr>()];
    
    println!("{:?}", unsafe { memcpy(&mut buf, &ints) });
}

/*
char *buffer = (char *) malloc(512);
free(buffer);
memcpy does copy. In contrast,
the [T]::copy_from_slice that uses memcpy under the hood 
but requires two slices of the same type and same length
*/