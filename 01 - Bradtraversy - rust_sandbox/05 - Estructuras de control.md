# ESTRUCTURAS DE CONTROL:
Las estructuras de control son fundamentales para determinar el flujo de ejecución de un programa. Permiten que el programa tome decisiones y repita acciones basándose en ciertas condiciones. Sin ellas, simplemente se ejecutarían las instrucciones de forma secuencial de principio a fin.

## CONDICIONALES:
Sentencias que verifican una condición y actúan en consecuencia. La sintaxis es similar a otros lengujaes, inclusive con sentencias como *and* (&&) y *or* (||). La instrucción que se debe o no ejecutar, se encierra entre llaves:

    let age: u8 = 22;
    let check_id: bool = true;
    let knows_person_of_age = true;

    if age >= 21 && (check_id || knows_person_of_age) {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave");
    } else {
        println!("Bartender: I need to see an ID");
    }

En el ejemplo anterior, se verifican tres condiciones. Primero el bartender debe preguntar la edad, si dice ser mayor a 21, debe verificar el documento o conocer a la persona por su edad. En caso de que la persona sea menor a 21, no importa si tiene un ID o si es conocido, no podrá pedir una bebida. En caso de que sea mayor de 21, pero no sea conocida del bartender o tenga un ID consigo, tampoco podrá ordenar.

Existe una forma sintética de utilizar un condicional para definir una variable, y es declarando la condición al asignar el valor de la variable, y entre llaves incluir el código que se ejecutaría en caso de cumplirse la condición, o de no cumplirse: 

    let is_of_age = if age >= 21 { true } else { false };
    println!("Is Of Age: {}", is_of_age)

## ITERACIONES:
Se utilizan para repetir una instrucción bajo ciertas circunstancias:

### INFINITE LOOP:
El tipo findamental de iteración en Rust, se debe complementar con una condición para evitar que justamente se atrape al programa en una repetición infinita. En el siguiente ejemplo se declara la sentencia *loop* y se asigna un contador para que aumente en cada repetición. El condicional del final, interrumpirá la ejecución cuando el contador llegue a 20:
    
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 20 {
          break;
        }
    }

### WHILE LOOP:
La iteración se repetirá mientras que una condición esté cumplida. Por ejemplo, para realizar un *fizzfuzz*, en el que se itera en una cierta cantidad de números, imprimiendo *fizz* si es múltiplo de 3, *fuzz* si es múltiplo de 5, *fizzfuzz* si es múltiplo de ambos, o directamente el número si no se cumple ninguna condición:

    while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", count);
        
        count += 1;
    }

### FOR RANGE LOOP:
Esta estructura iterará en el rango especificado, puede ser numérico como un conjunto de datos ordenados. Por ejemplo, realizar el mismo desafío *fizzfuzz* con *for*:

    for x in 0..100 {
        if x % 15 == 0 {
            println!("fizzbuzz");
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", x);
        }
    }
