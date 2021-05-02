# mandelbrot visualisation tutorial

in this tutorial we will create an app with gtk 3.0 and rust to display the mandelbrot set.

## 0.0: What is the mandelbrot set?
from https://en.wikipedia.org/wiki/Mandelbrot_set:

> "_The Mandelbrot set (/ˈmændəlbrɒt/) is the set of complex numbers c for which the function f(z) = z² + c does not diverge when iterated from z = 0, i.e., for which the sequence f(0),f(f(0)), etc., remains bounded in absolute value._"

## 0.1: What is Rust?
>rust is a modern low-level programming language that focuses on security and speed. It is free and open-source.

You can find more information about it at https://www.rust-lang.org/.

## 0.2: What is GTK?
>gtk is a free and open-source GUI library that provides many language bindings. It can run on Linux, MS Windows and MacOS.

You can find more information about it at https://gtk.org/.

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
let's go through this line by line!



## let's get graphic!

for the interface, we'll use a glade file. You can create your own interface with [glade](https://glade.gnome.org/).
in the mandelbrot directory, create a folder called **`rsc/`** and save this **`main_window.glade`** file.
```xml
<?xml version="1.0" encoding="UTF-8"?>
<!-- Generated with glade 3.38.2 -->
<interface>
  <requires lib="gtk+" version="3.24"/>
  <object class="GtkWindow" id="main_window">
    <property name="can-focus">False</property>
    <child>
      <object class="GtkBox">
        <property name="visible">True</property>
        <property name="can-focus">False</property>
        <property name="orientation">vertical</property>
        <child>
          <object class="GtkBox">
            <property name="visible">True</property>
            <property name="can-focus">False</property>
            <child>
              <object class="GtkButton" id="inc_btn">
                <property name="label" translatable="yes">+</property>
                <property name="visible">True</property>
                <property name="can-focus">True</property>
                <property name="receives-default">True</property>
              </object>
              <packing>
                <property name="expand">True</property>
                <property name="fill">True</property>
                <property name="position">0</property>
              </packing>
            </child>
            <child>
              <object class="GtkButton" id="dec_btn">
                <property name="label" translatable="yes">-</property>
                <property name="visible">True</property>
                <property name="can-focus">True</property>
                <property name="receives-default">True</property>
              </object>
              <packing>
                <property name="expand">True</property>
                <property name="fill">True</property>
                <property name="position">1</property>
              </packing>
            </child>
          </object>
          <packing>
            <property name="expand">False</property>
            <property name="fill">True</property>
            <property name="position">0</property>
          </packing>
        </child>
        <child>
          <object class="GtkDrawingArea" id="drawing_area">
            <property name="visible">True</property>
            <property name="can-focus">False</property>
          </object>
          <packing>
            <property name="expand">False</property>
            <property name="fill">True</property>
            <property name="position">1</property>
          </packing>
        </child>
      </object>
    </child>
  </object>
</interface>
```
