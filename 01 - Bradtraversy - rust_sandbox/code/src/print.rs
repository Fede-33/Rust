pub fn run(){
    //Imprimir en la consola
    println!("Imprimir desde print.rs");
    
    //Formateo básico
    println!("Number: {}", 1);
    println!("{} tiene {} años", "Fede", 38);
    
    //Positional Arguments
    println!(
        "{0} tiene {1} años y {0} es de {2}", 
        "Fede", 38, "Boca"
    );

    //Named Arguments
    println!(
        "{name} tiene {age} años", 
        name = "Fede", age = 38
    );

    //Placeholder traits
    println!(
        "{} Formatted in Binary: {:b} / HEX: {:x} / Octal: {:o}", 10, 10, 10, 10
    );
    println!(
        "{0} Formatted in Binary: {0:b} / HEX: {0:x} / Octal: {0:o}", 10
    );

    //Placeholder for debug trait
    println!("{:?}", (10, "Pepe", true));

    //Basic Math
    println!("{0}+{0}={1}", 10, (10+10));
}