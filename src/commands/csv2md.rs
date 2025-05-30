use anyhow::Result;
use csv::ReaderBuilder;
use std::io;

pub fn convert(with_header: bool) -> Result<()> {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin());

    let mut records: Vec<Vec<String>> = Vec::new();

    for result in reader.records() {
        let record = result?;
        records.push(record.iter().map(|s| s.to_string()).collect());
    }

    if records.is_empty() {
        return Ok(());
    }

    if with_header && !records.is_empty() {
        // Print header row
        let header = &records[0];
        print!("|");
        for cell in header {
            print!(" {} |", cell);
        }
        println!();

        // Print separator row
        print!("|");
        for _ in header {
            print!(" --- |");
        }
        println!();

        // Print data rows (skip first row since it's header)
        for record in &records[1..] {
            print!("|");
            for cell in record {
                print!(" {} |", cell);
            }
            println!();
        }
    } else {
        // Print all rows as data (no header)
        for record in &records {
            print!("|");
            for cell in record {
                print!(" {} |", cell);
            }
            println!();
        }
    }

    Ok(())
}
