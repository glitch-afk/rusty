pub mod loops;

#[allow(unused_variables, unused_assignments, dead_code, unused_mut)]

fn main() {
    /* VARIABLES */
    let x = 3; // immutable by default
               // x = 6; -> will result in error

    let mut y = 12; // mut keyword make variables mutable
    y = 10; // y = 10
    y += 4; // y = 14

    /* CONSTANTS */
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    /* SHADOWING */
    let guess = "hello_rust";
    let guess = 12; // compiler will see this guess variable
    {
        let guess = true; // guess = true
    } // scope ends -> guess = 12

    // SHOWDING is different from mutability (`mut`) as we have to use the `let` keyword to reassign the value otherwise we'll get the compile time error

    // 💡 we cannot mutate a variables type but can reassign it using `let` keyword
    let mut spaces = "   ";
    // spaces = spaces.len() -> ❌

    let spaces = "   ";
    let spaces = 3; // ✅

    loops::loops();
}
