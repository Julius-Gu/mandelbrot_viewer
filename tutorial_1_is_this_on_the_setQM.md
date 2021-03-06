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

    // 0.3 is not in the set
    println!("the number {} is in the set: {:?}", 0.3, check_if_in_set(0.3));

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
    println!("z: {}",z);
    // let's see whether this number get's big. If it does, it is not contained in the mandelbrot set
    // note: since the values can diverge to negative infinity and positive infinity we'll need to use the absolute value
    if z.abs() > std::f64::MAX {
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
z: 0
the number 0 is in the set: true
z: 0.11270166537925831
the number 0.1 is in the set: true
z: 0
the number -1 is in the set: true
z: inf
the number 0.3 is in the set: false
z: inf
the number 1 is in the set: false

```
let's go through this code:
Once again, in order for a number c to be in the mandelbrot set, iterating through z<sub>n+1</sub>=z<sub>n</sub>??+c with z<sub>0</sub> = 0 mustn't diverge (i.e. become + or - infinite).
we'll this need a function to iterate through this:
```rust
fn iteration(z: f64, c: f64) -> f64 {
    return z*z + c;
}
```
notice that this takes two variables of the type **`f64`**. This stands for a 64-bit [floating point number](https://en.wikipedia.org/wiki/Floating-point_arithmetic). Floating point numbers are fascinating, but for short, you can think of them like the computer equivalent to the scientific notation that you learned in scool: 1.234 ??? 10<sup>-23</sup>.

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

On the 10th iteration of this, we'll be at `20 773 872 763 941 816`. Needless to say, this number gets **very** big.
It may already be obvious that the numbers grow _exponentially_ fast.

Before we head of, lets have a look at this snippet:
```rust
// from check_if_in_set
    if z.abs() > std::f64::MAX {
        return false;
    } else {
        return true;
    }
```
How come that an f64 can be larger than the maximum value?

**in Rust, if a value becomes larger than the maximum value, it becomes inifinite.**


## 1.2: Hello, imaginaries!
The Mandelbrot set gains it beauty and fractality from being plotted [on the imaginary plane](https://en.wikipedia.org/wiki/Complex_number). It only makes sense for us then, to make our programm accept imaginary numbers.

Before we code it, how may we do this?

we currently have something like
```
let z = 0
repeat 100 times:
    z = z??+c
```

now, however, z and c are allowed to be an imaginary number of the form _a+bi_ where _a,b ??? ???_ and _i = ???(-1)_.

_z = z?? + c_

thus becomes:

_a<sub>z</sub> + b<sub>z</sub>i = (a<sub>z</sub>+b<sub>z</sub>i)?? + a<sub>c</sub> + b<sub>c</sub>i_

using the binomial formulas, we get:

_(a<sub>z</sub>+b<sub>z</sub>i)?? + a<sub>c</sub> + b<sub>c</sub>i = a<sub>z</sub>??+2a<sub>z</sub>b<sub>z</sub>i+b<sub>z</sub>??i??+ a<sub>c</sub> + b<sub>c</sub>i_

since _i=???(-1)_ ??? _i??=-1_. Our formula becomes then:

_a<sub>z</sub> + b<sub>z</sub>i = a<sub>z</sub>??+2a<sub>z</sub>b<sub>z</sub>i-b<sub>z</sub>??+ a<sub>c</sub> + b<sub>c</sub>i_

this can be split up into two parts: one for every summand multiplied with _i_ on one for every one without.

_a<sub>z</sub> = a<sub>z</sub>??-b<sub>z</sub>??+a<sub>c</sub>_

_b<sub>z</sub> i = 2a<sub>z</sub>b<sub>z</sub>i+b<sub>c</sub>i_  | /i

_b<sub>z</sub> = 2a<sub>z</sub>b<sub>z</sub>+b<sub>c</sub>_

with this, we can create a part of our new algorithm:
```
let az = 0
let bz = 0

repeat 100 times:
    // note: since we need az for new_bz, we need a temporary variable
    let new_az = az??-bz?? + ac
    bz = 2*az*bz + bc
    az = new_az
```

but how can we know whether this diverges to infinity?

We will have a look at its magnitude, i.e. ???(a<sub>z</sub>??+b<sub>z</sub>??.
Since ???inf = inf, the sqrt isn't necessary.

the full code is thus:

```rust
fn main() {
    println!("Hello, imaginaries!");

    // check some numbers
    // 0 is in the set
    println!("the number {} is in the set: {:?}", 0, check_if_in_set(0.0,0.0));

    // 0.1 is in the set
    println!("the number {} is in the set: {:?}", "1-i", check_if_in_set(1.0,-1.0));

    // -1 is in the set
    println!("the number {} is in the set: {:?}", "-0.3+0.1i", check_if_in_set(-0.3,0.1));

    // 0.3 is not in the set
    println!("the number {} is in the set: {:?}", "5+6i", check_if_in_set(5.0,6.0));

    // 1 is not in the set
    println!("the number {} is in the set: {:?}", "0.3+0.6i", check_if_in_set(0.3,0.6));
}
// input: number of the form a+bi with real numbers a and b
fn check_if_in_set(ac: f64, bc: f64) -> bool {
    // z starts at 0
    let mut az: f64 = 0.0;
    let mut bz: f64 = 0.0;

    // repeat 100 times:
    for _i in 0..100 {
        // break when one of these becomes infinite
        if az*az>=std::f64::MAX || bz*bz>=std::f64::MAX {
            break;
        }
        let new_az = az*az - bz*bz + ac;
        bz = 2.0*az*bz + bc;
        az = new_az;
    }
    // check if its magnitude is infinite
    if (az*az+bz*bz) > std::f64::MAX {
        return false;
    } else {
        return true;
    }
}
```
(result)
```
Hello, imaginaries!
the number 0 is in the set: true
the number 1-i is in the set: false
the number -0.3+0.1i is in the set: true
the number 5+6i is in the set: false
the number 0.3+0.6i is in the set: false
```

A few things have been added here.

First, this now checks whether the number becomes infinite.
```rust
if az*az>=std::f64::MAX || bz*bz>=std::f64::MAX {
    break;
}
```
this is done
- in order to make our code faster
- to avoid substracting two possibly infinite numbers `let new_az = az*az - bz*bz + ac;` which would break maths and destroy our universe

The full code is available [here](https://github.com/Julius-Gu/mandelbrot_viewer/tree/main/programs/tutorial_1_set_checker)

This is still not the most efficient code and we will refine it later in this tutorial, however you are probably itching to start writing some GTK. We will do this in the next chapter.
