fn main() {
let  name = "maria";
println!("The length of the name string is:{}", name.len());
let name_slice = &name[1..2];
println!("The slice length is:{}", name_slice.len());
let emoji = "😁";
println!("The length of the emoji string is:{}", emoji.len());
// Shouldn't do this.e
// let emoji_slice = &emoji[1..2];
// println!("The length of the emoji slice is:{}", emoji_slice);
}
