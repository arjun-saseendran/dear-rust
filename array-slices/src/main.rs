fn main() {
    let mut nums: [i32; 5] = [1, 2, 3, 4, 5];
    let nums_slice: &mut [i32] = &mut nums[1..4];
    nums_slice[0] = 50;
    println!("Updated slice is: {:?}", nums_slice);
    println!("Original array ia: {:?}", nums)
}
