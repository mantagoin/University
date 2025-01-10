use csv::{ReaderBuilder, WriterBuilder};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // نوشتن به فایل CSV
    let mut wtr = WriterBuilder::new().has_headers(true).from_writer(std::io::stdout());

    wtr.write_record(&["نام", "سن", "شهر"])?;
    wtr.write_record(&["علی", "25", "تهران"])?;
    wtr.write_record(&["مریم", "30", "شیراز"])?;
    
    // ذخیره کردن به فایل
    let mut wtr_file = csv::Writer::from_path("data.csv")?;
    wtr_file.write_record(&["نام", "سن", "شهر"])?;
    wtr_file.write_record(&["علی", "25", "تهران"])?;
    wtr_file.write_record(&["مریم", "30", "شیراز"])?;
    wtr_file.flush()?;

    // خواندن از فایل CSV
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path("data.csv")?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}
