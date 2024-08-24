struct User {
    name: String,
    age: u32,
    active: bool,
}

/* https://doc.rust-lang.org/book/ch05-01-defining-structs.html?highlight=struct#defining-and-instantiating-structs */

// i also have to learn about side quest - Learning about traits
// next , what is copy and clone. And how are hey diff from each other in Rust

/* tuple struct */
struct Color(i32, i32, i32);
struct Name(String, String);

/****************************** struct implementation **************************/
/* This struct implementation makes, structs in rust very close to class in JS. Which means you can attach function to instances of structs. */

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    /* self is rust as being similar to `this` in js, both refer current instance  */
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    /* Let's go... lets do structs */
    /* tuple struct, unit struct */
    let name = String::from("Ankit");
    let user = User {
        name,
        age: 23,
        active: true,
    };

    // tuple struct
    let black = Color(0, 0, 0);
    let name = Name(String::from("Ankit"), String::from("Mukhia"));

    let mut online = String::from("");

    if user.active {
        online.push_str("Online");
    }

    /* Creating an instance of Rect, known as struct literal in Rust */
    let my_struct_implementation_ruct = Rect {
        width: 5,
        height: 10,
    };

    println!(
        "{} is {} years old. And I am {}",
        user.name, user.age, online
    );
    println!(
        "Color - Red: {}, Green: {}, Blue: {}",
        black.0, black.1, black.2
    );
    println!("My name is {} {}", name.0, name.1);

    println!("{}", my_struct_implementation_ruct.area());

    /* enum part */

    println!("------------------------ Below are enums ------------------------");
    enums();
}

enum Directions {
    North,
    East,
    South,
    West,
}
fn enums() {
    let my_direction = Directions::North;
    move_around(my_direction);
}

fn move_around(my_direction: Directions) {
    // this match method is called pattern matching
    match my_direction {
        Directions::North => println!("Moving North"),
        Directions::East => println!("Moving East"),
        Directions::South => println!("Moving South"),
        Directions::West => println!("Moving West"),
    }
}
