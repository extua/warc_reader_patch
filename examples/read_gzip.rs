use warc::WarcHeader;
use warc::WarcReader;

fn main() -> Result<(), std::io::Error> {
    let file = WarcReader::from_path_gzip("warc_example.warc.gz")?;

    let mut count = 0;
    for record in file.iter_records() {
        count += 1;
        match record {
            Err(err) => println!("ERROR: {}\r\n", err),
            Ok(record) => {
                println!("{}: {}", WarcHeader::RecordID, record.warc_id());
                println!("{}: {}", WarcHeader::Date, record.date());
                println!();
            }
        }
    }

    println!("Total records: {}", count);

    Ok(())
}
