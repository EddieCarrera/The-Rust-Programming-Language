/***************************** Important Notes *********************************
 * 1) Like immutable variables, constants are values that are bound to a name and 
 * are not allowed to change. Constants can be declared in any scope, including the 
 * global scope, which makes them useful for values that many parts of code need 
 * to know about. Variables declared with 'let' cannot be global.
 * 
 * 2) The other difference between mut and shadowing is that because we’re effectively 
 * creating a new variable when we use the let keyword again, we can change the type 
 * of the value but reuse the same name.
 * 
 * 3) char type is 4 bytes in rust
 * 
 * 4) When you attempt to access an element using indexing, Rust will check that the 
 * index you’ve specified is less than the array length. If the index is greater than 
 * or equal to the length, Rust will panic. This check has to happen at runtime, especially 
 * in this case, because the compiler can’t possibly know what value a user will enter 
 * when they run the code later.
 *******************************************************************************/
 fn main() {

    /* Example of mutability */
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    /* Example of shadowing  

    within an inner scope created with the curly brackets, the 
    third let statement also shadows x and creates a new variable, 
    multiplying the previous value by 2 to give x a value of 12. 
    When that scope is over, the inner shadowing ends and x returns 
    to being 6*/
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    /* Tuple access example */
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;    
}