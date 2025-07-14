use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyfunction;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

#[gen_stub_pyfunction]
#[pyfunction]
fn guess_the_number() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

#[gen_stub_pyfunction]
#[pyfunction]
/// simply add to integers
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(guess_the_number, m)?)?;
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}

// Don't forget to define this to generate the stub information
pyo3_stub_gen::define_stub_info_gatherer!(stub_info);
