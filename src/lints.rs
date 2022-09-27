/**
 * This gives an example of all available lints and defines their behavior 
 * lints are a static code analysis tool used to flag programming errors, bugs, stylistic errors and suspicious constructs.
 * lint flags in rust allow us to allow, warn, or deny certain lint checks during coding, compile-time, or runtime.
 * Techically the flag-types are: allow, warn, force-warn, deny, forbid.
 */
/**
* The most easily understood example is someone who wants to code a program that takes a random number from a user and multiplies it by 10.
* The coder doesn't know how they will go about coding a solution, whether or not it is modular, or what features or functions they will define just yet
* all they know is that they will need "random" numbers. If you don't know this already, there is no such thing as a truly random number generated
* by a computer without some type of input entropy. That is: something truly random to generate these numbers from such as a lava lamp, or waves in an ocean.
* Rather than code this, there are external crates that simulate this behavior up to a certain (usually large) depth. So we need a crate
* The first thing the person does is find and add that crate before writing any other code.  
*/
#![allow(unused_imports)]
use random::*;
/** 
 * There are red squigs under the crate import because the rust compiler(lint check) is checking for any unused imports to support cleaner more organized code.
 * and one can bypass this by adding #![()] to the top of the module (before any other code) 
 * or #[()] directly above the item they want to exclude from this specific lint check
 * the actual synax is : #![allow(unused_imports)]. The lint command goes inside the square bracket, and the lint type related to your 
 * command goes inside the parenthesis.
 * No More red squigs!!
 */
#![allow(unused_mut)]
#![allow(unused_must_use)]
#![allow(unused_assignments)]
pub fn blank() {

}


