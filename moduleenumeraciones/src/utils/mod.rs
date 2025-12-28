pub mod utils {

    // pub(crate) trait Alimentos {
    //     fn obtener_nombre(&self) -> String;
    // }

    // pub enum TipoDieta {
    //     Carnivoro,
    //     Herviboro,
    // }

    // pub struct Animal<T> {
    //     pub name: String,
    //     pub dieta: T,
    // }

    // impl<T> Animal<T> {
    //     pub(crate) fn new_animal(animal: impl Into<String>, dieta: T) -> Self {
    //         Self {
    //             name: animal.into(),
    //             dieta,
    //         }
    //     }
    // }

    // impl<T: std::fmt::Display> Alimentos for (Animal<T>, TipoDieta) {
    //     fn obtener_nombre(&self) -> String {
    //         let prefix = match self.1 {
    //             TipoDieta::Carnivoro => println!("Carnivoro"),
    //             TipoDieta::Herviboro => println!("Herbiboro"),
    //         };
    //         format!(
    //             "{:?} es: name: {} dieta: {}",
    //             prefix, self.0.name, self.0.dieta
    //         )
    //     }
    // }

    pub(crate) trait GetUsers {
        fn getByIdUser(&self);
        fn get_details(&self);
        fn getProfile(&self);
    }

    pub(crate) trait PostUser {
        fn inset_new_user(&self);
        fn inset_new_profile(&self);
        fn inset_new_account(&self);
    }

    pub(crate) trait RepositoryUser: GetUsers + PostUser {}

    impl<T> RepositoryUser for T where T: GetUsers + PostUser {}
}
