// Traditional struct
struct Color<'a> {
    name: &'a str,
    red: u8,
    green: u8,
    blue: u8
}

// Tuple Struct
struct Socio<'b> (&'b str, &'b str, u16);


// Struct con funciones
struct Person {
    first_name: String,
    last_name: String
}

impl Person{
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }
    //Full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    // Set last name
    fn set_last_name(&mut self, last:&str){
        self.last_name = last.to_string()
    } 
    //Name to tuple
    fn name_to_tuple(self) -> (String, String){
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    //Trad struct
    let mut rojo = Color {
        name: "Rojo",
        red: 255, 
        green: 0, 
        blue: 0
    };

    rojo.red = 240;

    println! ("Código RGB de {}: {}, {}, {}", rojo.name, rojo.red, rojo.green, rojo.blue);

    //tuple struct
    let mut s = Socio("Pepe", "Pérez", 12538);

    s.0 = "José";

    println! ("Asociado: {} {}, nro. {}", s.0, s.1, s.2 );

    //struct fn
    let pers_1 = Person::new("John", "Doe");
    println!("Nombre: {}, Apellido: {}", pers_1.first_name, pers_1.last_name);
    println!("Nombre completo: {}", pers_1.full_name());
    let mut pers_2 = Person::new("Juana", "Pérez");
    println!("Nombre completo: {}", pers_2.full_name());
    pers_2.set_last_name("Gómez");
    println!("Nombre completo: {}", pers_2.full_name());
    println!("Nombre en tupla: {:?}", pers_2.name_to_tuple())


}