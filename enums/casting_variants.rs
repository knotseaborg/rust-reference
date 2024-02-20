/**
 * Looks like enums in rust can also behave like standard c-like enums
 * You can create alias for enums
 * You can explicitely assign values of isize to enum variants, but typecasting is always required for comparisons
 * If any of the enum variants have associated data, then any of the enum variants (even unit variants) cannot be typecasted into isize*/

#[derive(Debug)]
enum MyFavouriteNumberEnum { // The name is so verbose
    Zero,
    One,
    Two,
    /*
    * If you uncomment the line below, then the variants from the enums cannot be casted into intergers.
    * */
    // Any(i32),
}

// type can be used to create an alias for the verbose name
type Number = MyFavouriteNumberEnum;

enum Color {
    /*Mere hexadecimal representationsof isize*/
    Red = 0xff000,
    Green = 0x00ff00,
    Blue = 0x000ff,
}

fn main() {
    // println!("zero is actually {:?}", Number::Any(9));
    /*
     * Cannot be type casted if the Any(i32) is uncommented
     */
    println!("zero numerically is {}", Number::Zero as i32);

    println!("roses are #{:06x}", Color::Red as i32); 
    println!("violets are {}", Color::Blue as i32);
}
