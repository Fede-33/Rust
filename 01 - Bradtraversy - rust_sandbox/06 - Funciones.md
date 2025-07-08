# FUNCIONES:
Se utilizan para crear bloques de código que luego puedan ser reutilizados.

## DECLARACIÓN Y LLAMADA:
Para declarar una función en un script local se utiliza la sentencia *fn* y cada uno de los parámetros definidos debe tener especificado el tipo de dato que contendrá:

    fn greeting(greet: &str, name: &str) {
        println!("{} {}, nice to meet you!", greet, name);
    }

Esta función simplemente saluda a una persona y solicita dos cadenas de texto para ejecutarse. Y puede ser llamada de la siguiente manera:

    greeting("Hello", "Jane");

También pueden definirse funciones que retornen un valor, en vez de imprimirlo. Por ejemplo una función de suma de dos enteros, que retorne otro entero, especificado mediante la sintaxis *->* y la operación a realizar entre llaves:

    fn sumar(n1: i32, n2: i32) -> i32 {
        n1 + n2
    }

Para llamar a esta función también es posible definir otra variable y ligarla a esta función, por ejemplo:

    let sumatoria = sumar(5, 5);
    println!("la suma es: {}", sumatoria);

Otra forma más compacta de definir funciones es mediante la sintaxis de *pipelines* en una definición de otra variable, por ejemplo:

    let add_nums = |n1: i32, n2: i32| n1 + n2;
    println!("C Sum: {}", add_nums(3, 3));

Esta sintaxis es más sencilla cuando no es necesario reutilizar la función fuera de esa variable.