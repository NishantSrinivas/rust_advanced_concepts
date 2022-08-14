use std::ops::Add;
// fn test_func() {
//     println!("Hello from test func!");
// }

// fn multiply_int(x: u16, y: u16) {
//     println!("{} * {} = {}", x, y, x * y);
// }

// fn return_sum(x: u16, y: u16) -> u16 {
//     x + y // to return something don't put semicolon at then end.
// }

struct Point {
    x: f64,
    y: f64,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// struct Line {
//     start: Point,
//     end: Point,
// }

// impl Line {
//     fn length(&self) -> f64 {
//         let dx = self.end.x - self.start.x;
//         let dy = self.end.y - self.start.y;
//         ((dx * dx) + (dy * dy)).sqrt()
//     }
// }

trait Animal {
    fn create(_name: String) -> Self;
    fn say_name(&self);
    fn talk(&self) {
        println!("cannot talk!");
    }
}

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<u32> for Vec<u32> {
    fn sum(&self) -> u32 {
        let mut sum: u32 = 0;
        for i in self {
            sum += i;
        }
        sum
    }
}

struct Human {
    name: String,
}

struct Cat {
    name: String,
}

impl Animal for Human {
    fn create(_name: String) -> Human {
        Human { name: _name }
    }

    fn say_name(&self) {
        println!("Hello, my name is {}", self.name);
    }

    fn talk(&self) {
        println!("{}: let's talk!!", self.name);
    }
}

impl Animal for Cat {
    fn create(_name: String) -> Cat {
        Cat { name: _name }
    }

    fn say_name(&self) {
        println!("Cat's name is {}", self.name);
    }
}

// fn give_info(object: impl Animal)
// fn give_info<T: Animal>(object: T)
fn give_info<T>(object: T)
where
    T: Animal,
{
    object.say_name();
}

struct Person {
    name: String,
}

impl Person {
    // fn new<T: Into<String>> (name: T) -> Person
    fn new<T>(name: T) -> Person
    where
        T: Into<String>,
    {
        Person { name: name.into() }
    }
}

fn main() {
    // test_func();
    // multiply_int(10, 50);
    // println!("the sum of 100 and 200 is {}", return_sum(200, 100));

    let p1: Point = Point { x: 10f64, y: 10f64 };
    let p2: Point = Point { x: 20f64, y: 20f64 };
    let p3: Point = p1 + p2;
    println!("({}, {})", p3.x, p3.y);

    // let line: Line = Line { start: p1, end: p2 };

    // println!("the length of line is {}", line.length());

    // // closure
    // let add_one = |x: u16| -> u16 { x + 1 };
    // let num = 1;
    // println!("{} + 1 = {}", num, add_one(num));

    // let mut num2 = 10;
    // let mul_ten = |x: &mut u16| { *x *= 10 };
    // mul_ten(&mut num2);
    // println!("num2 = {}",num2);

    // Traits

    // let person1:Human = Human{name:"nishant".to_string()};
    let person1: Human = Human::create("nishant".to_string());
    person1.say_name();
    person1.talk();

    // let cat1:Cat = Cat{name:"kitty".to_string()};
    let cat1: Cat = Cat::create("kitty".to_string());
    cat1.say_name();
    cat1.talk();

    let some_nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("sum of some_nums is {}", some_nums.sum());

    give_info(person1);

    let p1: Person = Person::new("Tony");
    println!("p1 name: {}", p1.name);
}
