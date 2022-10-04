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
// Reference Counting Smart pointer
/**
 * This covers the reference counter smart pointer, which allows you to share ownership of some data.
 * To enable multiple ownership for a value, the RC counter keeps track of the number of references 
 * to the value and cleans up when there are no more references. The RC smartpointer is used when we want to allocate a value on the heap
 * and have multiple parts read that value and don't know which part is going to finish using the data last. Thee examples only relate to 
 * single threaded programs **see threads** 
 */

 enum List {
    Cons(i32, Box<List>),                                       //we could change definition of cons variant to hold references instead of owned values
    Nil,                                                        //but that would require the use of lifetimes, and is essentially us specifying
}                                                               //that every element in the list has to live atleast as long as the list itself 
                                                                //in this case we replace Cons(i32, Box<List>) < with 

 use List::{Cons, Nil}; 

 pub fn run() {
    ref_count_example();
 }

pub fn ref_count_example(){
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));                   // "b" now owns "a" which means we can't use "a" in "c" 
    // let c = Cons(4, Box::new(a));
}



