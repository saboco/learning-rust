extern crate smart_pointers;
use smart_pointers::MyBox;

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