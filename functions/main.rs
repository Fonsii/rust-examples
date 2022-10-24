fn say_hello(){
    let hello = "Hello World";
    println!("{}", hello);
}

fn return_function(letters: &str) -> String{
    let mut salute = String::from("Salute ");
    salute.push_str(letters);
    salute
}

fn main(){
    let hello = "Hello World in Main";
    say_hello();
    let salute = return_function(hello);
    println!("{}", salute);
    println!("{}", hello);
}