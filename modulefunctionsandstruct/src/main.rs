fn main() {
    suma(4, 3);
    println!("Hello, world!");

    let mut name = String::from("Hola ");

    {
        let completename = has_string(&mut name);

        println!("{completename}");
    }
    name.push_str(" sss");
    implfuncion(&name);
}

fn suma(x: i32, y: i32) -> i32 {
    println!("Sumatoria de informacion");
    y * x
}

fn has_string<'a>(value: &'a mut String) -> &'a mut String {
    value.push_str("Daniel");
    value
}

fn implfuncion(value: &String) -> impl std::fmt::Display + Clone {
    //!
    //! ```
    //! Tipo de Retorno	Uso Principal
    //!impl Debug	Para inspección rápida.
    //!impl Display	Para mostrar al usuario final.
    //!impl Iterator<Item = T>	Para cadenas de procesamiento de datos.
    //!impl Future<Output = T>	Para programación asíncrona (async).
    //!impl Fn(i32) -> i32	Para devolver lógica o funciones.
    //! ```
    println!("{value}");
    return value;
}
