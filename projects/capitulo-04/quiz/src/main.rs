// fn main() {
//     let s = String::from("hello");
//     let s2;
//     let b = false;
//     if b {
//         s2 = s;
//     }
//     println!("{}", s);
// }


// fn move_a_box(b: Box<i32>) {
//     // This space intentionally left blank
// }

// fn main() {
//     let b = Box::new(0);
//     move_a_box(b);
//     println!("{}", b);
// }


// fn incr(n: &mut i32) {
//     *n += 1;
//   }
//   fn main() {
//     let mut n = 1;
//     incr(&n);
//     println!("{n}");
//   }



// fn give_and_take(v: &Vec<i32>, n: i32) -> i32 {
//     v.push(n);
//     v.remove(0)
// }
// fn main() {

//     let v = vec![1, 2, 3];
//     let n = &v[0];
//     give_and_take(&v, 4);
//     println!("{}", n);
// }


// fn main() {
//     let s = String::from("Hello world");
//     let s_ref = &s;
//     let s2 = *s_ref;
//     println!("{s2}");

// }


// fn copy_to_prev(v: &mut Vec<i32>, i: usize) {
//     let n = &mut v[i];
//     *n = v[i - 1];
//   }
// fn main() {
//     let mut v = vec![1, 2, 3];
//     copy_to_prev(&mut v, 1);
// }


fn main() {
    let mut point = [0, 1];
    let mut x = point[0];
    let y = &mut point[1];
    x += 1;
    *y += 1;
    println!("{} {}", point[0], point[1]);
}
