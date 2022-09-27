#![allow(unused)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
use std::{cell::{RefCell, Cell}, rc::Rc, ops::Deref};

/**
 * This module elaborates on how to use different types of pointers in rust 
 * which seem like an instrumental tool when working with gui's
 * when used with lifetimes/generics
 * Smart pointers are structs that implement the deref trait and drop trait
 * 
 */

pub fn run() {
    references();
    basic_pointers();
    simple_pointer_example();
    simple_box_example();
    another_pointer_example();
 }

 /**************************************************Function/Example Definintions**************************************************** */

 /**
  * Basically serving as a syntax reference guide 
  */
 pub fn references() {
    let mut num = 5; 
    let mut num1 = 7; 

    let r1 = &num;                   //immutable reference
    let r2 = &mut num1;          //mutable reference

    println!("r1 is: {}, r2 i: {}", r1, r2);
 }

 /**
  * Basically serving as a syntax reference guide
  */
 pub fn basic_pointers() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32; 
    println!("num is: {}, r1 is: {:?}, r2 i: {:?}", num, r1, r2);
 }


 /**********************************************Advanced pointers******************************************** */
 pub fn simple_pointer_example() {

    let mut owned = "counter-->";
    let mut some_str = "this is a str";

    let ref stolen = owned;
    let x = Rc::new(RefCell::new(owned));
    println!("Stolen is: {}", stolen);
    println!("original owned is {:?}", x);
    println!("referenced owned is {:?}", &x);
    println!("pointer to owned is {:?}", *x);

    let old_val = x.replace(some_str);
    println!("old val from pointer: {}", old_val);
    println!("Val was changed to: {:?}", *x);

    looper(x.clone());
 }

 /** 
  * This example uses pointers to change a stored value with multiple owners
  * and reference the new value. This requires lifetimes due to the nature of calling borrowed or referenced values 
  * which is the essence of pointers.
  */
  pub fn looper(Cell_val: Rc<RefCell<&str>>) {

    println!("RefCell a passed to looper is: {:?}\n", *Cell_val);                          //confirming value passed to function is as expected
    // let mut old_val;
    let mut item1: &str;       

    for i in 0..10 {
        let mut item2 = format!("{:?} --># {}", *Cell_val, i.clone());              //using referenced value to create a new value
        // old_val = Cell_val.replace(item2);                                              //replaceing the old value with the new value 
        println!("tester is: {:?}", *Cell_val);  
        println!("item2 is: {}", item2);        
    }
 }

  /**
  * This example goes over the box smart pointer which is a reference to a heap allocation holding another value
  * There are 2 types of boxes, manages and owned owned boxes are simply boxes instantiated at creation like variable "owned" below
  * Boxes don't have any overhead other than storing on the heap but don't have many other capabilities either
  * Typical Use Case: (1) When you have a type whose exact size can't be known at compile time and you want to use a value of that type in a context which requires
  * knowing the exact size. (2) When you have a large amount of data and  you want to transfer ownership of that data but want to ensure this is done without copying
  * (3) When you own a value and only care that the value implements a specific trait rather than it being of a specific type - also known as a **trait object**
  * boxes are deallocated when they go out of scope.
  */
  pub fn simple_box_example() {

//Example 1
    let owned = Box::new("All mine");                       //we want to store "All mine" on the heap using var "owned" stored on the stack
    println!("(Ex. 1-a) owned is: {}", owned);                           //works
    let stolen = owned;
    println!("(Ex. 1-a) stolen is: {}", stolen);                         //works

//Example 1-b
    let owned1 = Box::new("All mine-1");  
    println!("(Ex. 1-b) owned1 is: {}", owned1);
    // let stolen = owned1;                                    //allows assignment but commented out for example 1-c due to line below referencing moved value
    // println!("owned1 is now: {}", owned1);                  // doesn't work -fails due to use of moved value ~simple borrow checker example

//example 1-c
    let ref stolen1 = owned1;
    println!("(Ex. 1-c) owned1 is: {}", owned1);                         //We can now print both because we declare stolen as a "ref" (same as borrow)
    println!("(Ex. 1-c) stolen1 is: {}", *stolen1);                      //and use a pointer to access the original value 
    let stolen2 = &owned1;
    println!("(Ex. 1-c) stolen2 is: {}", stolen2);                       //example using borrow instead of pointer
 }


/**********************************************************Another Example *************************************************************/

 use List::{Cons, Nil}; 
enum List {
    Cons(i32, Box<List>),        //tuple that holds integer and a list(this enum), so its recursive
    Nil,                         //with recursive enums, we don't know how much space is taken up...
                            //**Cons list is a data structure that comes from list and basically stores a value and a pointer to the next box until it gets to nil(no value) */
 }

/** The rust compiler states :error[E0072]: recursive type `List` has infinite size and suggests: 
 * Cons(i32, Box<List>),        //tuple that holds integer and a list(this enum), so its recursive
   |         ++++    +
*/




 /**
  * The basic idea behind this example is that we can compile a type of unknown type(the Cons list because its recursive)   
  * by wrapping it in the Box pointer which allows us to point to its value on the heap
  * we then use the pointer as our fixed size essentially to get it to compile...
  * We do this by wrapping the list items in box's and the enum variant in the box  
  */
pub fn another_pointer_example() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)
                         )           )    )           )     ); //ignore the weirdness

}

/********************End Another Pointer Example ******************************/


/*************************************************************Smart Pointer Deref Trait************************************************************/

