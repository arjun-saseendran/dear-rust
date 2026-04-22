fn main() {

    let full_name = String::from("Angel Maria Issac");
    let first_name = &full_name[0..6];
    println!("Her first name is: {first_name}");
    let second_name = &full_name[6..12];
    println!("Her second name is: {second_name}");
    let third_name = &full_name[12..17];
    println!("Her third name is: {third_name}");
}
