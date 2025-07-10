# ESTRUCTURAS
Son similares a las clases en cuanto a que poseen atributos y funciones propias. 

## ESTRUCTURA TRADICIONAL:
Se definen mediante la sentencia *struct* y su nombre se escribe con mayúscula inicial por convención. Dentro de las llaves se define cada uno de sus atributos, especificando el tipo de dato que contendrá. En el caso en que sea necesario definir un tipo de dato *&str*, se debe tener en cuenta que se trata de una referencia, y al estar dentro de una estructura, debe tener asignado un *timelimit parameter*. Esto sucede porque esa referencia debe tener un límite de tiempo para ser válida, y se define mediante *<'a>* siendo cualquier letra minúscula para un *timelimmit parameter standard*, es decir, genérico y definido por el usuario:

    struct Color<'a> {
        name: &'a str,
        red: u8,
        green: u8,
        blue: u8
    }

Al momento de crear una instancia de este tipo de estructura, se debe definir como una variable, especificando entre llaves cada uno de los valores que contendrán los atributos. Recordando que, en caso de ser un objeto mutable, se debe definir con la sentencia *let mut*:

    let mut rojo = Color {
        name: "Rojo",
        red: 255, 
        green: 0, 
        blue: 0
    };

    rojo.red = 240;

    println! ("Código RGB de {}: {}, {}, {}", rojo.name, rojo.red, rojo.green, rojo.blue);

## ESTRUCTURA DE TUPLA:
Se define como si fuera una tupla de tipos de dato, teniendo en cuenta igualmente el *lifetime parameter* para referencias:

    struct Socio<'b> (&'b str, &'b str, u16);

Para crear una instancia se define como una variable, especificando los valores que contendrán los argumentos, que se llaman a través de los índices de la tupla, con sintaxis de punto:

    let s = Socio("Pepe", "Pérez", 12538);
    println! ("Asociado: {} {}, nro. {}", s.0, s.1, s.2 );

## ESTRUCTURA CON FUNCIONES:
A diferencia de otros leguajes, las funciones se definen en un bloque separado que los atributos. Mientras que estos se definen dentro de la sentencia *struct* las funciones, o comportamientos, lo hacen dentro de la sentencia *impl* especificando el nombre de la estructura: 

    struct Person {
        first_name: String,
        last_name: String
    }

En el siguiente ejemplo, se definen cuatro funciones: 

    impl Person{
        fn new(first: &str, last: &str) -> Person {
            Person {
                first_name: first.to_string(),
                last_name: last.to_string()
            }
        }

        fn full_name(&self) -> String {
            format!("{} {}", self.first_name, self.last_name)
        }
        
        fn set_last_name(&mut self, last:&str){
            self.last_name = last.to_string()
        } 
        
        fn name_to_tuple(self) -> (String, String){
            (self.first_name, self.last_name)
        }
    }

* **new:** que recibe dos parámetros como *&str* y retorna una instancia de *Person*, compuesta por los dos atributos de la estructura original. Se tiene en cuenta que los datos ingresados deben ser transformados mediante la función *to_string()* para coincidir con el tipo de dato que se definió en la sentencia *struct*.
    
    De esta manera, al momento de crear una instancia de esta estructura se realizaría a través del método *new()* con sintaxis de *::* especificando los valores de los atributos:

        let pers_1 = Person::new("John", "Doe");
    
    Al momento de obtener los datos, se especifica el nombre del objeto y del atributo mediante una sintaxis de punto:

        println!("Nombre: {}, Apellido: {}", pers_1.first_name, pers_1.last_name);

* **full_name:** que recibe la referencia *&self* a la misma estructura y retorna un tipo de dato *String*, que es una cadena formateada de los dos argumentos concatenados.

    Al ser un método que solo retorna información, se llama mediante sintaxis de punto:

        println!("Nombre completo: {}", pers_1.full_name());
    
* **set_last_name:** que permite modificar el apellido, por lo que recibe el argumento *&mut self* y un *&str*, para reasignarlo al propio atributo de la estructura.

    Para llamar a este método, también mediante sintaxis de punto, se debe especificar el atributo que será modificado:

        pers_2.set_last_name("Gómez");
    
    Obviamente, que no retornará ningún valor, por lo que será necesario llamar a otro método de la estructura, como *full_name* definido anteriormente.

* **name_to_tuple:** que recibe recibe parámetros de sí mismo, y retorna una tupla de dos *String*, esa tupla se forma con sus propios atributos.

    Siendo este método de retorno de un dato del tipo tupla, se llama mediante sintaxis de punto, pero en el formateo se debe usar un placeholder en formato debug *{:?}*:
    
        println!("Nombre en tupla: {:?}", pers_2.name_to_tuple())