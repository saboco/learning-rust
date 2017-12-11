pub mod mybox;
pub use mybox::*;

pub enum MyList {
    Cons(i32, Box<MyList>),
    Nil,
}

pub fn cons(a: i32, b:i32) -> MyList {
    MyList::Cons(a, Box::new(MyList::Cons(b, Box::new(MyList::Nil))))
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let list = Cons(1, 
            Box::new(Cons(3,
                Box::new(Cons(4, Box::New(Nil))))));
    }
}
