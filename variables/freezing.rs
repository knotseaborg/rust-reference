/**
 * Freezing can be observed when used with variable bindings of types which implement the Copy trait */

fn main() {
    with_copy_trait();
    without_copy_trait();
}

fn with_copy_trait() {
    let mut _mutable_integer = 7i32;
    {
        /* 
         * Shadowing by immutable `_mutable_integer`, where the value of _mutable_integer from the outer scope is copied into the _mutable_integer in th inner scope*/
        let _mutable_integer = _mutable_integer;

        /* 
         * _mutable_integer is frozen in this scope. Uncomment to find out*/
        // _mutable_integer = 50; // -> ERROR!
    }

    /* `_mutable_integer` is not frozen here*/
    _mutable_integer = 3;
}

/*
 * Here we do not work with primitive typed variable bindings, thus the "shadowing" attempted essentially executes a "move", transferring the ownership into the inner scope.*/
struct Pair(i32,i32);

fn without_copy_trait(){
    let mut _mutable_struct = Pair(5,6);

    {
        /*
         * Attempted "Shadowing" by immutable `_mutable_struct`. Since Copy trait isn't implemented, trasfer of ownership takes place */
        let _mutable_struct = _mutable_struct;
    }

    /*
     * _mutable_struct has lost ownership after the "move", causing an error!*/
    // _mutable_struct.0 += 10; // -> ERROR!
}
