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

/*
*   In function signatures, you must declare the type of each parameter. This is a
*   deliberate decision in Rust’s design: requiring type annotations in functions
*   definitions means the compiler almost never needs you to use them elsewhere in
*   the code to figure out what type you mean.
*/

// When defining multiple parameters, separate the parameter declarations with commas
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

/*
*   Function bodies are made up of a series of statements optionally ending in an
*   expression. So far, the functions we've covered haven't included an ending expression,
*   but you have seen an expression as part of a statement. This is important, as Rust is an
*   expression-based language. Other languages don't have the same distinctions.
*/

// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value.

fn statements_and_expressions() {
    // This is an example of a statement.
    let y = 6;

    println!("{y}");

    /*
    This is an example of an incorrect statement.
    let x = (let y = 6);
     */
    // Function definitions are also statements; the entire preceding example is a statement in itself.
    // Statements do not return values. Therefore, you can’t assign a let statement to another variable.
}
/*
*   The let y = 6 statement does not return a value, so there isn’t anything for x to bind to.
*   This is different from what happens in other languages, such as C and Ruby,
*   where the assignment returns the value of the assignment.
*   In those languages, you can write x = y = 6 and have both x and y have
*   the value 6; that is not the case in Rust.
*/

// A new scope block created with curly brackets is an expression
fn new_scope_block() {
    let y = {
        let x = 3;
        x + 1
        /*
        Note that the x + 1 line doesn’t have a semicolon at the end,
        which is unlike most of the lines you’ve seen so far. Expressions do not
        include ending semicolons. If you add a semicolon to the end of an expression,
        you turn it into a statement.
        */
    };

    println!("The value of y is: {y}");
}

/*
*   Functions can return values to the code that calls them. We don’t name return values,
*   but we must declare their type after an arrow (->). In Rust, the return value of the
*   function is synonymous with the value of the final expression in the block of the body
*   of a function. You can return early from a function by using the return keyword and specifying
*   a value, but most functions return the last expression implicitly.
*/

// Example of a function that returns a value, the keyword "return" is implied
fn five() -> i32 {
    5
}

// Example of the same type of function with explicitly stating "return"
fn six() -> i32 {
    return 6;
}

// Entry point
fn main() {
    println!("Hello, world!");

    // Calling the "snake_case" function
    snake_case();

    // Calling the "fun_parameters" function and specifying x to the value 5
    fun_parameters(5);

    // Calling the "print_labeled_measurement" function and specifying the int and char
    print_labeled_measurement(5, 'h');

    statements_and_expressions();

    new_scope_block();

    let x = five();
    let y = six();

    println!("The value of x is: {x}, and the value of y is: {y}");
}
