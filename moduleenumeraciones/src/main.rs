struct Sedan;
struct Moto;
pub mod utils;

trait LandDrive {
    fn driver(&self);
}

impl LandDrive for Sedan {
    fn driver(&self) {
        println!("sedan is derive")
    }
}

impl LandDrive for Moto {
    fn driver(&self) {
        println!("sedan is derive")
    }
}

fn drive_all<'a, T: LandDrive>(vehiculo: &'a T) {
    vehiculo.driver();
}

trait Describe {
    fn describe(&self) -> String;
}

struct Caja<T> {
    contenido: T,
}

impl<T> Caja<T> {
    fn nueva(valor: T) -> Self {
        Self { contenido: valor }
    }
}

impl<T: std::fmt::Display> Describe for Caja<T> {
    fn describe(&self) -> String {
        format!("Value devuelve {}", self.contenido)
    }
}

fn imprimir_caja<T: Describe>(object: T) {
    println!("Imprimiento contenido de la caja {}", object.describe())
}

fn main() {
    // let dieta_carnivoro = utils::utils::Animal::new_animal("Caballo", "Carnivoro");
    // println!(
    //     "nombre: {} dieta:{}",
    //     dieta_carnivoro.name, dieta_carnivoro.dieta
    // );

    // let new_animal_herviboro = utils::utils::
    //   "Caballo",
    //    "(pasto,geno,paha) que debe ser la base del (60%-80%)",
    // );
    // utils::utils::comer(new_animal_herviboro);

    //   let new_animal_carnivoro = utils::utils::Carne::new_animal("leon", "90% carne y 10% de aguia");
    //   utils::utils::comer(new_animal_carnivoro);

    let caja: Caja<String> = Caja::nueva(String::from("Zapatos"));
    let caja: Caja<String> = Caja::nueva(String::from("Pantalones"));
    imprimir_caja(caja);

    drive_all(&Moto);
    drive_all(&Sedan);

    let value: Option<String> = Some(String::from("handle"));
    println!("Hola mundo {:?}", assert_eq!(value.is_some(), true));
    realizar_operacion(Operacion::Smar(2, 4));
    realizar_operacion(Operacion::Restar { a: 2, b: 6 });
    realizar_operacion(Operacion::Others(Valor { x: 9, y: 1434 }));

    let calls_mensages: Vec<Mensage> = vec![Mensage::Salir, Mensage::Entrar];

    for i in calls_mensages {
        i.call();
    }

    let message_salir = Mensage::Salir;

    println!("Saliendo {}", message_salir.salir());
}

struct Valor {
    x: i32,
    y: i32,
}

enum Operacion {
    Smar(i32, i32),
    Restar { a: i32, b: i32 },
    Others(Valor),
}

enum Mensage {
    Salir,
    Entrar,
}

impl Mensage {
    fn call(&self) {
        match self {
            Mensage::Entrar => println!("Mensage entrar"),
            Mensage::Salir => println!("Saliendo"),
        }
    }
}
macro_rules! handle {
    ($expression:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {
        match $expression {
            $pattern $(if $guard)? => true,
            _ => false
        }
    };
}

impl Mensage {
    fn salir(&self) -> bool {
        handle!(self, Mensage::Salir)
    }
}

fn realizar_operacion(operacion: Operacion) {
    match operacion {
        Operacion::Smar(num_a, num_b) => println!("{}", num_a + num_b),
        Operacion::Restar { a, b } => println!("{}", a - b),
        Operacion::Others(Valor { x, y }) => println!("{}", x - y),
    }
}
