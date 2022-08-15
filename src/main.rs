use std::thread;
// use std::sync::{Mutex, Arc};
// use std::ops::Add;
// fn test_func() {
//     println!("Hello from test func!");
// }

// fn multiply_int(x: u16, y: u16) {
//     println!("{} * {} = {}", x, y, x * y);
// }

// fn return_sum(x: u16, y: u16) -> u16 {
//     x + y // to return something don't put semicolon at then end.
// }

// struct Point {
//     x: f64,
//     y: f64,
// }

// impl Add for Point {
//     type Output = Point;

//     fn add(self, rhs: Self) -> Self::Output {
//         Point {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//         }
//     }
// }

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

// trait Animal {
//     fn create(_name: String) -> Self
//     where
//         Self: Sized;
//     fn say_name(&self);
//     fn talk(&self) {
//         println!("cannot talk!");
//     }
// }

// trait Summable<T> {
//     fn sum(&self) -> T;
// }

// impl Summable<u32> for Vec<u32> {
//     fn sum(&self) -> u32 {
//         let mut sum: u32 = 0;
//         for i in self {
//             sum += i;
//         }
//         sum
//     }
// }

// struct Human {
//     name: String,
// }

// struct Cat {
//     name: String,
// }

// impl Animal for Human {
//     fn create(_name: String) -> Human {
//         Human { name: _name }
//     }

//     fn say_name(&self) {
//         println!("Hello, my name is {}", self.name);
//     }

//     fn talk(&self) {
//         println!("{}: let's talk!!", self.name);
//     }
// }

// impl Animal for Cat {
//     fn create(_name: String) -> Cat {
//         Cat { name: _name }
//     }

//     fn say_name(&self) {
//         println!("Cat's name is {}", self.name);
//     }
// }

// fn give_info(object: impl Animal)
// fn give_info<T: Animal>(object: T)
// fn give_info<T>(object: T)
// where
//     T: Animal,
// {
//     object.say_name();
// }

// struct Person {
//     name: String,
// }

// impl Person {
//     // fn new<T: Into<String>> (name: T) -> Person
//     fn new<T>(name: T) -> Person
//     where
//         T: Into<String>,
//     {
//         Person { name: name.into() }
//     }
// }

// fn take_a_string(ss: String) {
//     println!("{}", ss);
// }

// fn borrow_a_string(ss: &String) {
//     println!("{}", ss);
// }

// trait Printable {
//     fn format(&self) -> String;
// }

// impl Printable for u32 {
//     fn format(&self) -> String {
//         format!("u32 : {}", *self)
//     }
// }

// impl Printable for String {
//     fn format(&self) -> String {
//         format!("String : {}", *self)
//     }
// }

// fn print_it<T: Printable>(x: T) {
//     println!("{}", x.format());
// }

// fn print_it_dynamic(x: &dyn Printable) {
//     println!("{}", x.format());
// }

// struct Car<'a> // specifying lifetime <'a>
// {
//     name: &'a str // 'a
// }

// impl<'a> Car<'a>
// {
//     fn name(&self)
//     {
//         println!("This is a {} car.",self.name);
//     }
// }

struct Account {
    balance: u64,
}

impl Account {
    fn new() -> Account {
        Account { balance: 0 }
    }

    fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: u64) {
        if self.balance >= amount {
            self.balance -= amount;
        } else {
            println!("Sorry op can't be performed, not enough balance");
        }
    }

    fn enquire_balance(&self) {
        println!("available balance: {}", self.balance);
    }
}

fn main() {
    // test_func();
    // multiply_int(10, 50);
    // println!("the sum of 100 and 200 is {}", return_sum(200, 100));

    // let p1: Point = Point { x: 10f64, y: 10f64 };
    // let p2: Point = Point { x: 20f64, y: 20f64 };
    // let p3: Point = p1 + p2;
    // println!("({}, {})", p3.x, p3.y);

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
    // let person1: Human = Human::create("nishant".to_string());
    // person1.say_name();
    // person1.talk();

    // let cat1:Cat = Cat{name:"kitty".to_string()};
    // let cat1: Cat = Cat::create("kitty".to_string());
    // cat1.say_name();
    // cat1.talk();

    // let some_nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // println!("sum of some_nums is {}", some_nums.sum());

    // give_info(person1);

    // let p1: Person = Person::new("Tony");
    // println!("p1 name: {}", p1.name);

    // let my_string: String = "test string".to_string();
    // let my_new_string: String = String::from("new test string");
    // my_string = "test string".to_string();

    // println!("{}", my_string);
    // take_a_string(my_string);
    // println!("{}",my_string); // problem as take a string takes away my string!

    // println!("{}", my_new_string);
    // borrow_a_string(&my_new_string);
    // println!("{}",my_new_string); // no problem as borrow a string doesn't take ownership

    // let x: u32 = 100;
    // let xstr: String = String::from("test string");
    // println!("{}",x.format());
    // print_it(x);
    // print_it(xstr);

    // print_it_dynamic(&x);
    // print_it_dynamic(&xstr);

    // vector of Animals
    // let mut animals: Vec<Box<dyn Animal>> = Vec::new();
    // let person1: Human = Human::create("nishant".to_string());
    // let cat1: Cat = Cat::create("kitty".to_string());
    // animals.push(Box::new(Human::create("nishant".to_string())));
    // animals.push(Box::new(Cat::create("kitty".to_string())));

    // for animal in animals.iter() {
    //     animal.say_name();
    // }

    // Lifetime in Structs impl
    // let new_car: Car = Car{name:"BMW"};
    // new_car.name();

    let mut acc: Account = Account::new();
    acc.deposit(500);

    let thr = thread::spawn(move || {
        let mut x: u8 = 0;
        while x < 10 {
            acc.withdraw(50);
            acc.enquire_balance();
            x += 1;
        }
    });

    let thr2 = thread::spawn(move || {
        let mut x: u8 = 0;
        while x < 10 {
            println!("hello");
            x += 1;
        }
    });

    thr.join().unwrap();
    thr2.join().unwrap();
    // acc.enquire_balance();
}

// Reference count variables
// use std::rc::Rc
// name: string -> name: Rc<String>
// "name".to_string(); -> Rc::new("name".to_string());
// to get referenc count Rc::strong_refernce(&name)
