fn main() {
let name = "Angel Maria";
let first_name = &name[..6];
println!("Her first name is: {first_name}");
let last_name = &name[6..];
println!("Her last name is: {last_name}");
let slice_name = &name[..];
println!("The slice full name is: {slice_name}");
}

