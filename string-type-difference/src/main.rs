fn main() {
let first_name = String::from("Maria");
let second_name = "Issac";

display(&second_name);
// view(&second_name);
}

fn display(name: &str){
    println!("Her name is: {name}");
}

fn view(name: &String){
    println!("Her name is: {name}");
}