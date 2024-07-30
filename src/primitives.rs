fn main() {
    let logical: bool = true;

    let a_float: f64 = 1.0;

    let default_float = 3.0; // f64

    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;

    let mut mutable = 12; // Mutable i32
    mutable = 21;

    mutable = true; // Error: expected integer, found bool
    println!("{}", mutable);

    //let mutable = true; // Variable can be overwritten with shadowing
}