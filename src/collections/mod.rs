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