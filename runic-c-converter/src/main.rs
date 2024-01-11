use std::fs;



fn main() {
    // At some point, make this accept a custom header file for custom stuff 

    let mut args = std::env::args();
    args.next().unwrap(); // throw away file name

    let mut input_filename = args.next().expect("Input file required.");
    let mut output_filename = args.next().expect("Output file required.");

    let mut input_file = String::from_utf8(fs::read(input_filename)
        .expect("File can't be opened."))
        .expect("It brokie :)");

    // let mut output_text = "".to_string();
    // input_file.
    for mut char in input_file.as_mut().chars(){
        match char{

            _ => {
                char = 'a';
            }
        }
    }





    let output_file = fs::write(output_filename, "test");


}
