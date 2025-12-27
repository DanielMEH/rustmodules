struct User {
    name: String,
    edad: u8,
}
struct Rec {
    x: u32,
    y: u32,
}
struct Area {
    ancho: u32,
    alto: u32,
}

impl Area {
    pub fn new() -> Self {
        Self { ancho: 0, alto: 0 }
    }

    pub fn save_area(&mut self, ancho: u32, alto: u32) {
        self.alto = alto;
        self.ancho = ancho;
    }

    pub const fn read_value(&self) -> u32 {
        self.alto * self.ancho
    }
}

impl Rec {
    pub const fn area(&self) -> u32 {
        self.x * self.y
    }
}

struct Server<F, I>
where
    F: Fn() -> I + Send + Clone + 'static,
{
    factory: F,
}

impl<F, I> Server<F, I>
where
    F: Fn() -> I + Send + Clone + 'static,
{
    pub fn new(factory: F) -> Self {
        Server { factory }
    }
}
trait Productos {
    fn value(&self) -> String;
}

struct Leche;
struct Hold;

impl Productos for Leche {
    fn value(&self) -> String {
        return String::from("instancie object");
    }
}
impl Productos for Hold {
    fn value(&self) -> String {
        return String::from("instancie object");
    }
}

fn call_productos(product: impl Productos) {
    println!("INSTANCIE {}", product.value());
}

fn main() {
    call_productos(Hold);
    call_productos(Leche);
    Server::new(|| {
        let mut instacie_obj = Area::new();
        instacie_obj.save_area(4, 2);
        println!("area: {}", instacie_obj.read_value());
    });

    let new_area = Rec { x: 3, y: 7 };

    println!("handle func Rec {}", new_area.area());

    suma(4, 3);
    println!("Hello, world!");

    let mut name = String::from("Hola ");

    {
        let completename = has_string(&mut name);

        println!("{completename}");
    }
    name.push_str(" sss");
    let user = User {
        edad: 18,
        name: "Daniel".to_string(),
    };
    implfuncion(&name);

    generic_lifetime(&user);

    println!("name: {} - edad: {}", user.name, user.edad)
}

fn suma(x: i32, y: i32) -> i32 {
    println!("Sumatoria de informacion");
    y * x
}

fn has_string<'a>(value: &'a mut String) -> &'a mut String {
    value.push_str("Daniel");
    value
}

#[inline]
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

fn generic_lifetime<'a, T>(value: &'a T) -> &'a T {
    value
}
