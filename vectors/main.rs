fn main(){
    let vector = vec![1, 2, 3, 4, 5];
    println!("The third element of the vector is {}", vector[2]);

    for i in &vector {
        println!("{}", i);
    }
}