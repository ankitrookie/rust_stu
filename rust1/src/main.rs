fn main() {
    /* data types */
    let x: i8 = 34;
    let y: i16 = 3244;
    let z: i32 = 2323423;
    let e: f32 = 23.3;
    println!("x: {}, y: {}, z: {}, e: {}", x, y, z, e);

    /* boolean if else */
    let is_male: bool = true;
    let is_above_18 = true;

    if is_male {
        println!("You are a male!")
    } else {
        println!("You are not a male!")
    }

    if is_male && is_above_18 {
        println!("You are a legal male")
    }
    /* <!---------------------- String ---------------------!> */

    /* string is little complicated, it seems it is easy but not easy as JS
    the reason is, lets say the length of the string jincreases in run time. */

    /* initally we assigned 8 byts for our string -> but it turned bigger in
    run time, so this is the reason string becomes complicated */

    let greeting = String::from("Sring in Rust");
    println!("{}", greeting);

    // here we am printing the specific index letter,
    let char1: Option<char> = greeting.chars().nth(1000);
    /* as you can see we are going out of index here, if it goes out of index it
    will not return undefined like JS, */

    // :: solution
    // this is pattern matcing
    match char1 {
        Some(c) => println!("{}", c),
        // if something is there in that index, then print that
        None => println!("No character at this index"),
        // if it is out of index. or none print that string
    }

    // conditional
    let legal_age: bool = true;
    if legal_age {
        println!("{}", "you are 18, you can drive!")
    } else {
        println!("You are under age, can't drive!")
    }

    // loops
    for i in 0..10 {
        println!("{}", i)
    }

    let sentence = String::from("my name is ankit");
    let first_word = get_first_word(sentence);

    println!("first word is : {}", first_word);

    /* ::: STACK :::
    Rust uses stack for most premative data types, where size is known in compole time. */
    /* ::: HEAPS :::
    String are stored on the heap. Heap stores those data, that can grow at run time. */
    // ex: victore, or string, this heap process is quite slow compater to stack

    stack_fn(); // Call the function that uses stack memory
    heap_fn(); // Call the function that uses heap memory
    update_string(); // Call the function that changes size of variable at runtime
    rust_ownership();

    // let sl2 = String::from("Hello Rust 2");
    // rust_ownership_second(sl2);
    // println!("{} this will complain coz the sl2 is not longer the owner of the that, rust_ownerhip_second has become owner now : ", sl2)
    rust_refrences()
}

fn get_first_word(sentence: String) -> String {
    let mut ans: String = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;
}

fn stack_fn() {
    // Declare a few integers on the stack
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("Stack function: The sum of {} and {} is {}", a, b, c);
}

fn heap_fn() {
    // Create a string, which is allocated on the heap
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{} {}", s1, s2);
    println!("Heap function: Combined string is '{}'", combined);
}

fn update_string() {
    // Start with a base string on the heap
    // and if it goes out of the capacity, it will change pointer somewhere else, cause right after
    // that specific Capacity something elase could be running
    let mut s = String::from("Initial string");
    println!("Before update: {}", s);
    println!(
        "Capacity: {}, Length: {}, Pointer: {:p}",
        s.capacity(),
        s.len(),
        s.as_ptr()
    );

    // Append some text to the string
    s.push_str(" and some additional text");
    println!("After update: {}", s);
    println!(
        "Capacity: {}, Length: {}, Pointer: {:p}",
        s.capacity(),
        s.len(),
        s.as_ptr()
    );
}

/****************************** OwnerShip ******************************/

fn rust_ownership() {
    // this is owner ship,
    // 1. sl create stack and heap in it way
    // and then we are assigning our sl to si
    // 2. the first sl will no longer has control over String now,
    // it has move to si, the owenerof "Hello Rust!" is now
    // si, now if we try to print sl, it wont compile
    let sl = String::from("Hello Rust!");
    rust_ownership_second(sl);
    /*     let si = sl; */
    /*     println!("{}", si); */
}

fn rust_ownership_second(some_string: String) {
    // this is little diff, it will take that initial defined string varable
    // as an parameter, and this function will be owner of this that string
    println!("{}", some_string);
}

/****************************** Refrences ( we use "&" sign for indicating refrence ) ******************************/

fn rust_refrences() {
    // 1st way of borrowing
    let my_string =
        String::from("Refrence means just borrowing, not handaling over whole ownership.");
    // 2nd way of borrowing
    let s2 = &my_string;
    /* here mystring is being passed to function, but mystring is still main owner of it, so we are using & to pass string as a parameter */
    borrow_veriable(&my_string);
    println!("{}", s2);

    // 3rd
    let mut mut_string = String::from("Hello");
    // this line is alrady a 1st borrower
    /*     let s2: &mut String = &mut mut_string; */
    mutable_borrowing(&mut mut_string);
    println!("{}", mut_string);

    // we have to remember that we can only have single mutable, at a time we can do multiple
    /* note:
    even tho you have mutated it once, but havent used it, the compiler is smart enough to
    know the you havent used  it yet, so it will not complain if you mutate it for third time */
    // println!("{}", s2);
}

fn borrow_veriable(some_string: &String) {
    println!("{}", some_string)
}

/****************************** Mutable borrowing (meaning to change it in run time, we supposed to make that passing parameters mutable, and even make the receaving side called argunment mutable ), and we can have one mutable refrences. ---"but we can have manu emutable refrences"--- ******************************/

fn mutable_borrowing(s: &mut String) {
    s.push_str(" world!");
}

/****** Rules for borrowing, so even borrowing, makes it memory safe ******/
// 1. we can only have one mutalble refrences at a time
// 2. let's say we have two mutable refrneces but we are not updating that, it will run, coz compiler is smart to knwo that you diden't update it. below ex ;
fn rule_one() {
    let mut s1 = String::from("Hello");
    let s2 = &mut s1;

    let s3 = &mut s1;
    print!("{}", s3);
}
// 3. now we are doing the smae thing but, this time we are updation that mutable value and try to borrow as same as previous time. ex below :
fn rule_two() {
    let mut s1 = String::from("Hello");
    let s2 = &mut s1;
    // it has been borrowed more then once
    let s3 = &mut s1;

    s2.push_str("world");

    print!("{}", s3);
}

// fn truthy() -> bool {
//     return true;
// }
//
// #[cfg(test)]
// mod test {
//     use super::truthy;
//
//     #[test]
//     fn test_something() {
//         assert_eq!(truthy(), true)
//     }
// }
