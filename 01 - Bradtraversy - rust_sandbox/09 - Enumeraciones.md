# ENUMERACIONES
En Rust, una enumeración *enum* es un tipo de dato personalizado que define un conjunto de variantes posibles, que pueden o no contener valores. Estas variantes pueden contener datos asociados, aumentando su flexibilidad.

Se crean mediante la sentencia *enum* y un nombre, siguiendo el mismo estilo capitalize que los *structs*, y dentro de llaves, se definen las posibles variantes. Por ejemplo, esta sería la sintaxis para crear un *enum* llamado *Movement*, con cuatro variantes de dirección: 
    
    enum Movement {
        Up,
        Down,
        Left,
        Right,
    }

El enum creado puede ser invocado a través de una función, en la cual se utiliza un  parámetro de entrada al que se le define el tipo de dato, en este caso, el tipo de dato será el nombre del *enum*, ya que el parámetro puede tomar el valor de alguna de las variables definidas. Se debe utilizar la sentencia *match* sobre el parámetro para evaluar cuál de las variantes *(patrones)* contiene, especificándolo mediante la sintaxis **enum::variante**, y asignarle, mediante **=>** un comportamiento *(expresión)* de salida: 

    fn move_avatar(m: Movement) {
        match m {
            Movement::Up => println!("Avatar moving up"),
            Movement::Down => println!("Avatar moving down"),
            Movement::Left => println!("Avatar moving left"),
            Movement::Right => println!("Avatar moving right"),
        }
    }

Finalmente, si se llama a esa función se le debe indicar como argumento la variante que se haya seleccionado, usando la misma sintaxis que el *patrón* definido **enum::variante**, y la función retornará la *expresión* que corresponda:

        move_avatar(Movement::Left);
        move_avatar(Movement::Up);
        move_avatar(Movement::Right);
        move_avatar(Movement::Down);
