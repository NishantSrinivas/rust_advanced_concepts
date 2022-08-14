fn test_func() {
    println!("Hello from test func!");
}

fn multiply_int(x: u16, y: u16) {
    println!("{} * {} = {}", x, y, x * y);
}

fn return_sum(x: u16, y: u16) -> u16 {
    x + y // to return something don't put semicolon at then end.
}

struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn length(&self) -> f64 {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        ((dx * dx) + (dy * dy)).sqrt()
    }
}

fn main() {
    test_func();
    multiply_int(10, 50);
    println!("the sum of 100 and 200 is {}", return_sum(200, 100));

    let p1: Point = Point { x: 0f64, y: 0f64 };
    let p2: Point = Point { x: 20f64, y: 20f64 };

    let line: Line = Line { start: p1, end: p2 };

    println!("the length of line is {}", line.length());

    // closure
    let add_one = |x: u16| -> u16 { x + 1 };
    let num = 1;
    println!("{} + 1 = {}", num, add_one(num));

    let mut num2 = 10;
    let mul_ten = |x: &mut u16| { *x *= 10 };
    mul_ten(&mut num2);
    println!("num2 = {}",num2);
}
