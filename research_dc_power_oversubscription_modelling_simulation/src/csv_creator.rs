use csv::Writer;
use prettytable::Table;

pub fn export_table_to_csv(table: &Table, filename: &str) -> Result<(), csv::Error> {
    let mut writer = Writer::from_path(filename)?;

    // Write the column headers to the CSV file
    let headers: Vec<String> = vec![
        "User ID".to_string(),
        "Initial Power Consumption".to_string(),
        "Resource Reduction".to_string(),
        "Updated Power Consumption".to_string(),
        "Bid Amount".to_string(),
        "Power Saved".to_string()
    ];
    writer.write_record(headers)?;

    // Write the table rows to the CSV file
    for row in table.row_iter() {
        let mut csv_row = Vec::new();
        for cell in row.iter() {
            csv_row.push(cell.to_string());
        }
        writer.write_record(csv_row)?;
    }

    writer.flush()?;
    Ok(())
}