mod compiler;
mod utils;

use compiler::Compiler;
use utils::{output_to_file, read_file_to_vec, CLArgHandler};

fn main() -> Result<(), String> {
    // Fetch input file name
    let mut argument_handler: CLArgHandler = CLArgHandler::new();
    match argument_handler.parse_arguments() {
        Err(e) => return Err(e),
        Ok(_) => {}
    }

    let input_filename: String = match argument_handler.input_file {
        Some(filename) => filename,
        None => return Err("[Error] No Input Filename Specified".to_string()),
    };

    // Create string of file contents
    let file_contents: String = match read_file_to_vec(input_filename.clone()) {
        Ok(file_cont) => file_cont,
        Err(e) => {
            return Err(format!(
                "[Error] Unable to open {}: {}",
                input_filename.clone(),
                e
            ))
        }
    };

    let memory_data: String = match Compiler::compile(file_contents) {
        Ok(data) => data
            .into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join("\n"),
        Err(e) => return Err(e),
    };

    let output_filename = match argument_handler.output_file {
        Some(name) => name,
        None => "out_mem_map.txt".to_string(),
    };

    match output_to_file(output_filename.clone(), memory_data) {
        Ok(_) => {}
        Err(e) => {
            return Err(format!(
                "[Error] Writing to file {}: {}",
                output_filename.clone(),
                e
            ))
        }
    }

    // Return valid exit code
    Ok(())
}
