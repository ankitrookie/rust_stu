use std::fs;

/* Generics in Rust, are same as in TS */
struct Info<N, A> {
    name: N,
    age: A,
}

/* Result enums is not somthing we provide, it is already defined by Rust itself */
/* https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html?highlight=enum%20result#recoverable-errors-with-result */

/* enum Result<T, E> {
    Ok(T),
    Err(E),
} */
/* https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html?highlight=options%20enum#the-option-enum-and-its-advantages-over-null-values */
/* Options enum in rust, this is just a how itstructure looks like, it is not like enums, types etc like ts. and above is link to read about enums, scroll down, ll find option enum */
/* enum Option<T> {
    None,
    Some(T),
} */

fn main() {
    // Error handling in Rust

    /*  try catch is similer concept in js, which is for error handling */

    /* In Rust we do it with Result Enum */
    let user_info = Info {
        name: "Ankit",
        age: 25,
    };

    println!(
        "My name is {}, I am {} years old.",
        user_info.name, user_info.age
    );

    println!("-------------- Above are example of Generics in Rust ---------------");

    /* let can_be_exists_or_not = fs::File::open("hello.txt");
    let can_be_exists_or_not_two = fs::read_to_string("hello.txt"); */

    /* The panic in Rust, used to terminate the current thread immediately and provide feeback to he caller of the program, what happned... and wont for further the thread */
    /* match can_be_exists_or_not {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file {:?}", error),
    }; */

    /* match can_be_exists_or_not_two {
        Ok(content) => println!("File content: {}", content),
        Err(error) => println!("Error {}", error),
    }; */

    println!("Thread did not crash");

    /* In Rust we cant directly print result, cause it is Result enum with two variants (Ok, Err), so need to pattern match first and print that return value. */
    let result = fun_impl_of_erro_handaling();

    match result {
        Ok(content) => println!("File content: {}", content),
        Err(err) => println!("Error: {}", err),
    }

    println!("-------------- Above are example of error handaling in Rust, and Result enum ---------------");

    /* the option enum is an enum that rust provides you to get rid of null types through out you
    codebase, in fact rust dose't have concept of null type in enum */

    /* The Option type encodes the very common scenario in which a value could be something or it could be nothing. */

    let some_num: Option<i32> = Some(5);
    let absent_number: Option<i32> = None;

    print_option_value(some_num);
    print_option_value(absent_number);

    println!("-------------- Above are example of Option enum ---------------");
}

// this says i gonna return two types of return string, string or it can be number string etc.
fn fun_impl_of_erro_handaling() -> Result<String, String> {
    let can_be_exists_or_not_two = fs::read_to_string("hello.txt".to_string());

    match can_be_exists_or_not_two {
        Ok(content) => return Ok(content),
        Err(_) => return Err("Error reding file".to_string()),
    };
}

fn print_option_value(option: Option<i32>) {
    match option {
        Some(value) => println!("The value is: {}", value),
        None => println!("No value present"),
    }
}
