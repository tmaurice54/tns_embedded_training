# Exercise 0: Hello world

This exercise is made for beginner who want to learn how to create a project simple in rust.
You will learn how to create a project, create dependencies, and verify that your rust installation work fine.

For the other rust exercises, the projects will be already initiated.

## Question 1: Create your project

To create a project in the current directory you can use the command:

```bash
cargo new myProjectName
```

Try it in this folder.
You can see that rust created a directory with the same name.
This folder contains another folder `src` with a main file `main.rs` which already contains the code to print `Hello, world!`.
And you can also find the file `Cargo.toml`:  

```conf
[package]
name = "myProjectName"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
# Here will be the dependencies
```

This fill will be used for dependencies and other configuration of the project.

## Question 2: Compile and execute your Hello world

Now that you have your project initiated, you must compile it.
You can do it with the following command:

```bash
cargo build
```

This command will create a folder `target` which will contain all the build.
You can find in this folder the executable `myProjectName.exe` in the subdirectory `target\debug\`.
You can execute it and see `Hello, world!` being print in your terminal.

By default, rust compile in debug mode, you can add the option `--release` to the command to compile in release mode.

You can also use the command `run` instead of `build`, which will compile the project and then directly execute it.

## Question 3: Add dependencies

Copy and paste this code in your main file:

```rust
use rand::Rng;

fn main() {
    println!("Guessing the two numbers:");
    
    let mut n=1;  

    while n<=3
    {  
        let secret_number_one = rand::thread_rng().gen_range(1..=20);
        let secret_number_two = rand::thread_rng().gen_range(1..=20);

        println!("The secret number in {n}th iteration is: {secret_number_one}");
        println!("The secret number in {n}th iteration is: {secret_number_two}");

        n+=1;
    }
}
```

Try to compile it.
What can you see?

You can't compile this programm because you don't have the necessary dependency.
To add the `rand` crate (rust package) to your project you can run the command:

```bash
cargo add crateName
```

Use it to add the `rand` crate and compile again to see that it works.

You can also look at the `Cargo.toml` file and see that a dependency has been created:

```conf
[dependencies]
rand = "0.8.5" #The version may be different
```

A dependency can also be added by a repository link.
For example you can replace `rand = "0.8.5"` by `rand = { git = "https://github.com/rust-lang-nursery/rand" }` to get the crate directly from the repo.

## Question 4: Create your own library

You can also create your own library using the option `--lib` on the `cargo new` command.
Try to create a library in the `0_HelloWorld` folder and create a function in it that you will call from your main project.

Don't forget to add the dependency in your `.toml` file using the path to your lib.
