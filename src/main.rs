/*
  A highly-specialized tool for parsing eTrade CSV output into a format that I use in my analysis
  spreadsheets.

  To use:
    1. Go to eTrade. Click "Portfolio" and view by "All Positions."
    2. Click the double-chevron next to the word "Symbol." This will expand all of the stocks to
       show their individual lots with purchase dates.
    3. Download the CSV file and rename it something useful. Preferably with a date in the name.
    4. Then `etrade-tool < THE_DOWNLOADED_CSV_FILE`. This will output to stdout a new CSV file
       organized so that it can be imported directly into Google Sheets.

  To import to my sheet:
    1. Create a copy of the last imported sheet (or the "test import" sheet).
    2. Delete all of the values on the left side of the sheet.
    3. Click on cell A1, and File->Import. Upload the output from the etrade-tool.
       Import location: Replace data at selected cell
       Separator type: Detect automatically works fine.
       Convert text to numbers, dates, and formulas: Checked
       Then click "Import Data"
    4. You may need to "fill" more rows on the bottom of the sheet or remove some rows.
*/

use chrono::NaiveDate;
use csv::StringRecord;
use std::num::ParseFloatError;

// TODO: look up the fields you use in this table to find the index.
// TODO: verify when parsing the header that all of these fields match.
static STOCK_HEADERS: &[&str] = &[
    "Symbol",
    "Last Price $",
    "Change $",
    "Change %",
    "Day's Gain $",
    "Qty #",
    "Price Paid $",
    "Total Gain $",
    "Total Gain %",
    "Value $",
];

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("CSV parsing error: {error:?}")]
    CsvError {
        #[from]
        error: csv::Error,
    },

    #[error("Header row not found in input data")]
    HeaderRowNotFound,

    // TODO: include row number?
    #[error("Missing field #{0}")]
    MissingField(u8),

    #[error("Parse float error: {error:?}")]
    ParseFloatError {
        #[from]
        error: ParseFloatError,
    },
}
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    // We are going to post-process a very inconsistent input file to
    // 1) find the data we care about,
    // 2) validate that the data is as expected.
    //
    // For these reasons, we make it "flexible". Oh, and trim() can't hurt.
    // The data we care about is halfway through the file, so we don't treat the first row as
    // special at all.
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .trim(csv::Trim::All)
        .from_reader(std::io::stdin());

    let mut converter = Converter::default();
    for result in rdr.records() {
        let record = result?;
        converter.process_record(record)?;
    }

    if converter.did_read_data() == false {
        return Err(Error::HeaderRowNotFound);
    }

    converter.to_csv(std::io::stdout())?;
    Ok(())
}

#[derive(Debug, Eq, PartialEq)]
enum Mode {
    SkippingStart,
    ReadingData,
    SkippingEnd,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::SkippingStart
    }
}

#[derive(Debug, Default)]
struct Converter {
    mode: Mode,
    current_symbol: String,
    lots: Vec<Lot>,
}

impl Converter {
    fn did_read_data(&self) -> bool {
        self.mode != Mode::SkippingStart
    }

    fn process_record(&mut self, record: StringRecord) -> Result<()> {
        self.mode = match self.mode {
            Mode::SkippingStart => self.look_for_headers(record)?,
            Mode::ReadingData => self.handle_data_record(record)?,
            Mode::SkippingEnd => Mode::SkippingEnd,
        };
        Ok(())
    }

    fn look_for_headers(&self, record: StringRecord) -> Result<Mode> {
        if record.get(0).unwrap_or("XXX") == STOCK_HEADERS[0]
            && record.get(1).unwrap_or("XXX") == STOCK_HEADERS[1]
        {
            // TODO: maybe check all headers to ensure the output from eTrade hasn't changed.
            Ok(Mode::ReadingData)
        } else {
            Ok(Mode::SkippingStart)
        }
    }

    fn handle_data_record(&mut self, record: StringRecord) -> Result<Mode> {
        if let Some(fld) = record.get(0) {
            if fld == "CASH" {
                return Ok(Mode::SkippingEnd);
            }

            if let Ok(purchase_date) = NaiveDate::parse_from_str(fld, "%m/%d/%Y") {
                // A "lot" row
                let symbol = self.current_symbol.clone();
                let total_value = record.get(9).unwrap_or("").parse()?;
                let total_gain = record.get(7).unwrap_or("").parse()?;
                let total_paid = total_value - total_gain;
                let quantity = record.get(4).unwrap_or("").parse()?;
                let price_paid = record.get(5).unwrap_or("").parse()?;
                let lot = Lot {
                    symbol,
                    quantity,
                    price_paid,
                    purchase_date,
                    total_paid,
                    total_gain,
                    total_value,
                };
                self.lots.push(lot);
            } else {
                // A "stock" row
                self.current_symbol = fld.to_string();
            }
        } else {
            return Err(Error::MissingField(0));
        }

        Ok(Mode::ReadingData)
    }

    fn to_csv(&self, out: impl std::io::Write) -> Result<()> {
        let mut writer = csv::Writer::from_writer(out);
        writer.write_record(&["Symbol", "Purchase Date", "Quantity", "Price Paid", "Total Paid", "Total Gain", "Total Value"])?;

        for lot in &self.lots {
            writer.write_record(&[
                &lot.symbol,
                &lot.purchase_date.format("%m/%d/%Y").to_string(),
                &lot.quantity.to_string(),
                &lot.price_paid.to_string(),
                &lot.total_paid.to_string(),
                &lot.total_gain.to_string(),
                &lot.total_value.to_string(),
                ]
            )?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Lot {
    symbol: String,
    quantity: f64,
    price_paid: f64,
    purchase_date: NaiveDate,
    total_paid: f64,
    total_gain: f64,
    total_value: f64,
}
