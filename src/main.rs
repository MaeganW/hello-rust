use ferris_says::say; // from the previous step
                      // use std::io::{stdout, BufWriter};
use std::io::stdout;
use std::io::BufWriter;

// how to use a fn from another file
mod arrays; // declares arrays.rs is a module
use arrays::arrays_mod; // import the function to be used

fn main() {
    let stdout = stdout();
    let out = b"Hello world!";
    let width = 24;

    let array: [i64; 5] = [1, 2, 3, 4, 5];
    let str = "hello";

    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();

    // Types experimentation

    println!("Testing");
    eprintln!("A standard error msg.");
    println!("First element: {}", array[0]);
    println!("Interplate {}", &str);

    let slice = &array[0..3]; // This will select the elements at index 0, 1, and 2. The first number is inclusive and second number is exclusive.
    for b in slice {
        println!("b is {}", b);
    }

    for x in &array[0..4] {
        // To make array iterable either use .iter() or do this
        println!("The element is {}", x);
    }

    // Mutable integer

    let mut counter = 0;
    for x in array.iter() {
        println!("The element at index {} is {}", counter, x);
        counter += 1;
    }

    // Tuple

    let tuple = ("hello", 42, "world", [3, 6, 9]);
    println!("First element is {}", tuple.0);
    println!("Second element is {}", tuple.1);
    println!("Third element is {}", tuple.2);
    let mut counter = 0;
    for x in &tuple.3 {
        println!("Element {} of the fourth element is {}", counter, x);
        counter += 1;
    }

    // Debug

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    fn do_thing() {
        let name = "Peter";
        let age = 27;
        let peter = Person { name, age };

        // Pretty print
        println!("{:#?}", peter);
    }

    do_thing();

    // Mutable Vectors

    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], 1);

    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);

    vec[0] = 7;
    assert_eq!(vec[0], 7);

    vec.extend([1, 2, 3].iter().cloned());

    for x in &vec {
        println!("{}", x);
    }
    assert_eq!(vec, [7, 1, 2, 3]);

    // Using the vec! macro to create new vectors more easily

    let mut vec = vec![1, 2, 3];
    vec.push(4);
    assert_eq!(vec, [1, 2, 3, 4]);

    // example assertion using pi

    use std::f64::consts;

    fn pi() {
        let x = 2.0 * consts::PI;

        let abs_difference = (x.cos() - 1.0).abs();

        assert!(abs_difference < 1e-10);
    };

    pi();

    // Calling a function from a custom module

    arrays_mod();
}
