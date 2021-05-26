fn main() {
    m2_ownership();
    m5_slices();
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
    // NOTE: plural was created as `mut` inseide the function,
    // but was returned as a non-mutable String
}

fn m4_borrowing () {
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

    let a_slice_literal = "Hello";
    let a_string = String::from("Hello");
    let a_string_slice = &a_string[..];

    fn print_0(s: &str) { println!("First letter is: {}", &s[0..1]); }
    fn print_1(s: &str) { println!("Second letter is: {}",&s[1..2]); }

    print_0( a_slice_literal );
    print_1( a_slice_literal );
}
