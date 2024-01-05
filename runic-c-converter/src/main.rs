use std::fs;



fn main() {
    let mut args = std::env::args();
    args.next().unwrap(); // throw away file name

    let mut input_filename = args.next().expect("Input file required.");
    let mut output_filename = args.next().expect("Output file required.");
    
    let mut input_file = fs::read


}
