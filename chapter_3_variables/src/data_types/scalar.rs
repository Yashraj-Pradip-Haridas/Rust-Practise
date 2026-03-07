fn scalar() {
    // scalar datatypes
    // Integers - I and U
    let a = 98_222; // Decimal
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Byte (u8 only)
    // let f:u8 =256;
    // in debug mode rust will panic and in release mode it will perform 2` compliment wrapping

    // Floating point numbers
    let f = 3.2;
    let g: f32 = 5.3;

    // Booleans
    let t = true;
    let f = false;

    // Character
    let c = 'a';
}
