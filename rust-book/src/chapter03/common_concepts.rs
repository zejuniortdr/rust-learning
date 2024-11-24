use crate::utils::terminal::wait_for_enter;


pub fn run() {
    let x = 5;

    // Esses prints são equivalentes pois não precisam computar o valor de x
    println!("The value of x is: {}", x);
    println!("The value of x is: {x}");

    // x = 6;
    // println!("The value of x is: {}", x);
    // Por padrão as variáveis no Rust são imutáveis
    // Isso significa que não podemos alterar o valor de x após a sua declaração ou o compilador responde com o erro:
    // error[E0384]: cannot assign twice to immutable variable `x`
    // --> src/main.rs:4:5
    // |
    // 2 |     let x = 5;
    // |         - first assignment to `x`
    // 3 |     println!("The value of x is: {}", x);
    // 4 |     x = 6;
    // |     ^^^^^ cannot assign twice to immutable variable
    // |
    // help: consider making this binding mutable
    // |
    // 2 |     let mut x = 5;
    // |         +++

    // For more information about this error, try `rustc --explain E0384`.
    // error: could not compile `common_concepts` (bin "common_concepts") due to 1 previous error

    // Para cenários onde é necessário computar o valor, somente o segundo funciona
    println!("The value of 2*x is: {}", 2 * x);

    // E o primeiro gera o erro:
    //     println!("The value of 2*x is: {2*x}");
    //     error: invalid format string: expected `'}'`, found `'*'`
    //   --> src/main.rs:33:38
    //    |
    // 33 |     println!("The value of 2*x is: {2*x}");
    //    |                                    - ^ expected `'}'` in format string
    //    |                                    |
    //    |                                    because of this opening brace
    //    |
    //    = note: if you intended to print `{`, you can escape it using `{{`

    // error: could not compile `common_concepts` (bin "common_concepts") due to 1 previous error

    let mut y = 10;
    println!("The value of y is: {y}");
    y = 20;
    println!("The value of y is: {y}");


    // Constantes sempre em upper case
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("There are {THREE_HOURS_IN_SECONDS} seconds in 3 hours.");

    // const four_HOURS_IN_SECONDS: u32 = 60 * 60 * 4;
    //  Compiling common_concepts v0.1.0 (/home/sronly/Devel/pessoal/rust-learning/projects/capitulo-03/common_concepts)
    // warning: constant `four_HOURS_IN_SECONDS` is never used
    // --> src/main.rs:54:11
    // |
    // 54 |     const four_HOURS_IN_SECONDS: u32 = 60 * 60 * 4;
    // |           ^^^^^^^^^^^^^^^^^^^^^
    // |
    // = note: `#[warn(dead_code)]` on by default

    // warning: constant `four_HOURS_IN_SECONDS` should have an upper case name
    // --> src/main.rs:54:11
    // |
    // 54 |     const four_HOURS_IN_SECONDS: u32 = 60 * 60 * 4;
    // |           ^^^^^^^^^^^^^^^^^^^^^ help: convert the identifier to upper case: `FOUR_HOURS_IN_SECONDS`
    // |
    // = note: `#[warn(non_upper_case_globals)]` on by default

    // warning: `common_concepts` (bin "common_concepts") generated 2 warnings
    // Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
    // Running `target/debug/common_concepts`


    // Shadowing
    // Shadowing é diferente de marcar uma variável como mut porque teremos um erro de tempo de compilação
    // se acidentalmente tentarmos reatribuir a essa variável sem usar a letpalavra-chave. Usando let, podemos
    // executar algumas transformações em um valor, mas fazer com que a variável seja imutável após essas transformações
    // terem sido concluídas.

    let z = 5;
    let z = z + 1;

    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}");
    }

    println!("The value of z is: {z}");


    let spaces = "     ";
    let spaces = spaces.len();
    println!("The length of '{}' is {}.", spaces, spaces);

    let spaces2 = "       ".len();
    println!("The length of '{}' is {}.", spaces2, spaces2);


    // FLOATS
    let a = 2.6; // f64
    let b: f64 = 3.1; // f32

    println!("{a} {b}");


    // OPERATIONS
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncate = -5 / 3;
    let remainder = 43 % 5;

    println!("Sum: {sum}");
    println!("Difference: {difference}");
    println!("Product: {product}");
    println!("Quotient: {quotient}");
    println!("Truncate: {truncate}");
    println!("Remainder: {remainder}");


    // BOOLEANS
    let t = true;
    let f: bool = false;
    println!("{t} {f}");

    // CHARS
    // Aspas simples para chars, duplas para strings
    let c = 'z';
    let d: char = 'Z';
    let heart_eyed_cat = '😻';
    println!("{c} {d} {heart_eyed_cat}");


    // TUPLES
    // Podem ter tipos diferentes
    let tup: (i32, f64, u8, char, bool) = (500, 6.4, 1, 'a', true);
    println!("The tuple is: {:?}", tup);

    let (el1, el2, el3, el4, _) = tup;
    println!("{el1}, {el2}, {el3}, {el4}");
    println!("{}", tup.4);

    let mut tup2: (i32, i32) = (1, 2);
    tup2.0 = 10;
    tup2.1 = 11;
    println!("{}, {}", tup2.0, tup2.1);

    // ARRAYS
    // Precisam ser do mesmo tipo
    let arr = [1,2,3,4,5];
    let arr2: [i32;5] = [10,20,30,40,50];
    println!("The second element of the array is: {}", arr[1]);
    println!("The second element of the array2 is: {}", arr2[1]);

    let arr_same_val = [3; 5];
    println!("The array with all elements being 3 is: {:?}", arr_same_val);


    wait_for_enter();
}
