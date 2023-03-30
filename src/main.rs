use std::process;
use std::{fs, io::Read};

use brainfuck_compiler as bfc;
use bfc::compiler::Compiler;
use bfc::interpreter::Interpreter;

use clap::{Arg, ArgAction, Command};

fn main() {
    const DEFAULT_OUTPUT: &'static str = "output";

    let cmd = Command::new("Brainfuck compiler")
        .about("An actual brainfuck compiler")
        .version("1.0")
        .bin_name("bfc")
        .arg_required_else_help(true)
        .author("wizard")
        .subcommand_required(true)
        .arg(
            Arg::new("cell count")
                .required(false)
                .long("cellcount")
                .action(ArgAction::Set)
                .default_value("2000"),
        )
        .subcommand(
            Command::new("compile")
                .short_flag('c')
                .long_flag("compile")
                .about(format!("Compile brainfuck code, default output file name: `{0}` and `{0}.asm` (will be deleted)", DEFAULT_OUTPUT))
                .arg_required_else_help(true)
                .arg(
                    Arg::new("output to assembly")
                        .action(ArgAction::SetTrue)
                        .required(false)
                        .long("asm")
                        .short('s')
                )
                .arg(
                    Arg::new("file path")
                        .action(ArgAction::Set)
                        .required(true)
                        .long("path")
                        .short('p'),
                )
                .arg(
                    Arg::new("compile and run")
                        .action(ArgAction::SetTrue)
                        .required(false)
                        .long("run")
                        .short('r'),
                )
                .arg(
                    Arg::new("output file name")
                        .action(ArgAction::Set)
                        .required(false)
                        .long("output")
                        .short('o'),
                ),
        )
        .subcommand(
            Command::new("interpret")
                .short_flag('i')
                .long_flag("interpret")
                .about("Interpret brainfuck code")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("file path")
                        .required(true)
                        .action(ArgAction::Set)
                        .long("path")
                        .short('p'),
                ),
        );

    let matches = cmd.get_matches();
    let cellcount = matches
        .get_one::<String>("cell count")
        .unwrap()
        .parse::<usize>()
        .expect("Unexpected cell count value");

    match matches.subcommand() {
        Some(("compile", matches)) => {
            let file_path = matches.get_one::<String>("file path").unwrap();
            let run = matches.get_one::<bool>("compile and run").unwrap();
            let output_asm = matches
                .get_one::<bool>("output to assembly")
                .unwrap_or(&false);
            let output_file_path = match matches.get_one::<String>("output file name") {
                Some(f) => f.clone(),
                None => DEFAULT_OUTPUT.to_string(),
            };
            let output_asm_file = format!("{}.asm", output_file_path);

            let mut file_obj = fs::File::open(file_path).expect("File does not exist");
            let mut bf_code = String::new();

            file_obj
                .read_to_string(&mut bf_code)
                .expect("Failed to read from file");

            let mut compiler = Compiler::with_cells(bf_code.as_str(), cellcount);

            compiler
                .save_assembly_output(&output_asm_file)
                .expect("Failed to save the output file `output.asm`");

            process::Command::new("nasm")
                .arg("-felf64")
                .arg(&output_asm_file)
                .spawn()
                .unwrap()
                .wait()
                .unwrap();

            process::Command::new("ld")
                .arg("-o")
                .arg(&output_file_path)
                .arg(format!("{}.o", output_file_path))
                .spawn()
                .unwrap()
                .wait()
                .unwrap();

            fs::remove_file(format!("{output_file_path}.o")).expect("Failed to delete object file");
            (!output_asm).then(|| {
                fs::remove_file(output_asm_file).expect("Failed to delete assembly output file")
            });

            run.then(|| {
                process::Command::new(fs::canonicalize(output_file_path.as_str()).unwrap())
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();
            });
        }

        Some(("interpret", matches)) => {
            let file_path = matches.get_one::<String>("file path").unwrap();

            let mut file_obj = fs::File::open(file_path).expect("File does not exist");
            let mut bf_code = String::new();

            file_obj
                .read_to_string(&mut bf_code)
                .expect("Failed to read from file");

            let mut interpreter = Interpreter::with_cells(bf_code.as_str(), cellcount);

            interpreter.interpret();
        },

        _ => unreachable!(),
    };
}
