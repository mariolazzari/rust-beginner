const WORLD:i32 = 5;

fn main() {
    // mutable
    let mut hello =1;
    println!("Hello, world {}", hello);

    hello = 2;
    println!("Hello, world {}", hello);

    // redefine
    let hello = "5";
    println!("Hello {} {}", hello, WORLD);

    // no automatic casting
    let var64:i64 = 5;
    let var32:i32 = var64 as i32;
    println!("No casting: {} {}", var64, var32);

    // real
    let float:f32 = 1.11;
    let double:f64 = 1.1234;
    println!("Real: {} {}", float, double);

    // no conversions on operation too
    let results = float + double as f32;
    println!("No conversion on sum: {}", results);

    // boolean
    let bool_true:bool = true;
    let bool_false:bool = false;
    println!("Boolean: {} {}", bool_true, bool_false);

    // char
    let char_a:char = 'a';
    let char_b:char = 'b';
    println!("Char: {} {}", char_a, char_b);

    // tuples
    let tuple_a = (1, "hello", true);
    println!("Tuple: {} {} {}", tuple_a.0, tuple_a.1, tuple_a.2);
    // debug print
    println!("Tuple: {:?}", tuple_a);
    // tuple destructuring
    let (a, b, c) = tuple_a;
    println!("Tuple destructuring: {} {} {}", a, b, c);

    // arrays
    let array_a = [1, 2, 3, 4, 5];
    println!("Array: {} {} {} {} {}", array_a[0], array_a[1], array_a[2], array_a[3], array_a[4]);
    // array type
    let array_b: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array type: {} {} {} {} {}", array_b[0], array_b[1], array_b[2], array_b[3], array_b[4]);
    let first = array_b[0];
    println!("Array first: {}", first);


}
