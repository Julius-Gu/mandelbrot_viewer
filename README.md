## 1.0: creating a project

open the terminal and go to your projects folder. then type **`cargo new mandelbrot`** and **`cd mandelbrot/`** (or **`cd .\mandelbrot\`** on windows).
next, type **`cargo run`**. The text "**`Hello, World!`**" should appear in your terminal.

you can then navigate to the project and open **`main.rs`** in the **`src/`** folder with your favorite text editor ([Kate](https://kate-editor.org/) in my case).

You will see the following code:
```rust
fn main() {
    println!("Hello, world!");
}
```

## 1.1: let's start with the real numbers

Technically, the mandelbrot set is defined on the [complex name](https://en.wikipedia.org/wiki/Complex_plane/). To start, however we'll only regard the real numbers. Let's start with the following:
```rust
fn main() {
    println!("Hello, Mandelbrot!");

    // check some numbers
    // 0 is in the set
    println!("the number {} is in the set: {:?}", 0, check_if_in_set(0.0));

    // 0.1 is in the set
    println!("the number {} is in the set: {:?}", 0.1, check_if_in_set(0.1));

    // -1 is in the set
    println!("the number {} is in the set: {:?}", -1, check_if_in_set(-1.0));

    // 0.5 is not in the set
    println!("the number {} is in the set: {:?}", 0.5, check_if_in_set(0.5));

    // 1 is not in the set
    println!("the number {} is in the set: {:?}", 1, check_if_in_set(1.0));
}
fn check_if_in_set(c: f64) -> bool {
    // z starts at 0
    let mut z: f64 = 0.0;
    // repeat 100 times:
    // note: this number should technically be infinite, but that would not be calculatable.
    for _i in 0..100 {
        z = iteration(z,c);
    }
    // let's see whether this number get's big. If it does, it is not contained in the mandelbrot set
    // note: once again, the number 100 is chosen fairly arbitrarily.
    // note: since the values can diverge to negative infinity and positive infinity we'll need to use the absolute value
    if z.abs() > 100.0 {
        return false;
    } else {
        return true;
    }
}
fn iteration(z: f64, c: f64) -> f64 {
    return z*z + c;
}
```
this will produce the following output:
```
Hello, Mandelbrot!
the number 0 is in the set: true
the number 0.1 is in the set: true
the number -1 is in the set: true
the number 0.5 is in the set: false
the number 1 is in the set: false
```
let's go through this code:
Once again, in order for a number c to be in the mandelbrot set, iterating through z<sub>n+1</sub>=z<sub>n</sub>²+c with z<sub>0</sub> = 0 mustn't diverge (i.e. become + or - infinite).
we'll this need a function to iterate through this:
```rust
fn iteration(z: f64, c: f64) -> f64 {
    return z*z + c;
}
```
notice that this takes two variables of the type **`f64`**. This stands for a 64-bit [floating point number](https://en.wikipedia.org/wiki/Floating-point_arithmetic). Floating point numbers are fascinating, but for short, you can think of them like the computer equivalent to the scientific notation that you learned in scool: 1.234 ⋅ 10<sup>-23</sup>.

Now that we have a function for one iteration, we need to call it multiple times to check whether a given number is in the set. This happens here:
```rust
// from check_if_in_set(c)
    // ...
    for _i in 0..100 {
        z = iteration(z,c);
    }
    // ...
```
where our function gets called a hundret times in a row and the result is saved in z.

> note: To some who already know the ownership model in rust, this might seem a bit weird, after all shouldn't z get moved to the iteration function? This would indeed be the case if our z weren't a simple type (f64 in this case). With simple types, the value gets copied over instead of a pointer to the data being copied. This slight point of confusion is there because it would indeed be slower overall to copy pointers for simple types.

## Hello, imaginaries!
The Mandelbrot set gains it beauty and fractality from being plotted [on the imaginary plane](https://en.wikipedia.org/wiki/Complex_number). It only makes sense for us then, to make our programm accept imaginary numbers.

