
macro_rules! decl_fn {
    ($($name: ident,)*) => {
        $(fn $name(values: &[f64]) -> f64 {
            f64::$name(values[0])
        })*
    };
}

decl_fn!(sin, cos, tan, abs, acos, cosh, acosh, asin, atan, asinh, atanh, cbrt, ceil, floor, log10, log2, ln, round, sqrt, to_radians, to_degrees, );


pub const FUNCTIONS: [(&str, (usize, fn(&[f64]) -> f64)); 22] = [
    ("cos", (1, cos)),
    ("sin", (1, sin)),
    ("tan", (1, tan)),
    ("abs", (1, abs)),
    ("acos", (1, acos)),
    ("cosh", (1, cosh)),
    ("acosh", (1, acos)),
    ("asin", (1, asin)),
    ("atan", (1, atan)),
    ("acosh", (1, acosh)),
    ("asinh", (1, asinh)),
    ("atanh", (1, atanh)),
    ("cbrt", (1, cbrt)),
    ("ceil", (1,ceil)),
    ("floor", (1, floor)),
    ("log10", (1, log10)),
    ("log2", (1, log2)),
    ("ln", (1, ln)),
    ("round", (1, round)),
    ("sqrt", (1, sqrt)),
    ("to_radians", (1, to_radians)),
    ("to_degrees", (1, to_degrees)),
];

pub fn get_function(key: &str) -> Result<(usize, fn(&[f64]) -> f64), ()> {
    for element in FUNCTIONS {
        if element.0 == key {
            return Ok(element.1);
        }
    }
    Err(())
}