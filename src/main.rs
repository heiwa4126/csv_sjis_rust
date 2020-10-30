#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

// via
// [character encoding - How to read a non-UTF8 encoded csv file? - Stack Overflow]
// (https://stackoverflow.com/questions/53826986/how-to-read-a-non-utf8-encoded-csv-file)

use encoding_rs::SHIFT_JIS;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::fs::File;

fn try_main() -> anyhow::Result<()> {
    let file = File::open("./test/csv_sjis/sjis.csv")?;
    let transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(SHIFT_JIS))
        .build(file);
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(transcoded);
    for result in rdr.records() {
        let r = result?;
        println!("{:?}", r);
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    try_main()?;
    Ok(())
}
