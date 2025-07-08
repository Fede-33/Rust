# FORMATEO DE MENSAJES:
Para cualquier mensaje que sea necesario mostrar por consola, se debe formatear mediante *placeholders* o identificando los espacios en los que se ignresará un dato. Por ejemplo, el formateo básico requiere que toda variable o expresión que se obtenga desde fuera de la cadena, se sitúe entre llaves en la posición indicada, y luego se defina:

    println!("Number: {}", 1);
    println!("{} tiene {} años", "Fede", 38);
    
Los argumentos posicionales, sirven para repetir algún dato en la impresión, sin necesidad de repetirlo en el código, definiendo a los datos como un conjunto y accediendo a ellos mediante su índice:

    println!(
        "{0} tiene {1} años y {0} es de {2}", 
        "Fede", 38, "Boca"
    );

Los argumentos con nombre, funcionan similar a los posicionales, pero asignándole un nombre a cada uno de ellos:

    println!(
        "{name} tiene {age} años", 
        name = "Fede", age = 38
    );

Existen también algunas características de formateo, como la posibilidad de convertir los datos para que sean mejor interpretados por el sistema:

    println!(
        "{} Formatted in Binary: {:b} / HEX: {:x} / Octal: {:o}", 10, 10, 10, 10
    );

Combinando los argumentos posicionales con la conversión de datos, la sintaxis podría ser:

    println!(
        "{0} Formatted in Binary: {0:b} / HEX: {0:x} / Octal: {0:o}", 10
    );

También puede usarse el placeholder genérico que puede almacenar una cantidad indefinida de datos, en esta caso, como una tupla:

    println!("{:?}", (10, "Pepe", true));

Finalmente, es posible incluir expresiones matemáticas básicas en el formateo:

    println!("{0}+{0}={1}", 10, (10+10));