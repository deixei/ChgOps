pub mod dx;


#[derive(Debug)]
pub enum Verbose {
    Empty,
    V,
    VV,
    VVV,
}

pub fn print_verbose(verbose: Verbose) {
    match verbose {
        Verbose::Empty => println!("No verbosity"),
        Verbose::V => println!("Some verbosity"),
        Verbose::VV => println!("More verbosity"),
        Verbose::VVV => println!("Maximum verbosity"),
    }
}

#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => ({
        use colored::*;
        eprintln!("{} {}", "ERROR:".red(), format!($($arg)*));
    });
}

#[macro_export]
macro_rules! print_warning {
    ($($arg:tt)*) => ({
        use colored::*;
        eprintln!("{} {}", "WARNING:".yellow(), format!($($arg)*));
    });
}

#[macro_export]
macro_rules! print_info {
    ($($arg:tt)*) => ({
        use colored::*;
        println!("{} {}", "INFO:".blue(), format!($($arg)*));
    });
}

#[macro_export]
macro_rules! print_success {
    ($($arg:tt)*) => ({
        use colored::*;
        println!("{} {}", "SUCCESS:".green(), format!($($arg)*));
    });
}


#[macro_export]
macro_rules! print_banner_yellow {
    ($($arg:tt)*) => ({
        use colored::*;
        println!("{}", format!($($arg)*).yellow());
    });
}