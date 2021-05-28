fn main() {
    m2_ownership();
    m4_borrowing();
    m5_slices();
    m7_patterns();
}

fn m2_ownership () {
    fn pluralize(s: &String) -> String {
        let mut plural = s.clone();
        plural.push('s');

        return plural;
    }

    let singular = String::from("book");
    let plural   = pluralize( &singular );

    println!("One {}, or two {}? That is the question...", singular, plural);

    // plural.push('s'); <-- fails
    // NOTE: plural was created as `mut` inside the function,
    // but was returned as a non-mutable String
}

fn m4_borrowing () {
    struct Star {
        startype: String,
        radius: f64,
    }

    impl Star {
        fn new(t: &str, r: f64 ) -> Self {
            Star {
                startype : String::from( t ),
                radius   : r,
            }
        }
        fn explode(&mut self) {
            self.startype = String::from("White Dwarf");
            self.radius = 1.0;
        }
        fn to_black_hole(self) {
        }
    }

    let mut albireo = Star::new("White giand", 100.0);
    let mut sun     = Star::new("Yellow normal", 10.0);

    albireo.explode();
    sun.explode();

    println!("Sun is now just {} wide", 2.0*sun.radius);

    albireo.to_black_hole();

    // The below fails, as after the function above the Star has disappeared from the known universe
    // println!("Albireo is now a black hole with radius: {}",albireo.radius);
}


fn m5_slices() {
    // Slice:
    // A Rust language data type that *borrows* data
    let a = [1,2,3,4];
    let v = vec!(1,2,3,4);
    let sa = &a[..];
    let sv = &v[..];

    fn array_ref_0(x: &[i32; 4]) { assert_eq!( x[0], 1 ); }
    fn vec_ref_0(x: &Vec<i32>)   { assert_eq!( x[0], 1 ); }
    fn slice_0(x: &[i32])        { assert_eq!( x[0], 1 ); }

    array_ref_0( &a );
    vec_ref_0( &v );
    slice_0( sa );
    slice_0( &a );
    slice_0( &v );
    slice_0( &a[0..2] );
    slice_0( &a[..2] );
    slice_0( &v[0..2] );
    slice_0( &v[..2] );
    slice_0( &sv[0..2] );
    slice_0( &sv[..2] );
    slice_0( &sa[..3] );

    let mut vm = vec![1,2,3,4];
    let s_vm   = &vm[..];
    println!("The last element of the slice is: {}", s_vm[3]);
    
    vm[3] = 10;
    vm.pop();
    //println!("The last element of the slice is: {}", s_vm[3]); // Fails at compile time!

    let v_0 = v.first().unwrap();
    println!("The first element of the vector is: {}", v_0 );

    let a_slice_literal = "Hello";
    let a_string = String::from("Hello");
    let a_string_slice = &a_string[..];

    fn print_0(s: &str) { println!("First letter is: {}", &s[0..1]); }
    fn print_1(s: &str) { println!("Second letter is: {}",&s[1..2]); }

    print_0( a_slice_literal );
    print_1( a_slice_literal );
}

fn m7_patterns() {
    use std::collections::HashMap;

    let text = "hello hello hello world";

    let mut freqs = HashMap::new();
    for word in text.split_whitespace() {
        match freqs.get_mut( word ) {
            Some(value) => { *value += 1; },
            None => { freqs.insert( word, 1 ); },
        }
    }

    println!("Found `{}` times `hello`", freqs.get("hello").unwrap());
}
