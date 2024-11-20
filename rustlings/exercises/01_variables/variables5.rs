fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {}", number);

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    // number = 3;
    //       ^ expected `&str`, found integer

    // Shadowing da variável `number`
    let number:i32 = 3;

    /*
    O shadowing em Rust é uma funcionalidade que permite declarar uma nova variável com o mesmo nome
    de uma já existente no mesmo escopo. A nova variável "sombreia" (ou substitui temporariamente) a antiga,
    permitindo reutilizar o nome sem alterar a variável original.

    Como funciona o shadowing?
    Quando uma nova variável é declarada com o mesmo nome, a anterior é "escondida" (somente naquele escopo).
    A nova variável pode ter um tipo diferente ou ser manipulada de outra forma.
    A variável anterior não é mod
    */

    println!("Number plus two is: {}", number + 2);
}
