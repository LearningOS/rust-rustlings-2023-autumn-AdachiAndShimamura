// try_from_into.rs
//
// TryFrom is a simple and safe type conversion that may fail in a controlled
// way under some circumstances. Basically, this is the same as From. The main
// difference is that this should return a Result type instead of the target
// type itself. You can read more about it at
// https://doc.rust-lang.org/std/convert/trait.TryFrom.html
//
// Execute `rustlings hint try_from_into` or use the `hint` watch subcommand for
// a hint.


// Your task is to complete this implementation and return an Ok result of inner
// type Color. You need to create an implementation for a tuple of three
// integers, an array of three integers, and a slice of integers.
//
// Note that the implementation for tuple and array will be checked at compile
// time, but the slice implementation needs to check the slice length! Also note
// that correct RGB color values must be integers in the 0..=255 range.

// Tuple implementation
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, PartialEq)]
enum IntoColorError {
    BadLen,
    IntConversion,
}

impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;

    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        let red = u8::try_from(tuple.0);
        let green = u8::try_from(tuple.1);
        let blue = u8::try_from(tuple.2);

        if red.is_err() || green.is_err() || blue.is_err() {
            return Err(IntoColorError::IntConversion);
        }

        Ok(Color {
            red: red.unwrap(),
            green: green.unwrap(),
            blue: blue.unwrap(),
        })
    }
}

impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;

    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        let red = u8::try_from(arr[0]);
        let green = u8::try_from(arr[1]);
        let blue = u8::try_from(arr[2]);

        if red.is_err() || green.is_err() || blue.is_err() {
            return Err(IntoColorError::IntConversion);
        }

        Ok(Color {
            red: red.unwrap(),
            green: green.unwrap(),
            blue: blue.unwrap(),
        })
    }
}

impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;

    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        }

        let red = u8::try_from(slice[0]);
        let green = u8::try_from(slice[1]);
        let blue = u8::try_from(slice[2]);

        if red.is_err() || green.is_err() || blue.is_err() {
            return Err(IntoColorError::IntConversion);
        }

        Ok(Color {
            red: red.unwrap(),
            green: green.unwrap(),
            blue: blue.unwrap(),
        })
    }
}

fn main() {
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);

    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}

