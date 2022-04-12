use std::io::Read;
use csv::{Reader, StringRecordsIter};
use crate::error::{Result};

pub struct CSVWithHeaders< R> where R: Read {
    rdr: Reader<R>,
}

impl<R> CSVWithHeaders< R> where R:    Read {
    pub fn from_reader(r: R) -> Result<Self> {
        // We are going to post-process a very inconsistent input file to
        // 1) find the data we care about,
        // 2) validate that the data is as expected.
        //
        // For these reasons, we make it "flexible". Oh, and trim() can't hurt.
        // The data we care about is halfway through the file, so we don't treat the first row as
        // special at all.
        let rdr = csv::ReaderBuilder::new()
            .flexible(true)
            .has_headers(false)
            .trim(csv::Trim::All)
            .from_reader(r);

        Ok(CSVWithHeaders { rdr })
    }

    pub fn records(&mut self) -> StringRecordsIter<R> {
        self.rdr.records()
    }
}