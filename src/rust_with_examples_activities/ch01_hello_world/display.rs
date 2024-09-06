#![allow(dead_code)]

/*
Activity
After checking the output of the above example, use the Point2D struct as a guide to add a Complex struct to the example. When printed in the same way, the output should be:
Display: 3.3 + 7.2i
Debug: Complex { real: 3.3, imag: 7.2 }
*/

use std::fmt;

#[allow(dead_code)]
#[derive(Debug)]
struct Complex {
    real: f32,
    imag: f32,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{real} + {imag}i", real = self.real, imag = self.imag) // no semicolon returns fmt result?
    }
}

pub fn run() {
    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}
