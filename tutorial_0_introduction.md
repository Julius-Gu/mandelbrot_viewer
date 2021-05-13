# mandelbrot visualization tutorial

in this tutorial we will create an app with gtk 3.0 and rust to display the mandelbrot set.

## 0.0: What is the mandelbrot set?
from https://en.wikipedia.org/wiki/Mandelbrot_set:

> "_The Mandelbrot set (/ˈmændəlbrɒt/) is the set of complex numbers c for which the function f<sub>c</sub>(z)=z² + c does not diverge when iterated from z = 0, i.e., for which the sequence f(0),f(f(0)), etc., remains bounded in absolute value._"

![mandelbrot_set](https://github.com/Julius-Gu/mandelbrot_viewer/blob/main/images/0-mandelbrotset.png)

The Mandelbrot set is known for its surprising beauty comming from such a simple formula. It is also a popular example for self-similarity.

## 0.1: What is Rust?
![logo](https://github.com/Julius-Gu/mandelbrot_viewer/blob/main/images/0-logo.png)

>rust is a modern low-level programming language that focuses on security and speed. It is free and open-source.

You can find more information about it at https://www.rust-lang.org/.

## 0.2: What is GTK?
>gtk is a free and open-source GUI library that provides many language bindings. It can run on Linux, MS Windows and MacOS.

You can find more information about it at https://gtk.org/.

