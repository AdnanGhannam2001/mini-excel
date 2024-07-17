use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;

use parsing::Parser;

mod scanning;
mod parsing;

fn usage(program_name: String) -> String
{
    format!("{} <input>", program_name)
}

fn main() -> io::Result<()>
{
    let mut args = env::args().collect::<Vec<String>>();

    let program = args.remove(0);

    if args.len() != 1
    {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, usage(program)));
    }

    // Reading file
    let input_filename = args.remove(0);

    let input_file = fs::File::open(&input_filename);

    if let Err(_value) = input_file
    {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("File `{}` not found", &input_filename)));
    }

    let mut input = String::new();

    input_file.unwrap().read_to_string(&mut input).unwrap();

    // Generating output file
    let output = Parser::parse_file(input);

    let output_filename = "output";
    let mut output_file = fs::File::create(output_filename).unwrap();
    output_file.write_all(output.as_bytes()).unwrap();

    println!("Output saved to: {}", output_filename);

    Ok(())
}
