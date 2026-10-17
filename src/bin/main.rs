use money_monitor::{import::from_boursobank_csv, units::{Amount, EURO}};

fn main() -> Result<(), String>{
    // Load arguments from command line
    let args = std::env::args().collect::<Vec<String>>();

    // Check arguments
    if args.len() < 2 {
        return Err("Usage: money-monitor <relative file path>".to_string());
    }

    // Open the file
    let file_path = &args[1];
    let file = std::fs::File::open(file_path).map_err(|e| format!("Failed to open file {}: {}", file_path, e))?;

    // Read the content
    let reader = std::io::BufReader::new(file);
    let lines = from_boursobank_csv(reader);

    // Compute total amount and print lines
    let mut total_amount = Amount::new(0.0, &EURO);
    let mut nb_lines = 0;
    for line in lines {
        println!("{}", line);
        total_amount += line.get_amount();
        nb_lines += 1;
    }

    // Print total amount
    println!("Total amount: {total_amount:.2}, on {nb_lines} entries");

    Ok(())

}