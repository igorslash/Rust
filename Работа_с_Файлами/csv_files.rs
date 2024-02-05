fn main() {
    let write_csv = write_csv(
        &Csv{file: "input.csv".to_string(), delimiter: ','});
    let read_csv = read_csv(&Csv{file: "input.csv"
        .to_string(), delimiter: ','});
    println!("{:?} {:?}", read_csv, write_csv);
}
#[Derive(Debug, Serialize, Deserialize)]
struct Csv {
    file: String,
    delimiter: char,
}
fn write_csv(csv: &Csv) -> Result<(), Box<dyn std::error::Error>> {
    let mut wtr = csv::Writer::from_path("input.csv")?;
    wtr.write_record(&["foo", "bar", "baz"])?;
    wtr.write_record(&[1, 2, 3])?;
    wtr.flush()?;
    Ok(())
}

fn read_csv(csv: &Csv) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_path("input.csv")?;
    let headers = rdr.headers()?;
    println!("{:?}", headers);
    for result in rdr.records() {
        let record = result?;
        let deserialize = record.deserialize::<Vec<String>>()?;
        println!("{:?}", record);
        println!("{:?}", deserialize);
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_csv() {
        // Create a Csv instance for testing if needed
        let csv = Csv::new();

        // Call the write_csv function
        let result = write_csv(&csv);

        // Assert that the result is Ok
        assert!(result.is_ok());
    }

    #[test]
    fn test_read_csv() {
        // Create a Csv instance for testing if needed
        let csv = Csv::new();

        // Call the read_csv function
        let result = read_csv(&csv);

        // Assert that the result is Ok
        assert!(result.is_ok());
    }
}
//csv = "1.3.0"
//csv_cerde = "0.1.0"

/*This code is a Rust program that reads from and writes to a CSV file.
It defines a Csv struct with fields file and delimiter.
It also has functions write_csv and read_csv that
write to and read from the CSV file respectively.
The main function initializes a Csv object,
calls the write_csv and read_csv functions, and prints the results.
The code uses the csv and csv_cerde libraries for CSV file handling.
This code write with a library csv and read with a library csv_cerde.*/
