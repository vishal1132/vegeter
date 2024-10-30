use clap::Parser;
use csv::ReaderBuilder;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(name = "vegeter", version = "1.0", about = "Generate Vegeta targets from CSV")]
struct Opt {
    #[arg(short, long)]
    url: String,

    #[arg(short, long, default_value = "GET")]
    method: String,

    #[arg(short = 'H', long = "header")]
    headers: Vec<String>,

    #[arg(short, long)]
    file: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::parse();

    let file_path = &opt.file;
    let url_template = &opt.url;
    let method = &opt.method;

    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let headers = rdr.headers()?.clone();

    let mut targets_file = File::create("targets.txt")?;

    for result in rdr.records() {
        let record = result?;

        let record_map: HashMap<String, String> = headers
            .iter()
            .zip(record.iter())
            .map(|(header, value)| (header.to_string(), value.to_string()))
            .collect();

        let mut target_entry = url_template.clone();
        for (key, value) in &record_map {
            target_entry = target_entry.replace(&format!("{{{}}}", key), value);
        }

        let mut request_line = format!("{} {}\n", method, target_entry);

        for header in &opt.headers {
            let mut processed_header = header.clone();
            for (key, value) in &record_map {
                processed_header = processed_header.replace(&format!("{{{}}}", key), value);
            }
            request_line.push_str(&format!("{}\n", processed_header));
        }

        request_line.push_str("\n");

        targets_file.write_all(request_line.as_bytes())?;
    }

    println!("Targets written to targets.txt");

    Ok(())
}
