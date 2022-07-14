1) 

/* fn main() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x, y: i32) {
    x + y;
} */

Solutions : 

fn main() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x:i32, y: i32) -> i32 {
    x + y
}

Output : Success!

2) 

/* fn main() {
   print();
}

// Replace i32 with another type
fn print() -> i32 {
   println!("Success!");
} */

Solutions : 

fn main() {
   print();
}

// Replace i32 with another type
fn print() -> () {
   println!("Success!");
   
}

3)

/* // Solve it in two ways
// DON'T let `println!` works
fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    // Implement this function, don't modify the fn signatures
    
} */

Solutions : 

fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
   
   panic!("Never RETURN");
    
}

Output : 

  Compiling playground v0.0.1 (/playground)
warning: unreachable statement
 --> src/main.rs:6:5
  |
4 |     never_return();
  |     -------------- any code following this expression is unreachable
5 | 
6 |     println!("Failed!");
  |     ^^^^^^^^^^^^^^^^^^^ unreachable statement
  |
  = note: `#[warn(unreachable_code)]` on by default
  = note: this warning originates in the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: `playground` (bin "playground") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.56s
     Running `target/debug/playground`
thread 'main' panicked at 'Never RETURN', src/main.rs:11:4
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

4) 

/* fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    
    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    
} */
 
  Solutions : 
  

  
  fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    
    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    panic!()
    
}
  5) 
  
  /* fn main() {
    // FILL in the blank
    let b = __;

    let v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
} */
  
  Solutions : 
  
  fn main() {
    // FILL in the blank
    let b = false;

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}

  Output : 
  
  Compiling playground v0.0.1 (/playground)
    Finished dev [unoptimized + debuginfo] target(s) in 0.85s
     Running `target/debug/playground`
thread 'main' panicked at 'we have no value for `false`, but we can panic', src/main.rs:11:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Success!

  
  


