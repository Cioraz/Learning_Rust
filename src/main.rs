
fn main(){
    let mut x = 5;
    // It is a mutable variable hence its value can be changed during program course
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x now is {x}");

    // constants are also like mutable variables but constants are always immutable
    // Convention is all uppercase

    const THREE_HOURS_IN_SECONDS: u32= 60*60*3;
    /*
    signed and unsigned
    8-bit = i8/u8
    16-bit = i16/u16


    */

    let var : u8 = 255;

    let x = 2.0; // nothing specified so f64

    // Floating
    let y: f32 = 3.0;


    // Boolean
    let t = true;
    let f : bool = false ;// explicit type annotation


    // Character
    let c= 'z';
    let z:char = 'Z';

    // Tuples cannot group or shrink in size once declared
    // Can have different data types

    let tup : (i32,f64,u8) = (500,6.4,1);

    let (x,y,z) = tup;
    println!("{x} {y} {z}");

    // Indexing the tuple
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;


    // Array must have all same data types
    // Must also have a fixed length
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let a = [1,2,3,4];
    let b : [i32,5] = [1,2,3,4,5]; // All of i32 and 5 elements
    let a = [3;5] // Creates an array with 5 elements all of value 3

    // Array elements are put on stack memory
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    
}