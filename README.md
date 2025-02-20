# References and Borrowing in Rust
A reference to a String value is like a pointer in that its an address we can followto access the data stored at that address, that is owned by some other variable. Unlike a pointer though, a reference is guaranteedto point to a valid valueof a particular type for the life of that reference.
If we were to describe a Picture of a reference - we would have s with (name and value WITH a pointer) -> to s1 with also (name and value) s1 has a len and capacity PLUS a -> pointing to the STACK where there is (name, value) and an address for the string 'HELLO' and the corresponding indexes like ( 0 H 1 E 2 L 3 L 4 O).
NOTE: The opposite of referencing by using & is dereferencing, which is accomplished by with the dereference operator, *.

Looking closely at the function call:

let s1 = String::from("hello");
let len = calculate_length(&s1);
The &s1 syntax lets us create a referenc that referes to the value of s1 but does not own it. Because 
it does not own it, the value it points to will not be dropped when the reference stops being used.

Likewise, the signature of the function uses & to indicate that the type of the parameter s is a refrence .

