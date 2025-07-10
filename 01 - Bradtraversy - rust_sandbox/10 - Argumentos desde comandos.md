## LINEA DE COMANDOS:
Mediante la función **args()** se pueden obtener los argumentos ingresados por línea de comandos al momento de ejecutarse el programa, por ejemplo mediante el comando *cargo run argumento1 argumento2, argumento3*, la función **args()** puede recuperar *la ruta* del programa que se ejecuta y lo ingresado en *argumento1 argumento2, argumento3*. En el caso de combinarlo con el método *.collect()*, permite almacenar la ruta y los argumentos en un vector, en el siguiente ejemplo, se hace en un vector de tipo *String*, y luego se imprime por consola:  

    let argumentos: Vec<String> = std::env::args().collect();
    println!("{:?}", argumentos);

En el caso de haber ejecutado el comando *cargo run papa pepe pipi popo pupu*, lo que se imprimirá por consola es el vector que contiene el *path* y los 5 argumentos, es decir, *["PATH", "papa", "pepe", "pipi", "popo", "pupu"]*. A continuación, continuando con el mismo vector, se presentan los datos en orden mediante una iteración en el rango de los índices del vector, comenzando por 1, puesto que el 0 es el path del archivo ejecutado: 

    println!("Argumentos ingresados:"); 
    for indice in 1..argumentos.len() {
        println!("- {}", argumentos[indice]);
    }

Este código nos dará como resultado:

    Argumentos ingresados:
    - papa
    - pepe
    - pipi
    - popo
    - pupu

