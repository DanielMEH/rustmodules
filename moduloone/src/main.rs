#[derive(Debug)]
enum AlmacenError {
    CapacidadExcedida,
    CantidadInvalida,
}

pub(crate) trait Almacenable {
    fn add_product(&mut self, cantidad: i32) -> Result<String, AlmacenError>;
}

#[derive(Debug)]
struct Bodega {
    conteo_cambios: i32,
}
#[derive(Debug)]
struct Tienda {
    total_item: i32,
    name: String,
}

impl Almacenable for Bodega {
    fn add_product(&mut self, cantidad: i32) -> Result<String, AlmacenError> {
        if cantidad <= 0 {
            return Err(AlmacenError::CantidadInvalida);
        }
        self.conteo_cambios += cantidad;
        Ok(String::from("Success for Bodega"))
    }
}

impl Almacenable for Tienda {
    fn add_product(&mut self, cantidad: i32) -> Result<String, AlmacenError> {
        println!("new product: {}", self.name);
        if cantidad <= 0 {
            return Err(AlmacenError::CantidadInvalida);
        }
        self.total_item += cantidad;
        Ok(String::from("Success for Tienda"))
    }
}

pub(crate) trait Auditable: Almacenable {
    fn registrar_operacion(&mut self, cant: i32) -> Result<String, AlmacenError>;
}

impl<T> Auditable for T
where
    T: Almacenable + std::fmt::Debug,
{
    fn registrar_operacion(&mut self, cantidad: i32) -> Result<String, AlmacenError> {
        println!(
            "Cantidad de productos registrados: {cantidad} audits:{:#?}",
            self
        );
        self.add_product(cantidad)?;
        Ok(String::from("sucess"))
    }
}
fn main() {
    let cant_productos: i32 = 0;
    let mut tienda = Tienda {
        name: "Jabon".to_string(),
        total_item: cant_productos,
    };
    match tienda.registrar_operacion(cant_productos) {
        Ok(v) => println!("value: {}", v),
        Err(err) => println!("error: {:?}", err),
    }

    let count = 0;
    let mut bodega = Bodega {
        conteo_cambios: count,
    };

    match bodega.registrar_operacion(count) {
        Ok(v) => println!("value: {}", v),
        Err(err) => println!("error: {:?}", err),
    }

    println!("handle")
}
