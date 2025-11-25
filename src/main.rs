fn main() {
    // s is not valid here, it’s not yet declared
    // {  

    // let s = "Hello"; // s is valid from this point forward


    // }
    // s is not valid here, it’s out of scopeD
    // let a = 5; // is a literal, stored on the stack
    // let mut s = String::from("Hello"); // the :: indicates that we’re calling an associated function of the String type
    // // this string can be mutated
    // s.push_str(", wolrd");
    // println!("{}", s);

    let x = 5;           // literal, immutable, stack, ALWAYS GETS MAXIMNUM AMOUT OF ALLOCATED MEMORY FOR ITS TYPE, INCLUDING WHEN ITS VALUE IS SMALLER
    let y = x;           // literal, immutable, stack
    // the x is being copiead by y and both put onto the stack
    // gets the exact data from x and copies it to y

    let s1 = String::from("hello");
    let s2 = s1; // but this copies(moves) not the actual data, but a pointer to the data on the heap, so basically a name,
    // capasity and length, but the data is the same as from s1
    // gets a pointer(place where the data is) and the info for this data, but doesnt copy the actual data

    // println!("{}", s1); // if we try to use s1, it will throw an erorr,
    // bc s1 went out of scope after being 'moved' to s2
    // alsol if we try to get the s2 out of scope, it will also throw an error bc both s1 and s2 try to free the same memory 
    // which can lead to a double free error which will lead to memory corruption and security vulnerabilities

    // therefore rust will never automatically create 'deep'(move) copies of data to prevent these issues

    let mut s = String::from("hello"); // and its differ from literals is bc literals are stored on the stack and have a known fixed size at compile time
    // but String is stored on the heap and can grow and shrink in size
    s = String::from("ahoy");// rust will drop the first value of s, and allocate new memory for the new value
    
    println!("{s}, world!"); // this will print "ahoy, world!", bc new data comes on the old memory location
    // basically adding new s to old s
    // Memory location = Owner
    // and only 1 owner for 1 value at a time

    let s1 = String::from("hello");
    let s2 = s1.clone(); // the clone method will create a deep copy of the data on the heap
    // so both s1 and s2 are valid and own their own data, and the pointer data will be different
    // but this can be expensive for large data structures
    println!("s1 = {s1}, s2 = {s2}");

    
    // annotation tyoe is called Copy trait, and it is for types that have a known fixed size at compile time and are stored on the stack
    // these types include all the integer types, the boolean type, the floating point types, the char type
    // and tuples that only contain types that are also Copy types
    // Copy types do not implement the Drop trait, so they are not deallocated when they go out of scope
    // therefore they can be copied instead of moved
    // but bc of this they are stored on the stack, they are limited in size and cannot grow or shrink like heap data
    // the copy with a Copy trait gets stored where they are assigned
    
    // let s1 = String::from("hello");
    // let s2 = s1.clone();  // s2 owns its own copy, s1 still valid
    // println!("{}, {}", s1, s2);  // ✓ both valid

    // let s3 = &s1;  // s3 borrows s1, doesn't own it
    // let s4 = &s1;  // s4 also borrows s1
    // println!("{}, {}", s1, s3);  // ✓ s1 still the owner, s3/s4 just reference it


//     fn main() {
//     let s = String::from("hello");  // s comes into scope

//     takes_ownership(s);             // s's value moves into the function...
//                                     // ... and so is no longer valid here

//     let x = 5;                      // x comes into scope

//     makes_copy(x);                  // Because i32 implements the Copy trait,
//                                     // x does NOT move into the function,
//                                     // so it's okay to use x afterward.

// } // Here, x goes out of scope, then s. However, because s's value was moved,
//     // nothing special happens.

//     fn takes_ownership(some_string: String) { // some_string comes into scope // basically we move s into some_string
//         println!("{some_string}");
//     } // Here, some_string goes out of scope and `drop` is called. The backing
//     // memory is freed.

//     fn makes_copy(some_integer: i32) { // some_integer comes into scope // basically we copy x into some_integer
//         println!("{some_integer}");
//     } // Here, some_integer goes out of scope. Nothing special happens.

// fn main() {
//     let s1 = gives_ownership();        // gives_ownership moves its return
//                                        // value into s1

//     let s2 = String::from("hello");    // s2 comes into scope

//     let s3 = takes_and_gives_back(s2); // s2 is moved into
//                                        // takes_and_gives_back, which also
//                                        // moves its return value into s3
// } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
//   // happens. s1 goes out of scope and is dropped.

// fn gives_ownership() -> String {       // gives_ownership will move its
//                                        // return value into the function
//                                        // that calls it

//     let some_string = String::from("yours"); // some_string comes into scope

//     some_string                        // some_string is returned and
//                                        // moves out to the calling
//                                        // function
// }

// // This function takes a String and returns a String.
// fn takes_and_gives_back(a_string: String) -> String {
//     // a_string comes into
//     // scope

//     a_string  // a_string is returned and moves out to the calling function
// }


// fn main() {
//     let s1 = String::from("hello"); 

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{s2}' is {len}.");
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() returns the length of a String

//     (s, length) // we can also return multiple values using a tuple
// }
}   
