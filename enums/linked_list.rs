/*
 * To reduce the verbosity of the enum variants, we can bring List into the scope of this crate*/
use crate::List::*;

enum List {
    /*
     * Notice we use Box<Type>
     * Box is a smart pointer, which allocates memory on heap.
     * Warning:: Do not use normal pointers. Rust does not allow it since it is not memory safe!
     * */
    Link(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    /*
     * Notice that prepend() requires transfer of ownership. Makes sense, since after prepend the
     * calling function has no requirement of the former "head" of the linkedlist*/
    fn prepend(self, elem: u32) -> List{
        /*
         * Note that the head of the list is always on the stack!
         * To prepend the head of the list is being transferred to the heap using Box::new()
         * After which the pointer is used to create and return a Link value
         * */
        Link(elem, Box::new(self))
    }

    /*
     *Notice that in contrast to prepend(), the calling function requires access to the "head", so we pass borrowed values. 
     * */
    fn len(&self)-> u32 {
        match self {
            /*
             * We use "ref" here because destructuring causes transfer of ownership.
             * However, if the detructured type implements the "Copy" trait, then we do not require "ref"*/
            Link(_, ref next) => 1+next.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        /*
         * Here, we do not require "ref" for the first associated value of the variant since it is
         * u32, which is a primitive type, thus implementing the Copy trait*/
        match self {
            /*
             * Note that format! is a macro which is exactly like println!, but it returns a String instead of printing to stdio*/
            Link(value, ref next) => format!("{} {}",next.stringify(),value),
            Nil => String::from("Nil"),
        }
    }
}

fn main() {
    /*
     * Behold! it works <3*/
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
