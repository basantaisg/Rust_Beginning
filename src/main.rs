fn main(){
    println!("Basantalovesjs!"); // this is a macro call. if ! is used before function call then its macro! and macro means this code will get broken down into more after soemtime!

    let ans = sum(1,2);
    println!("{}", ans)

}

fn sum (a: u32, b: u32) -> u32{
    return a + b;
}