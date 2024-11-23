// struct Point {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let mut a = Point { x: 1, y: 2 };
//     a.x += 1;
//     // a {x:2, y:2}

//     // cria um ponto b com y=1 e o resto igual ao ponto a
//     // i32 implementa a trait de Copy
//     let b = Point { y: 1, ..a };
//     // b {x:2, y:1}

//     a.x += 1;
//     // a {x:3, y:2}

//     println!("{}", b.x);
//     // 2

//     println!("a.x: {}, a.y: {}", a.x, a.y);
//     println!("b.x: {}, b.y: {}", b.x, b.y);

// }


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut p = Point { x: 1, y: 2 };
    let x = &mut p.x;
    let y = &mut p.y;

    *x += 1;
    *y += 1;

    println!("{} {}", p.x, p.y);

}
