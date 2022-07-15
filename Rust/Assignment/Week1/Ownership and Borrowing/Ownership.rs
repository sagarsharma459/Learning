Ownership

1) 

/* fn main() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    let y = x;
    println!("{},{}",x,y);
} */


Solutions : 

fn main() {
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}",x,y);
}

Output : hello, world,hello, world

2) 

/* // Don't modify code in main!
fn main() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) {
    println!("{}", s);
} */

Solutions : 

fn main() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

Output : 
hello, world
hello, world

3) 

/* fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    // Convert String to Vec
    let _s = s.into_bytes();
    s
} */

Solutions : 

fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    // Convert String to Vec
   // let _s = s.into_bytes();
    s
}

Output : 
hello, world

4) 

/* // Fix the error without removing code line
fn main() {
    let s = String::from("hello, world");

    print_str(s);

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
} */


Solutions: 

fn main() {
    let s = String::from("hello, world");

    print_str(s.clone());

    println!("{}", s);
}

fn print_str(s: String) {
    println!("{}",s)
}

Output : 

hello, world
hello, world

5) 

/* // Don't use clone ,use copy instead
fn main() {
    let x = (1, 2, (), "hello".to_string());
    let y = x.clone();
    println!("{:?}, {:?}", x, y);
} */


Solutions : 

fn main() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}

6) 

/* fn main() {
    let s = String::from("hello, ");
    
    // Modify this line only !
    let s1 = s;

    s1.push_str("world");

    println!("Success!");
} */

Solutions: 

fn main() {
    let s = String::from("hello, ");
    
    // Modify this line only !
    let mut s1 = s;

    s1.push_str("world");

    println!("Success!");
}

Output : Success!

7) 

/* fn main() {
    let x = Box::new(5);
    
    let ...      // Implement this line, dont change other lines!
    
    *y = 4;
    
    assert_eq!(*x, 5);

    println!("Success!");
} */


Solutions : 

fn main() {
    let x = Box::new(5);
    
    let mut y = Box::new(3);       // Implement this line, dont change other lines!
    
    *y = 4;
    
    assert_eq!(*x, 5);

    println!("Success!");
}

Output : Success!

8) 

/* fn main() {
   let t = (String::from("hello"), String::from("world"));

   let _s = t.0;

   // Modify this line only, don't use `_s`
   println!("{:?}", t);
} */

Solutions : 

fn main() {
   let t = (String::from("hello"), String::from("world"));

   let _s = t.0;

   // Modify this line only, don't use `_s`
   println!("{:?}", t.1);
}

Output : "world"

9) 

/* fn main() {
   let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (__, __) = __;

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
} */

Solutions: 

fn main() {
   let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (s1, s2) = t.clone();

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}

Output : "hello", "world", ("hello", "world")
