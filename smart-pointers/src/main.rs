extern crate smart_pointers;
use smart_pointers::MyBox;
use smart_pointers::MyListRc::{Cons, Nil};
use std::rc::Rc;

fn main(){
    let x = 5;
    let y = &x;
    let z = &y;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, **z);

    println!("x {}", x);
    println!("y {}, *y {}", y, *y);
    println!("z {}, *z {}, **z {}", z, *z, **z);

    let x = 3;
    let y = MyBox::new(x);

    assert_eq!(3, x);
    assert_eq!(3, *y);

    println!("x {}", x); 
    println!("y {:?}, *y {:?}", y, *y);

    // deref coercions
    let m = MyBox::new(String::from("Rust"));
    println!("m is {:?}", m);
    hello(&m);

    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");

    let e = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    drop(e);
    println!("CustomSmartPointer dropped before the end of main.");

    // shared list with Rc
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    // visualizing the counting
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}