# PUNTEROS
Son elementos que apuntan a otro recurso en la memoria. Se producen cuando a un elemento le es asignado el valor o conjunto de valores de otro elemento preexistente. La diferencia radica en que funciona de distinta manera para elementos primitivos como los **arrays**, que para elementos no primitivos como los **vectors**, que no son primitivos.

## PRIMITIVE:
Si hay un arreglo preexistente, puede definirse un segundo arreglo y asignarle el valor del primero. Esto simplemente duplicará el arreglo y dejará el primitivo sin modificar:

    let arr1:[&str; 3] = ["aa", "bb", "cc"];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));

## NON-PRIMITIVE: 
En este caso, si se asigna valores a otra variable, la variable original será vaciada, es decir, ya no contendrá esa información. Es por ello que se debe trabajar con referencias a esa variable original, las referencias se logran con la sintaxis de ampersand *&*, para apuntar a ese recurso en la memoria:

    let vec1: Vec<i32> = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1, vec2));