use std::mem;


pub fn run (){
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    
    //re asignar valor
    numbers[2] = 20;

    // Agregar valor
    numbers.push(6);

    // Quitar Valor
    numbers.pop();

    //acceder a todo el arreglo
    println!("{:?}", numbers);
    
    //acceder a un valor
    println!("{}", numbers[0]);

    //length:
    println!("Vector Length: {}", numbers.len());

    //stack allocated - Memory capacity
    println!("{} bytes", mem::size_of_val(&numbers));

    //sclicing
    let slice: &[i32] = &numbers;
    println!("Copia completa {:?}", slice);

    let slice: &[i32] = &numbers[0..2];
    println!("Copia parcial {:?}", slice);    

    // Loop
    for x in numbers.iter(){
        println!("{}", x)
    }

    // Looop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers * 2: {:?}", numbers)

}