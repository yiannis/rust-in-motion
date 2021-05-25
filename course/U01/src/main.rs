use std::io;

fn main() {
    
    let one;
    let two;
    one = 1;
    two = 2;
    println!("Hello, {}{} world!", one, two);

    if false {
        m4_types();
        m6_flow();
    }
    m8_structs();
}

fn m4_types() {
    // *Bool*
    let cold = true;
    let hot  = false;

    if cold && !hot {
        println!("Winter!")
    }

    // *usize* size of pointer
    let a = [0,1,2,3];
    let idx = 2;

    a[idx];

    // idx is `usize`

    // *char*
    let pi = 'π';
    println!("Unicode char is: {}", pi);

    // *&str*
    let word = "Hello";
    println!("{} world!",word);

    // *tuple*
    let red = ( 255, 0, 0, 0.99 );

    println!("Red: {}, Green: {}, Blue: {}, Alpha: {}", red.0, red.1, red.2, red.3 );
}

fn m6_flow() {
    // for
    let a = 0..10;
    for i in a {
        println!("i == {}",i);
    }

    let x = [0,0,-6];
    match x {
        [0,0,z] => println!("[0,0,{}]",z),
        [1,5,0] => println!("x is `[1,5,0]`"),
        [1,5,6] => println!("x is `[1,5,6]`"),
        [i32::MIN..=-1_i32,i32::MIN..=-1_i32,i32::MIN..=-1_i32] => println!("x is all negative"),
        [_,_,_] => println!("x is a 3 element array"),
    }

    // STDIN
    let mut word = String::new();
    while word.trim() != "rust" {
        println!("Secret word:");
        word = String::new();
        io::stdin().read_line(&mut word).expect("ERROR reading line");
    }

    println!("Correct, `rust` was the secret word!");

}

enum Clock {
    Solar   { hour: u8 },
    Digital { hour: u8, minutes: u8 },
    Analog  { hour: u8, minutes: u8, seconds: u8 },
}
fn tell_time(c: Clock) {
    match c {
        Clock::Solar   { hour: h } => println!("The time is: {}",h),
        Clock::Digital { hour: h, minutes: m } => println!("The time is: {}:{}",h,m),
        Clock::Analog  { hour: h, minutes: m, seconds: s } => println!("The time is: {}:{}:{}",h,m,s),
    }
}

fn m8_structs() {
    // Tuple struct (named tuple)
    struct Color(u8,u8,u8);

    let grass = Color(0,255,0);

    println!("grass has {} for green", grass.1);

    match Color(4,6,4) {
        Color(_,_,1) => println!("Any Color with 3rd element 1"),
        Color(4,6,4) => println!("Color with (4,6,4)"),
   //     grass => println!("Color of grass"), // <-- matches any value: WHY?
                                             // Matches the type rather than the fields???
        _     => println!("Any Color"),      // <-- unreachable pattern
    }

    // Normal struct
    struct ColorNamedFields {
        red: u8,
        green: u8,
        blue: u8,
    }
    let fire = ColorNamedFields {
        red: 255,
        green: 0,
        blue: 0,
    };
    println!("fire has red == {}", fire.red);
    match (ColorNamedFields{ red: 1, green: 2, blue: 3 }) {
        ColorNamedFields{ red: 1, green: 2, blue: 3 } => println!("Found 1,2,3"),
        _ => println!("Random ColorNamedFields"),
    }

    // Enum named fields
    const HOUR:u8   = 10;
    const MINUTE:u8 = 22;
    const SECOND:u8 = 59;
    let garden_clock = Clock::Solar   { hour: HOUR };
    let wrist_watch  = Clock::Digital { hour: HOUR, minutes: MINUTE };
    let bed_alarm    = Clock::Analog  { hour: HOUR, minutes: MINUTE, seconds: SECOND };

    let mut c = garden_clock;
    tell_time( c );
    c = wrist_watch;
    tell_time( c );
    c = bed_alarm;
    tell_time( c );
}

