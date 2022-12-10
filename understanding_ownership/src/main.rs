#[allow(unused_variables)]
fn main() {
    let x = 5;
    let y = x;
    println!("{} {}", x, y);
    /* this is valid because integers are stored on stack a
       nd not on heap because they have fixed size on compile time
    */
    let s1 = String::from("Hello world");
    let s2 = s1;
    //  println!("{}", s1);
    /* MOVE
    1. this is invalid as the s1 is moved
    2. string data is stored on heap so its not copied (copying on heap is expensive) by default rather
    we added new pointer to it, and invalidated the previous one -> this is called move in rust.
    3. MOVE => shallow copy + invalidation of previous pointer
    */
    // ===========================================
    /*
       1. The above problem does not occurs with integers
       2. Integers are stored on stack memory and therefore
           the values are moved rather copied, as coping on stack is fast and inexpensive
    3. No difference in deep & shallow copy here
    */
    // ===========================================
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("{} {}", s3, s4);
    /* CLONE
    1. The above operation is valid as clone method makes a copy of string data on heap. aka. DEEP COPY
    2. This operation is expensive.
    3. s3 and s4 points to different memory locations.
    */
}
