fn main() {
    let s = "hello";
    println!("The value of s is: {s}");

    let mut s = String::from("hellows");

    s.push_str(", world");

    println!("{s}");

    // Scope and Assignment - memory management in the stack and heap

    let s1 = String::from("Botani");
 // let s2 = s1;    // this code produces an error, Rust will invalidate (use DROP function) s1, and only s2 will be managed, as it runs out of scope, 
                    // only s2 will free memory for use.

    println!("{s1}, world");

    // Scope and Alignment ..consider!

    let mut s = String::from("hello"); // Rust drops the original string and goes out of scope to manage memory.
    s = String::from("ahoy");

    println!("{s}, world");

   // The CLONE method - {deeply copies heap data of the string, not just stack data}-
   // Variables and Data interacting with clone
   
   let _s1 = String::from("hello");

   let s1 = String::from("hello");
   let s2 = s1.clone();

   println!("s1 = {s1}, s2 = {s2}");


   // Ownership and Functions

//   fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

//} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.



// Return Values and Scope - return values can also transfer ownership

//fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
//} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

// Rust does allow returning multiple values using a tuple

// fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
//}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

// References and Borrowing in Rust - a referece is like a pointer in that its an address we can follow to access the data
// stored at that address, that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to pint to a valid value 
// of a particular type for the life of that reference.

// fn main() {
        let s1 = String::from("hello");
        let len = calculate_length(&s1);

        println!("The length of '{s1}' is {ln}.");
//}
  fn calculate_length(s: &String) -> usize {
        s.len()
  }



}
