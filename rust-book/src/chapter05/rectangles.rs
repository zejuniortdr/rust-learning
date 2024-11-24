use crate::utils::terminal::wait_for_enter;


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn set_width(&mut self, width:u32) {
        self.width = width;
    }

    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

pub fn run() {
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    println!(
        "(fn) Area of rectangle 1: {} square units\n",
        calculate_area(&rect1)
    );
    println!(
        "(method) Area of rectangle 1: {} square units\n",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}\n", rect1.width);
    }

    println!("rect1 is {rect1:?}\n"); // single line
    println!("rect1 is {rect1:#?}\n"); // formatted


    let scale = 2;
    println!("Debug 30 * scale:");
    let rect2 = Rectangle {
        width: dbg!(30 * scale), // debug calculus [src/main.rs:26:16] 30 * scale = 60
        height: 50,
    };

    println!("Debug rect2:");
    dbg!(&rect2); // debug struct
    // [src/main.rs:30:5] &rect2 = Rectangle {
    //     width: 60,
    //     height: 50,
    // }


    let rect3 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect4 = Rectangle {
        width: 10,
        height: 60,
    };

    println!("Can rect1 hold rect3? {}\n", rect1.can_hold(&rect3));
    println!("Can rect1 hold rect4? {}\n", rect1.can_hold(&rect4));


    let square = Rectangle::square(10);
    dbg!(&square);

    let mut r = Rectangle{
        width: 1,
        height: 2,
    };

    r.set_width(2);
    Rectangle::set_width(&mut r, 2);

    println!("r: {r:?}\n"); // single line


    let rect5 = rect1.max(rect3);
    println!("max: {rect5:?}\n"); // single line

    wait_for_enter();
}


fn calculate_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
