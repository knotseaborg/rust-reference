/**
 * Let's take a look at how shadowing works in rust.
 * */

fn main() {
    let shadowed_binding = 1;
    {
        println!("Before being shadowed: {}", shadowed_binding);
        /*                                                                                
        * Re-binding of the variable causes shadowing.*/
        let shadowed_binding = "abc";
        println!("After being shadowed: {}", shadowed_binding);
    }
    /*
    * The shadowing is only limited to the block only.*/
    println!("outside inner block: {}", shadowed_binding);
}
