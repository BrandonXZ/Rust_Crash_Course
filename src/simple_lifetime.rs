#![allow(unused)]
use std::{cell::{RefCell, Cell}, rc::Rc};

/**
 * This module attempts to simplify the lifetimes concept which is a fancy way of referencing variable scope within different contexts
 * 
 * 
 */


pub fn run() {
    simple_lifetime_example();
 }



 /**************************************************Function/Example Definintions**************************************************** */


 /** 
  *  This example is an intro to lifetimes in rust and starts with the simples annotated lifetime example.
  *  This main function will call any alternate functions that should be preceded by "lifetime" to illustrate it is a part 
  *  of this example set.
  */
 pub fn simple_lifetime_example() {
    //Ex.1
    let word_1 = "superfuperduperlooper".to_string();
    let word_2 = "Texltexlmexlexl".to_string();
    let ex_1 = lifetime_bigger(&word_1, &word_2);                   //note how we are passing references for this example instead of strings or converting to owned
                                                                                    //this is explicitly to demonstrate this lifetime example.
    println!("{} is the bigger word\n", ex_1);

    //Ex.2
    let str1 = "This";
    let ex_2;
    {                                                                       //brackets to indicate change of scope
        let str2 = "Those";
        ex_2 = lifetime_bigger2(str1, str2);
    }
    println!("{} is the bigger word\n", ex_2);
 }

 // Function signature below changed FROM | fn bigger(s1: &str, s2:&str) -> &str ... which generates the error below
 // "this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `s1` or `s2`"
 fn lifetime_bigger<'a>(s1: &'a str, s2:&'a str) -> &'a str {                    //note we annotate the lifetime 'before' the args<'a>, and for each relevant arg-> 'a
    if s1.len() > s2.len() { s1 } else { s2 }                            
 }                                                                      /*The return type of the function in this case must be annotated also
                                                                        This translates to binding a lifetime to the input parameters(args) 
                                                                        so rust can infer that the return value will have atleast the min lifetime 
                                                                        of the args passed in, <'a> by the fn name is just a declaration of the identifier 
                                                                        Go to Ex. 2 in fn simple_lifetime_example() */

                                                                        
//Ex. 2                                    
fn lifetime_bigger2<'b>(s1: &'b str, s2:&'b str) -> &'b str {            //note we annotate the lifetime 'before' the args<'b>, and for each relevant arg-> 'b
    if s1.len() > s2.len() { s1 } else { s2 }                            
 }                                                 