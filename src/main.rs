// Entry point
fn main() {
    println!("Hello, world!");

    // Calling the "snake_case" function
    snake_case();

    // Calling the "fun_parameters" function and specifying x to the value 5
    fun_parameters(5);
}

/*
*   Rust code uses snake case as the conventional style for function and variable
*   names, in which all letters are lowercase and underscores separate words.
*/

fn snake_case() {
    println!("This is a function using snake case styling for the name!");
}

/*
*   We can define functions to have parameters, which are special variables that
*   are part of a function’s signature. When a function has parameters, you can
*   provide it with concrete values for those parameters. Or you can leave them
*   as a default value such as "x" and specify the value later in the main function.
*/

fn fun_parameters(x: i32) {
    println!("The value of x is: {x}");
}
