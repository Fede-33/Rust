# VARIABLES
En Rust las variables son inmutables por defecto. Además, al ser un lenguaje de bloques, las variables en Rust están confinadas al bloque en que se han definido.

PAra definir una variable se usa la sintaxis *let*, y esta será inmutable:

    let name = "Fede"; //Variable inmutable por default

Si se quiere definir una variable que pueda ser reasignada, se utiliza la sintaxis *let mut*:

    let mut age = 37;

Para definir una constante, se utiliza la sintaxis *const* se especifica el nombre de la variable (en mayúsculas por convención) y luego debe especificarse el tipo de dato que se almacenará, en el siguiente caso se define *i32* integer de 32 bits:

    const ID: i32 = 30564897; 

Pueden definirse múltiples variables mediante la siguiente sintaxis. Téngase en cuenta que se debe especificar en cada una de las variables si será mutable:
    
    let(nombre, mut edad, doc) = ("Fede", 38, 46578923); 