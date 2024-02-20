/**
 * In structs, we have field names and their associated values.
 * However in Enums, we work with VARIANTS, not FIELDS.
 * The conventions followed in naming variants is similar to that of naming types, which is
 * CapitalizedCamelCase
 *
 * You can view variant as a form of struct, but they are NOT structs.
 *
 * And lastly, what makes variants cool is that you can also implement traits, just like with their
 * cousins, structs.
 * */

enum WebEvent{
    /*
     * The variants can be unit variants of structs*/
    PageLoad,
    PageUnload,
    /*
    * Or they can also be tuple-like structs*/
    KeyPress(char),
    Paste(String),
    /*
    * Or they can be expressive c-structs*/
    Click {x: i64, y: i64},
}

fn inspect(event: WebEvent) {
    /*
     * Here lies the strength of using Enums. You can have such expressive selection using "match".
     * */
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => {let s = "page unloaded"; println!("{}",s)},
        /*
        *This is how you destructure the tuple-like-struct variant
        * */
        WebEvent::KeyPress(c) => println!("pressed {c}"),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),
        /*
        *This is how you destructure the c-like-struct variant!
        * */
        WebEvent::Click{x, y} => println!("clicked at x={x}, y={y}"),
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('R');
    let s = "Hello World!";
    let pasted = WebEvent::Paste(s.to_string());
    let click = WebEvent::Click{x:10, y:100};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
