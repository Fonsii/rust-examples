fn main(){
    let number = 5;
    println!("The value of the number is: {}", number);

    //number = 6; // Error because x can not be changed

    let mut number_two = 10;
    println!("The value of the number_two is: {}", number_two);

    {
        let number_two = 15;
        println!("The value of the number_two in other scope is: {}", number_two);
    }

    number_two = 11;
    println!("The value of the number_two is: {}", number_two);
    println!();
    println!("Sum of number and number_two is: {}", number + number_two);
    println!("Sub of number and number_two is: {}", number - number_two);
    println!("Mul of number and number_two is: {}", number * number_two);
    println!("Div of number and number_two is: {}", number_two / number);
    println!("Mod of number and number_two is: {}", number_two % number);

    let hello = "Hello";
    let world = String::from("World");
    let mut salute = "Salute"; //Error mut String 
    
    println!("{} {}", hello, world);
    println!("Len hello: {}", hello.len());
    println!("Len salute: {}", salute.len());

    salute = "Salute World";
    println!("Len salute: {}", salute.len());
}