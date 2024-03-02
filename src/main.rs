use csv::{QuoteStyle, ReaderBuilder, StringRecord, WriterBuilder};
use std::env;
use std::error::Error;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn Error>> {
    // Get the current directory
    let current_dir = env::current_dir()?;
    let folder_name = current_dir.file_name().ok_or("NoName")?.to_str().ok_or("NotUTF8")?;
    let output_file_name = format!("{}-combined.csv", folder_name);

    // Output file path is in the current directory with the name <folder-name>-combined.csv
    let output_file_path = current_dir.join(output_file_name);

    // Vector to hold all rows from all files
    let mut all_rows: Vec<Vec<String>> = Vec::new();
    let mut headers: Vec<String> = Vec::new();

    // Traverse the current folder and process each CSV file
    for entry in WalkDir::new(&current_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().unwrap_or_default() == "csv" && path.is_file() {
            
            let mut rdr = ReaderBuilder::new().from_path(path)?;

            // If headers are empty, read them from the first file
            if headers.is_empty() {
                headers = rdr.headers()?.iter().map(String::from).collect();
            }

            let mut prev_record = StringRecord::default();
            // Read rows from the CSV file
            for result in rdr.records() {
                match result {
                    Ok(record) => {
                        prev_record = record.clone();
                        let row: Vec<String> = record.iter().map(String::from).collect();
                        all_rows.push(row);
                    },
                    Err(e) => {
                        println!("Previous record: {:#?}", prev_record);
                        println!("Error in: {:#?}", path.file_name());
                    },
                }
                
            }

            println!("Finished with file: {:#?}", path.file_name());
        }
    }

    // Write all rows to a single output CSV file, ensuring all fields are quoted
    let mut wtr = WriterBuilder::new()
        .quote_style(QuoteStyle::Always) // Ensure every field is quoted
        .from_path(&output_file_path)?;

    wtr.write_record(&headers)?; // Write headers

    for row in all_rows {
        wtr.write_record(&row)?;
    }

    wtr.flush()?;
    println!("Combined CSV file created at: {:?}", output_file_path);
    Ok(())
}
