## A simple "VM" based calculator thing
### Features:
- Number systems: Decimal, Binary, Octal, Hexadecimal
    - Note that output is only in the decimal number system
- Strings with basic escape sequences parsed
    - Strings can be conactenated with the `+` operator
- Basic math operations: Add (`+`), Subtract (`-`), Divide (`/`), Multiply (`*`), Exponent (`**`), Modulo (`%`)
- Binary operations: AND (`&`), OR (`|`), XOR (`^`), Left Shift (`<<`), Right Shift (`>>`)
    - Note that these operations will truncate the floating point of both sides before proceeding
- Variables: Null values or floating point values (64 bit precision)
- Assignment + Operations on variables, ie. Add + Assign (`+=`), Subtract + Assign (`-=`), so on and so forth. This applies to all operators previously discussed
- Null values cannot have any operation performed on them
- Basic function support: each function allows only a single expression to compute
    - Also note that you cannot override built in functions, but you can do so for your own functions. 
- Deletion of variables and functions
    - You are not allowed to delete built in functions. Why would you want to? 
- Command line arguments:
    - `-r` | `--run-file` reads a file and executes it
    - `-b` | `--run-binary` runs the binary file provided by the next argument
    - `-w` | `--write-binary` reads a file provided by the next argument and generates the bytecode to stores it as binary file. This file is in the same location with the extension `.bin` if another argument is not provided, otherwise, it stores it to the path provided by that other argument.
    - `-p` | `--show-parsed` Shows the parsed output as a formatted expression, which looks similar to the code provided to it
    - `-i` | `--show-instructions` Shows the instruction set that is produced from the parsed AST tree, which is what the VM executes
    - `-t` | `--text` Runs the text provided after this flag
    - `-l` | `--repl` Runs the REPL

Here is a bit of an example of the syntax and the working:
Try to run it
```cs
// Currently using C# syntax highlighting. Which other language offers syntax highlighting that better suits this?

// : at the end to display output
// ; at the end to compute the result but not display it
// use the `print(...)` function to display outputs as well

print(1, 2, 3, 4); // 1 2 3 4

// Basic math
5 + 7:   // 12
5 * 7:   // 35
5 / 7:   // 0.7142...
5 ** 7:  // 78125
5 - 7:   // -2
5 ** -7: // 0.0000128
5 % 7:   // 5

// of course, any type of well known number system is supported:
10:    // 10
0b111: // 7
0o777: // 511
0xfff: // 4095
// The outputs are all in the decimal system and cannot be changed.

// Bitwise operations
// Note that any bitwise operation will truncate the fraction of both sides before proceeding since floating point bitwise operations don't make sense

1 & 2:  // 0
1 | 2:  // 3
1 >> 2: // 0
1 << 2: // 4
1 ^ 2:  // 3

// Declare variables
let variable_name = 1.5;
variable_name:

// Other operations that also assign to the variable:
variable_name *= 1.5;
variable_name /= 1.5;
variable_name **= 1.5;
variable_name += 1.5;
variable_name -= 1.5;

variable_name: // 1.837...

// Bitwise operations as well:
// The same condition as above applies to this as well

variable_name &= 2;
variable_name |= 2;
variable_name ^= 2;
variable_name >>= 2;
variable_name <<= 2;

variable_name: // 0

// Delete variables if you want to
// Delete functions the same way as well
// You cannot delete built in functions

delete variable_name;

// variable_name: // Will throw an error. Uncomment to try

// Strings
"Hello": // Hello
// String concatenation
"Hello" + " " + "World": // Hello World
// Printing achieves the same effect, although it directly prints to the console rather than allowing the result to be passed on as an output
print("Hello", "World"); // Hello World

// Functions

// Functions are values. They can be passed around

let no_args _ = sin(to_radians(90)); // Just a `_` implies no arguments
no_args(): // 1

let args _ a = a / _; // But `_` can be used as an argument when more than one argument is expected
args(5, 2): // 0.4
// The number of arguments are fixed and are not dynamic

// You can access variables declared after the function
let access_outside _ = c + d;

let c = 5;
let d = 10;

// This does work
access_outside():

// Just to be clear, recursion is *not* allowed. It does not make sense with a single expression function anyway.
// let sum a b = sum(a, b + 1); // Produces an error

// An interesting workaround I found for recursion (which I fixed now)
// let a _ = 1; // Something arbitrary
// let b _ = a();
// Now, redefine a such that recursion is produced
// let a _ = b(); // Doing this will produce an error

// Now that functions can be passed around as values, you can still break this by doing
// let call fn = fn(fn);
// call(call):

// Now, you may also check the type of a specific value
let say = "bingo";
typeof say: // "{String}"
// Try it with other types

// Beyond this, an implementation of haskell-like partial functions
let a f u n c = f + u + n + c;
// This will make b the same function as a
let b = a;
// b is once again just b. It has been partially called with 0 arguments and is waiting for more arguments to come by
let b = b();
// Now you can execute it as so:
b(1)(2)(3)(4): // Equivalent to b(1, 2, 3, 4)
// Or so:
b(1)(2, 3, 4):
// Or even:
b(1, 2)(3, 4):
// You get the point
// You can do this as well, which is expected.
// Now c is a function that's waiting for 2 more arguments to be passed to it.
let c = b(1, 2);
// And once passed, c is evaluvated
c(3, 4,):
// Yes, you may have an extra comma at the end

// Now, we have reached arrays

// You can declare them:
let arr = [1, 2, 3, 4];
// You can have an array inside an array. I don't care
let arr = [arr, arr, arr]; // [[1, 2, 3, 4], [1, 2, 3, 4], [1, 2, 3, 4]]
// Indexing is as other languages
// Any index will be truncated to a unsigned integer. Negative values to zero
arr[0]: // <Array> [1, 2, 3, 4]
// Many dimensional indexing as well
arr[0][0]: // 1
// Modifying the values for an array is pretty much the same as other languages as well.
arr[0] = "Modified";
arr[1][0] /= arr[2][1];
arr: // <Array> [Modified, <Array> [0.5, 2, 3, 4], <Array> [1, 2, 3, 4]]
// That's pretty much it for arrays.

// Values can be passed to other values as you change them
// Like so,
let a = let b = 5;
a: b: // 5, 5
b = a *= b;
a: b: // 25, 25
b = a = 1;
a: b: // 1, 1
```
Pretty simple, I'd say

Some things are still buggy, and some syntax does not allow you to do what you'd expect, but this is pretty much it.

### Dependencies:
- serde, bincode: Used to convert instructions to and from bytecode
- clap: Used to parse arguments