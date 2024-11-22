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

/// Prompt the user with a formatted string and returns `true` if they reply `'y'` or `'Y'`
///
/// This macro functions accepts the same syntax as `format!`. The prompt is written to
/// `stderr`. A space is also printed at the end for nice spacing between the prompt and
/// the user input. Any input starting with `'y'` or `'Y'` is interpreted as `yes`.
#[macro_export]
macro_rules! prompt_yes(
    ($($args:tt)+) => ({
        use std::io::Write;
        eprint!("roxide: ");
        eprint!($($args)+);
        eprint!(" ");
    //    uucore::crash_if_err!(1, std::io::stderr().flush());
//        uucore::read_yes()
    })
);
