use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

use crate::List::{Nil, Cons};

enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

#[derive(Debug)]
enum List2 {
    Cons(i32, RefCell<Rc<List2>>),
    Nil,
}

impl List2 {
    fn tail(&self) -> Option<&RefCell<Rc<List2>>> {
        match self {
            List2::Cons(_, item) => Some(item),
            List2::Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let l = Cons(Rc::new(RefCell::new(1)), Rc::new(Cons(Rc::new(RefCell::new(2)), Rc::new(Cons(Rc::new(RefCell::new(3)), Rc::new(Nil))))));
    let m = MyBox::new(String::from("Rust"));

    print_list(&l);
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    drop(c);
    println!("end of main");

    let a = Rc::new(Cons(Rc::new(RefCell::new(5)), Rc::new(Cons(Rc::new(RefCell::new(10)), Rc::new(Nil)))));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    print_list(&a);
    println!("a done");
    print_list(&b);
    println!("b done");
    print_list(&c);
    println!("c done");
    println!("count: {}", Rc::strong_count(&a));
    drop(b);
    println!("count: {}", Rc::strong_count(&a));
    if let Cons(val, l) = &*a {
        *val.borrow_mut() += 5;
    }
        print_list(&c);

    let a = Rc::new(List2::Cons(5, RefCell::new(Rc::new(List2::Nil))));
    let b = Rc::new(List2::Cons(10, RefCell::new(Rc::clone(&a))));

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    //println!("a next item = {:?}", a.tail());

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade());
}

fn hello(name: &str) {
    println!("hello, {name}!");
}

fn print_list(l: &List) {
    let mut cur_node = l;
    loop {
        match cur_node {
            Cons(val, next) => {
                println!("{}", val.borrow());
                cur_node = &next
            }
            Nil => break,
        };
    }
}