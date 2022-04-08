# etrade-csv-tool

A highly-specialized tool for parsing eTrade CSV output
into a format that I use in my analysis spreadsheets.


### To use:
1. Go to eTrade. Click "Portfolio" and view by "All Positions."
2. Click the double-chevron next to the word "Symbol." This will expand all of the stocks to
show their individual lots with purchase dates.
3. Download the CSV file and rename it something useful. Preferably with a date in the name.
4. Then `etrade-tool < THE_DOWNLOADED_CSV_FILE`. This will output to stdout a new CSV file
organized so that it can be imported directly into Google Sheets.

### To import to my sheet:
1. Create a copy of the last imported sheet.
2. Delete all of the values on the left side of the sheet.
3. Click on cell A1, and File->Import. Upload the output from the etrade-tool.  
   __Import location__: Replace data at selected cell  
   __Separator type__: Detect automatically works fine.  
   __Convert text to numbers, dates, and formulas__: Checked  
     Then click "Import Data"
4. You may need to "fill" more rows on the bottom of the sheet or remove some rows.

