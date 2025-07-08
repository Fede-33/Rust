# TIPOS DE DATOS:
Rust es un lenguaje de tipos estáticos, es decir, que antes de la compilación se debe tener definidos todos los tipos de datos almacenados. Muchas veces, el compilador puede inferir el tipo de dato que se intenta usar.
En rust existen los siguientes tipos primitivos de datos:

## INTEGER:
Números enteros que pueden ser regulares (i) o unsigned (u), los últimos no contemplan valores negativos. Para definirlos, se utiliza la letra que los identifica junto con la cantidad de bits que ocupan en memoria:

* i8
* u8
* i16 
* u16 
* i32 
* u32 
* i64 
* u64 
* i128 
* u128

### MÁXIMO:
Para saber el número máximo que puede tomar un tipo de dato, se puede utilizar la bilbioteca *std*, invocando el tipo de dato y la función *MAX* asociada:

    println!("Max i32: {}", std::i32::MAX);

## FLOATS:
Números fraccionarios, que se definen según la cantidad de bits que ocupan:

* f32
* f64

## BOOLEANS:
Los datos tipo **BOOL** pueden tomar valores de *true* o *false*. Al definirlos el lenguaje generalmente interpreta que se trata de este tipo de dato, por ejemplo:

    let is_active: bool = true;

Pero si queremos obtenerlo desde una expresión, es conveniente especificar el tipo de dato en la definición:

    let is_greater: bool = 10 < 5;

## CHARACTERS:
El tipo **CHAR** solo puede incuir un caracter, que pueden ser unicode */u{}*, como emojis:

    let a1 = 'a';
    let face = '\u{1F600}';

## STRINGS:
Las cadenas de texto son tipos de datos menos frecuentes en Rust que en otros lenguajes de más alto nivel. Existen dos tipos de cadenas:

* **str primitiva:** Son cadenas inmutables con un espacio de memoria fijado. Cuando se define una cadena de forma directa, se crea este tipo de dato:

        let hello = "Hello ";

* **String:** Cadena modificable, con uso de memoria variable y con mayor control del desarrollador. Y con la posibilidad de que el dato persista más tiempo (heap allocated). Para definirla, se necesita declarar el tipo de dato, además de hacerlo mutable:

        let mut hello = String::from("Hello ");

Para obtener la extensión de una cadena, puede usarse la función *.len()* 

    println!("Length: {}", hello.len());

Para agregar un único caracter al final de una cadena se utiliza la función *.push()*: 
    
    hello.push('W');

Para agregar una cadena al final, se usa *.push_str()*:
    
    hello.push_str("orld!");

Puede obtenerse la capacidad en bytes que puede almacenar, mediante la función *.capacity()*:

    println!("Capacity: {}", hello.capacity());

Puede verificarse si es una cadena vacía con la función *.is_empty()*:

    println!("Is Empty: {}", hello.is_empty());

Para verificar si un caracter o secuencia de caracteres se encuentra dentro de la cadena:

    println!("Contains 'World' {}", hello.contains("World"));

Mediante la función *.replace()* es posible reemplazar una serie de caracteres por otra: 
    
    println!("Replace: {}", hello.replace("World", "There"));

Una cadena puede ser dividida, por ejemplo, en sus espacios en blanco, para formar un conjunto de datos iterable, con cada una de sus palabras. Por ejemplo, con la función *split_whitespace()* se crea la tupla de elementos, cada uno siendo una de las palabras. Y cómo se úbica dentro de una estructura de loop for, la variable *word* va tomando el valor de cada uno de los elementos de esa tupla, y luego se imprimen por separado:

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

Puede ser definida una **String** sin contenido, pero especificando la capacidad de almacenamiento:

    let mut s = String::with_capacity(10);

Otra posibilidad es la de verificar una condición mediante un *assertion*. Mediante la función *assert_eq!* podemos comparar dos elementos. Por ejemplo, habiendo definido **s** en la línea anterior, se verificará que tenga una capacidad de *10*. Esta forma de verificación no detendrá la ejecución del programa en caso de ser falsa, sino que retornará un mensaje por consola en tiempo de ejecución:
    
    assert_eq!(10, s.capacity());


## TUPLES:
Es un conjunto de datos no ordenados, de diferentes tipos y 12 elementos como máximo. Para crear una tupla se debe definir como tal, definir los tipos de datos que va a contener, y luego asignarle los valores:

    let persona: (&str, &str, i8) = ("Fede", "Boca", 38);

Luego, para acceder a esos datos, se utilizan los índices internos, mediante sintaxis de punto:

    println!("{} es de {} y tiene {}", persona.0, persona.1, persona.2);

## ARRAYS:
Conjunto de datos ordenados.