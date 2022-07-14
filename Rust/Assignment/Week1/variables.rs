Questions 1) : A variable can be used only if it has been initialized.
/* Fix the error below with least amount of modification to the code
fn main() {
    let x: i32; // Uninitialized but used, ERROR !
    let y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}
*/

Solutions :

fn main() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}

Output : Success!


 Question 2) : Use mut to mark a variable as mutable.
/*
// Fix the error below with least amount of modification to the code
fn main() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}
*/

Solutions :

fn main() {
    let mut x =  1;
    x += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
}

Output : Success!

Question 3) : A scope is the range within the program for which the item is valid.
/*
// Fix the error below with least amount of modification
fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); 
}
*/

Solutions: 
fn main() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {}", x); 
}

Output : 
The value of x is 10 and value of y is 5
The value of x is 10

Questions 4) :  Fix the error with the use of define_x
/*
fn main() {
    println!("{}, world", x); 
}

fn define_x() {
    let x = "hello";
}
*/

Solutions :

fn main() {
    let x = define_x();
    println!("{}, world", x); 
}

fn define_x() -> String {
    let x = "hello";
    x.to_string()
    
}

Output : hello, world

Question 5) :  Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)

/*
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 5);
    }

    assert_eq!(x, 12);

    let x =  42;
    println!("{}", x); // Prints "42".
}
*/

Solutions :

fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x =  42;
    println!("{}", x); // Prints "42".
}

Output : 42

Question 6) Remove a line in the code to make it compile

/*
fn main() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let x = x; 
    x += 3;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 

    println!("Success!");
}
*/


Solutions : 

fn main() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let x = x; 
    


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 

    println!("Success!");
}

Output : Success!

Questions 7) : Fix the warning // Warning: unused variable: `x`
/*
fn main() {
    let _x = 1; 
}
*/

Soultions 1 for this : used _(underscore) before variable 

fn main() {
    let _x = 1; 
}

Solutions 2 for this : use the check equality using assert_eq! macro
 fn main() {
        let  x = 1;
        assert_eq!(x,1);

    }

Questions 8 ) Fix the error below with least amount of modification
/*
fn main() {
    let (x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}
*/

Solutions : 

fn main() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

Output : Success!

Question 9) // Fill the blank to make the code work
/*
fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], __);

    println!("Success!");
} 
*/

Solution : 

fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3,2]);

    println!("Success!");
}

Output : Success!

