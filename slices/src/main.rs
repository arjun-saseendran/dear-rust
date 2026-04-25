fn main() {
    let nums: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &nums[..3];
    println!("Slice data is: {:?}", slice);
    let small_slice: &[i32] = &slice[1..2];
    println!("Small slice data is: {:?}",small_slice);
}
