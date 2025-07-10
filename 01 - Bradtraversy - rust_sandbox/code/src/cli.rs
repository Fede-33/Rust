use std::env;

pub fn run() {
    let argumentos: Vec<String> = env::args().collect();
    println!("{:?}", argumentos);

    println!("\nArgumentos ingresados:"); 
    for indice in 1..argumentos.len() {
        println!("- {}", argumentos[indice]);
    }
}