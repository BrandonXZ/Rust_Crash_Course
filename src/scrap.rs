#![allow(unused)]
#![allow(non_snake_case)]
use std::{cell::{RefCell, Cell}, rc::Rc, ops::Deref};

/**
 * A scrap workarea for perfecting a concept example before its converted to it's namesake module and included in main...
 * Todays work should be centered on pointers, smart pointers, 
 * 
 * Modules are still needed for
 * references, deref operators
 * closures
 * advanced traits
 * generics
 * advanced coersion
 * elision
 */

 /****************************************************Function Definitions********************************************* */

 //The Deref Trait
struct MyBox<T>(T);

impl<T> MyBox<T> {              //here we recreate the box pointer but theres a big diff since x is not stored on the heap but for now its ok because we're focusing on deref
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
} 

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {      //we can also simply write -> &T
        &self.0                             //return reference to first item in our tuple struct which
    }
}

 pub fn run() {
    simple_deref_example();
 }


 pub fn simple_deref_example() {

    let x = 5;  
    assert_eq!(5, x); 
    
    let y = &x;                 //ref to x, y is a basic pointer
    assert_eq!(5, *y);                //pointer here//dereferencing y 
    // assert_eq!(5, y);              //throws error: error[E0277]: can't compare `{integer}` with `&{integer}` which is why we use the dereference 

    let z = Box::new(x);    //in this case Z is pointing to a "copy" of 
    assert_eq!(5, *z);                //and so the box allows us to deref the same way but is almost a cheat since ints are copied when passed to a function anyway

    let z1 = MyBox::new(x);
    assert_eq!(5, *z1);               //Error until deref is impl for MyBox
                                      //without the deref trait being impl the compiler only knows how to deref references.
    
                                    
 }
