fn main() {
let nums: [i32; 5] = [1, 2, 3, 4, 5];
let reff = &nums;
let slice = &nums[..3];

display(reff);
display(slice);


}
fn display(ref_arr: &[i32]){
    println!("{}", ref_arr.len());
}