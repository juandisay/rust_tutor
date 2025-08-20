// Smart pointers are data structures that act like a pointer but also have additional metadata and capabilities.

use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

// SECTION: `Box<T>` for allocating values on the heap
// Boxes allow you to store data on the heap rather than the stack.

// A cons list is a data structure that comes from the Lisp programming language.
// Using a `Box<T>` is necessary here to have a recursive type with a known size.
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// SECTION: `Deref` trait for customizing the dereference operator `*`
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// SECTION: `Rc<T>`, the Reference Counted Smart Pointer
// `Rc<T>` enables multiple ownership. It keeps track of the number of references to a value
// to determine whether or not the value is still in use. If there are zero references to a value,
// it can be cleaned up without breaking any references.
#[derive(Debug)]
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

// SECTION: `RefCell<T>` and the Interior Mutability Pattern
// Interior mutability is a design pattern in Rust that allows you to mutate data even when
// there are immutable references to that data.
// `RefCell<T>` represents single ownership over the data it holds, but it enforces the borrowing
// rules at runtime instead of at compile time.

// A use case for `RefCell<T>` is when you have a type that has methods that modify itself,
// but you need to make it available to other parts of your code that can only take immutable references.

// SECTION: `Weak<T>` to prevent reference cycles
// `Weak<T>` are weak references. They don't increase the reference count.
// This can be used to prevent memory leaks caused by circular references between `Rc<T>` instances.
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}


fn main() {
    // Using a Box<T>
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("Box list: {:?}", list);

    // Using our custom MyBox<T>
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); // `*y` is translated to `*(y.deref())`
    println!("MyBox dereferences successfully!");

    // Using Rc<T> to share data
    let a = Rc::new(RcList::Cons(5, Rc::new(RcList::Cons(10, Rc::new(RcList::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = RcList::Cons(3, Rc::clone(&a)); // Cloning increases the reference count
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = RcList::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        println!("List b: {:?}", b);
        println!("List c: {:?}", c);
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // Example of a reference cycle and how Weak<T> fixes it
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
