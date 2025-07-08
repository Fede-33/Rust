pub fn run(){
    let person: (&str, &str, i8) = ("Fede", "Boca", 38);

    println!("{} es de {} y tiene {}", person.0, person.1, person.2);
}