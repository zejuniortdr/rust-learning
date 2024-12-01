use crate::utils::terminal::wait_for_enter;


// O código apresenta um problema porque está tentando implementar o método unwrap_or
// diretamente para o tipo genérico Option<T>. No entanto, Option já faz parte da biblioteca
// padrão do Rust, e seus métodos (como unwrap_or) já estão definidos. Sobrescrever métodos
// de tipos da biblioteca padrão não é permitido.
// Se o objetivo for criar um método personalizado ou implementar lógica adicional, você pode
// criar um trait ou uma nova estrutura.

// impl<T> Option<T> {
//     fn unwrap_or(self, other: T) -> T {
//       match self {
//         Some(t) => t,
//         None => other
//       }
//     }
// }

// pub fn run() {
//     wait_for_enter();
// }


// Definir um trait com lógica personalizada
trait OptionExtras<T> {
    fn unwrap_or_log(self, other: T) -> T;
}

// Implementar o trait para `Option<T>`
impl<T> OptionExtras<T> for Option<T> {
    fn unwrap_or_log(self, other: T) -> T {
        match self {
            Some(value) => value,
            None => {
                println!("Using default value");
                other
            }
        }
    }
}

pub fn run() {
    let maybe_value: Option<i32> = None;
    let result = maybe_value.unwrap_or_log(42); // Usando o método personalizado
    println!("Result: {}", result);

    wait_for_enter();
}
