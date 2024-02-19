#[allow(dead_code)]
/**
 * Need to derive Debug.
 * "derive" is a directive which tells the compiler to generate implementation for std::fmt::Debug, which is a trait.
 * An analogy to understand #derive macro is liken it to a @decorator in python, which enhances the behaviour of the type.
 * 
 * Not all traits can be "dervived"
 * "derive" has been specifically created to auto-generate implementation for boilerplate traits.
 * Rust has made this design decision to ensure awareness of the implemented traits in users.
 * Another added benefit from this design decision might be to make the code lighter. In production, since there is no necessity to debug stuff, this piece can be eliminated with ease.
*/
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

/**
 * Unit structs are special because they do not contain any fields, neither can they be modified to include fields.
 * They merely exist to implement traits.
 * 
 * Still unclear how they can be effectively used in programming.
*/
struct Unit;

/**
 * The tuple struct differs from the usual struct as they do not have field names.
*/
struct Pair(i32, i32);

/**
 * See! having this annoying piece of code is a reminder that we, the user hasn't implemented the necessary trait.
*/
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

/**
 * Lo, a struct within a struct.
*/
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    /**
     * This is how you print a struct.
     * Thanks to the std::fmt::Debug trait, we can print it using ":?"
    */
    println!("{:?}", peter);

    let base_point = Point { x: 1.5, y: 2f32 };
    /**
     * This is how you create  new struct by overwritin values from an older struct.
    */
    let new_point = Point {
        x: 2.0,
        y: 3.0,
        ..base_point
    };
    println!("{:?}", base_point);
    println!("{:?}", new_point);

    /**
     * I like this a lot, as it allows "unwrapping" of values of complex structs.
     * This is also called as "Destructuring" a struct
    */
    let Point {
        x: x_coordinate,
        y: y_coordinate,
    } = base_point;
    println!("The x coordinate is {x_coordinate}, The y coordinate is {y_coordinate}");

    let new_pair = Pair(5, 6);
    /**
     * Here again, we destructre a tuple struct
     */
    let Pair(integer, decimal) = new_pair;
    println!("The content of new pair include: {}, {}", integer, decimal);
}
