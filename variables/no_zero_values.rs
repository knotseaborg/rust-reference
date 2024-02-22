/**
 * Unlike Go, rust has no zero-values
 * Every declared variable binding can only be used, if it is initialized.*/

fn main() {
    let a_binding;

    {
        let x = 2;
        a_binding = x*x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;
    /*
     * Here another_binding is bound to nothing, and if used will raise error.*/
    // println!("another binding: {}", another_binding); // -> ERROR!

    another_binding = 1;
    println!("another binding: {}", another_binding);
}
