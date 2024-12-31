fn main() {
    let mut x = 5;
    { // Using a block to limit the scope of the mutable borrow
        let y = &mut x; 
        *y += 1; 
    }
    { // A new block to create the second mutable borrow without conflict.
        let z = &mut x; 
        *z +=1;
    }
    println!("x = {}", x);
}