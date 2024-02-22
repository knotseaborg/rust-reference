/*
 * Take heed to the terminology. 
 * In rust variables do not "have" data, but are "bound" to data.
 * */
fn main() {
    /*
     * The variable name is prefixed with _, as it is unused.
     * It instructs the rust compiler to not raise warnings related to lack of use*/
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    /*
     * This is allowed!*/
    mutable_binding -=1;
    println!("After mutation: {}", mutable_binding);

    /*
     * Try uncommenting and check it out. This will lead to error!*/
    // _immutable_binding += 1;
}
