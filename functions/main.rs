fn say_hello(){
    let hello = "Hello World";
    println!("{}", hello);
}

fn main(){
    let hello = "Hello World in Main";
    say_hello();

    println!("{}", hello);
}