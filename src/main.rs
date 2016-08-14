//! CSV-matrix mirror
//!
//! Make numbers in a CSV matrix the same both horizontally and vertically.
#![warn(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_extern_crates, unused_import_braces, unused_qualifications, unused_results)]

extern crate quick_csv;

use std::io;

/// Main entry point for the CSV-matrix mirror utility.
fn main() {
    // Open standard input and read its contents into the CSV parser.
    let stdin = io::stdin();
    let stdin_buffer = stdin.lock();
    let input_csv = quick_csv::Csv::from_reader(stdin_buffer);

    // Loop through each row in the CSV file.
    for (rownum, row) in input_csv.into_iter().enumerate() {
        // Parse rows that are okay.
        if let Ok(_) = row {
            println!("Row {} is valid.", rownum);
        }
        // If we run into a row that is not okay, break!
        else {
            println!("Error reading row {}!", rownum);
            break;
        }
    }
}
