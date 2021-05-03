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
    let mut z: f64 = 0.0;
    // repeat 100 times:
    // note: this number should technically be infinite, but that would not be calculatable.
    for _i in 0..100 {
        z = iteration(z,c);
    }
    // ...
```
where our function gets called a hundred times in a row and the result is saved in z.

To make this more understandable, let's look at an example:
We want to know if the number 0.5 is in the set:

```
0.0) z is set to 0
1.0) call iteration with (z=0 and c=0,5)
-> in iteration: return 0*0+0.5=0.5
1.1) store this value (0.5) in z
2.0) call iteration with (z=0.5 and c=0.5)
-> 0.5*0.5+0.5=0.75
2.1) store this value (0.75) in z
3.0) call iteration with (z=0.75 and c=0,5)
-> 0.75*0.75+0.5=1.0625
3.1) store this value (1.0625) in z
4.0) call iteration with (z=1.0625 and c=0,5)
-> 1.0625*1.0625+0.5 = 1.62890625
```
and so on.

on the 10th iteration of this, we'll be at `20 773 872 763 941 816`. Needless to say, this number gets **very** big.


## Hello, imaginaries!
The Mandelbrot set gains it beauty and fractality from being plotted [on the imaginary plane](https://en.wikipedia.org/wiki/Complex_number). It only makes sense for us then, to make our programm accept imaginary numbers.

