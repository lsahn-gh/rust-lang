#[derive(Debug)]
struct Rect {
    length: u32,
    width: u32
}

impl Rect {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, another: &Rect) -> bool {
        self.length >= another.length && self.width >= another.width
    }

    fn square(size: u32) -> Rect {
        Rect { length: size, width: size }
    }
}

fn main() {
    let rect1 = Rect::square(50);
    let rect2 = Rect { length: 40, width: 10 };
    let rect3 = Rect { length: 45, width: 60 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

