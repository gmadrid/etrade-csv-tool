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
 */

fn main() {
    
}
