use ragout::{init, run};

fn main() {
    // enter raw mode and initialize necessary variables
    // the string literal argument will be the value of the prompt
    let (mut sol, mut i, mut h, mut ui) = init("");

    'main: loop {
        // catch and handle user actions

        // bind user input value to input var
        let input = run(&mut i, &mut h, &mut sol, &mut ui);

        // handle input then reset ui var to empty string
        if !input.is_empty() {
            // do some stuff with the user input
        }
    }
}
