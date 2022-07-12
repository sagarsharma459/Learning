Binding and mutability
🌟 A variable can be used only if it has been initialized.


// Fix the error below with least amount of modification to the code

    
    fn main() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
    }
  
    
    Success!
    
    Solution : To remove Error initialize x = 5; and to remove warning put _(underscore before y).
