pub fn run(){
    let name = "Fede"; //Variable inmutable por default
    let mut age = 37; //variable mutable
    println!("My name is {} and I am {}", name, age);
    
    age = 38;
    println!("My name is {} and I am {}", name, age);

    const ID: i32 = 30564897; // Constantes
    println!("My name is {} and I am {}. ID nº {}", name, age, ID);

    let (nombre, mut edad, doc) = ("Fede", 38, 46578923); //Define múltples variables
    println!("My name is {} and I am {}. ID nº {}", nombre, edad, doc);
}