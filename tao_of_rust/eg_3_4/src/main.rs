fn reset(arr: &mut [u32]) {
    arr[0] = 5;
    arr[1] = 4;
    arr[2] = 3;
    arr[3] = 2;
    arr[4] = 1;
    println!("array length {:?}", arr.len());
    println!("reset array {:?}", arr);
}
fn main() {
    let mut arr = [1, 2, 3, 4, 5];
    println!("reset before: origin array {:?}", arr);
    {
        let mut_arr: &mut[u32] = &mut arr;
        reset(mut_arr);
    }
    println!("reset after: origin array {:?}", arr);

    println!("size of <&[u32;5]>: {}", std::mem::size_of::<&[u32;5]>());
    println!("size of <&mut [u32]>: {}", std::mem::size_of::<&mut [u32]>());
}
