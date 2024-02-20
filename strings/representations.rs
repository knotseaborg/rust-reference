/**
 A study of String, &str and inter-casting between them
 
 &str::to_string() internally uses String::from().
 Thus their usage depends on preference.
 * */
fn main() {
    /*
     Here, "Hello World!" is a string literal. Which is stored in the script itself in binary.
     s_slice is of type &str, which is a reference to a "string slice", which is essentially a slice of characters/bytes. 

     [Slices in rust and similar to the slices in go]
     */
    let s_slice = "Hello World!";
    /*
     The reference to the string slice is being used to generate a String object.
     String is a heap allocated object.
     */
    let mut s_string1 = s_slice.to_string();
    let mut s_string2 = String::from(s_slice);
    /*
     String are mutable, but &str (string slices) are immutable.
    */
    s_string1.push_str(" Is is this Earth?");
    s_string2.remove(0);

    println!("These are the modified strings : \"{}\" and \"{}\"", s_string1, s_string2);
    println!("This is the original string : {}", s_slice);
}
