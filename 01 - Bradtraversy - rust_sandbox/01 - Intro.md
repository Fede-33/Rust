# RUST
Lenguaje de programación de nivel sistema, para desarrollo de aplicaciones que no interactúen directamente con el usuario, como controladores o web assembly. Para su instalación y configuración en *linux* se utililza el comando **curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y** para instalar:

* **rustc** el lenguaje de programación propiamente dicho.
* **rustup** el administrador de versiones de Rust.
* **cargo** el administrador de librerías y paquetes.

Trabajando en *VSCode* es conveniente instalar una extensión de resaltado de sintaxis de Rust.

## MANEJO DE ARCHIVOS:
Crear un archivo simple *.rs* de Rust puede hacerse mediante el comando *touch archivo.rs*. Al ser este un archivo principal, es necesario que tenga un punto de entrada, uan función principal. En el caso actual, simplemente imprimirá *Hello World!*:

    fn main () {
        println!("Hello World!");
    }

Para compilar este código, se ejecuta el comando *rustc archivo.rs* que crea un ejecutable en la misma carpeta.

Este proceso simplificado no es la forma más corriente de iniciar un proyecto de Rust. En ese caso es necesario inicializar un directorio con el comando *cargo init* estando dentro de la carpeta deseada. O puee crearse la carpeta e inicializarla mediante el comando *cargo new nombre*. Esto creará un archivo *Cargo.toml* que contiene los detalles técnicos de la aplicación iniciada, y una carpeta *.gitignore* para evitar subir archivos basura a GIT. También se genera una carpeta *src* con un archivo Rust exactamente igual al que se definió anteriormente.

Para compilar este código se utiliza el comando *cargo run*, que además lo ejecuta. El archivo ejecutable está dentro de la carpeta *target/debug*. Sin embargo, puede compilarse sin ejecutar mediante el comando *cargo build*. Y si quiere compilarse una versión de distribución, se necesista ejecutar el comando *cargo build --release* que creará los archivos ejecutables en la carpeta *target/release*

## MODULARIDAD:
Para ejemplificar, se crea un archivo *print.rs* en la misma raiz que el archivo *main.rs*. En el archivo print.rs, se define la siguiente función *run()*:

    pub fn run(){
        //Imprimir en la consola
        println!("Imprimir desde print.rs");
    }

Luego, en el archivo *main.rs* se importa el módulo *print* y se llama a la función *run()* mediante la siguiente sintaxis:

    mod print; //Importa el módulo

    fn main() {
        print::run(); //Llama a la función
    }
