fn main() {
    
    let result = factorial(5);
    println!("The result is: {result}");

}
fn factorial(num: i32) -> i32{
    if num <= 0 {
        1
    }else {
        num * factorial(num - 1 )
    }
}