Examples

fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

Exercises

1)

/* // Make it work with two ways
fn main() {
   let v = {
       let mut x = 1;
       x += 2
   };

   assert_eq!(v, 3);

   println!("Success!");
} */


Solutions i) 
fn main() {
   let v = {
       let mut x = 1;
       x += 2
   };

   assert_eq!(v, ());

   println!("Success!");
}

output: Success!

Solutions ii) 

fn main() {
   let v = {
       let mut x = 1;
       x += 2;
       x
   };

   assert_eq!(v, 3);

   println!("Success!");
}

output: Success!

Questions 2) 

/* fn main() {
   let v = (let x = 3);

   assert!(v == 3);

   println!("Success!");
} */

Solutions : 

fn main() {
   let v = {
            let x = 3;
            x
   };

   assert!(v == 3);

   println!("Success!");
}

output: Success!

Questions 3) 

/* fn main() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y;
} */

Solutions : 

fn main() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
output: Success!
