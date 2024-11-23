/// # verbose
#[macro_export]
macro_rules! verbose {
    ($verbose_flag:expr, $($arg:tt)*) => {
        if $verbose_flag {
        println!($($arg)*);
        }
    };
}

#[macro_export]
macro_rules! dev_debug {
($verbose_flag:expr, $($arg:tt)*) => {
    if $verbose_flag {
    println!("[{}:{}] {}", file!(), line!(), format!($($arg)*));
    }
};
}

/// Show an error to stderr in a similar style to GNU coreutils.
///
/// Takes a [`format!`]-like input and prints it to stderr. The output is
/// prepended with the current utility's name.
#[macro_export]
macro_rules! show_error(
    ($($args:tt)+) => ({
        eprint!("roxide: ");
        eprintln!($($args)+);
    })
);

// Prompt the user with a formatted string and returns `true` if they reply `'y'` or `'Y'`
//
// This macro functions accepts the same syntax as `format!`. The prompt is written to
// `stderr`. A space is also printed at the end for nice spacing between the prompt and
// the user input. Any input starting with `'y'` or `'Y'` is interpreted as `yes`.
//#[macro_export]
//macro_rules! prompt_yes(
//    ($($args:tt)+) => ({
//        use std::io::Write;
//        eprint!("roxide: ");
//        eprint!($($args)+);
//        eprint!(" ");
//    //    uucore::crash_if_err!(1, std::io::stderr().flush());
//        uucore::read_yes()
//    })
//);

/// Macro to prompt the user with a message and collect input.
/// Returns `true` if the input is "yes" or "y" (case-insensitive), otherwise `false`.
///
/// Example usage:
/// ```
/// if prompt_yes!("Do you want to continue? (yes/y):") {
///     println!("Continuing...");
/// } else {
///     println!("Exiting...");
/// }
/// ```
#[macro_export]
macro_rules! prompt_yes {
    ($($arg:tt)*) => {{
        use std::io::{self, Write};
        // Print the prompt and flush stdout
        print!($($arg)*);
        print!(" "); // Add a space after the prompt
        io::stdout().flush().unwrap();

        // Read input from stdin
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Trim and check for "yes" or "y" (case-insensitive)
        matches!(input.trim().to_lowercase().as_str(), "yes" | "y")
    }};
}
