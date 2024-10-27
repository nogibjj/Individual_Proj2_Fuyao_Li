use rusqlite::{params, Connection, Result};
use std::fs::{self, File, OpenOptions};
use std::path::Path;
use csv::ReaderBuilder;
use reqwest::blocking::get;
use std::error::Error;
use std::io::Write;

const LOG_FILE: &str = "db_log.md";

// Function to extract data from a URL and save to a file
pub fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = Path::new(file_path).parent() {
        fs::create_dir_all(parent)?;
    }

    let response = get(url)?;
    if response.status().is_success() {
        let mut file = File::create(file_path)?;
        file.write_all(&response.bytes()?)?;
        println!("File successfully downloaded to {}", file_path);
    } else {
        println!("Failed to retrieve the file from {}.", url);
    }

    Ok(())
}

// Function to load data from CSV file to SQLite database
pub fn load_data(dataset: &str) -> Result<(), Box<dyn Error>> {
    // Make the connection mutable
    let mut conn = Connection::open("CityDB.db")?;
    conn.execute("DROP TABLE IF EXISTS CityDB", [])?;
    conn.execute(
        "CREATE TABLE CityDB (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT,
            location TEXT,
            city TEXT,
            state TEXT,
            lat REAL,
            lng REAL
        )",
        [],
    )?;

    let file = File::open(dataset)?;
    let mut rdr = ReaderBuilder::new().delimiter(b',').has_headers(true).from_reader(file);

    // Start a transaction
    let tx = conn.transaction()?;
    for result in rdr.records() {
        let record = result?;
        tx.execute(
            "INSERT INTO CityDB (date, location, city, state, lat, lng) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                &record[0],
                &record[1],
                &record[2],
                &record[3],
                record[4].parse::<f64>()?,
                record[5].parse::<f64>()?,
            ],
        )?;
    }
    tx.commit()?;
    println!("Data loaded into CityDB.db");
    Ok(())
}

// Function to add SQL operation to log file
fn add_operation(query: &str) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().append(true).create(true).open(LOG_FILE)?;
    writeln!(file, "```sql\n{}\n```\n", query)?;
    Ok(())
}

// Function to read data from database
type DataRow = (i32, String, String, String, String, f64, f64);

pub fn read_data() -> Result<Vec<DataRow>, Box<dyn Error>> {
    let conn = Connection::open("CityDB.db")?;
    let mut stmt = conn.prepare("SELECT * FROM CityDB")?;
    let data = stmt
        .query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    add_operation("SELECT * FROM CityDB")?;
    Ok(data)
}

// Function to insert data into the database
pub fn create_subject(date: &str, location: &str, city: &str, state: &str, lat: f64, lng: f64) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("CityDB.db")?;
    conn.execute(
        "INSERT INTO CityDB (date, location, city, state, lat, lng) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![date, location, city, state, lat, lng],
    )?;

    add_operation(&format!(
        "INSERT INTO CityDB (date, location, city, state, lat, lng) VALUES ('{}', '{}', '{}', '{}', {}, {});",
        date, location, city, state, lat, lng
    ))?;
    Ok(())
}

// Function to update a subject in the database
pub fn update_subject(record_id: i32, date: &str, location: &str, city: &str, state: &str, lat: f64, lng: f64) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("CityDB.db")?;
    conn.execute(
        "UPDATE CityDB SET date = ?1, location = ?2, city = ?3, state = ?4, lat = ?5, lng = ?6 WHERE id = ?7",
        params![date, location, city, state, lat, lng, record_id],
    )?;

    add_operation(&format!(
        "UPDATE CityDB SET date='{}', location='{}', city='{}', state='{}', lat={}, lng={} WHERE id={};",
        date, location, city, state, lat, lng, record_id
    ))?;
    Ok(())
}

// Function to delete a subject from the database
pub fn delete_subject(record_id: i32) -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("CityDB.db")?;
    conn.execute("DELETE FROM CityDB WHERE id = ?1", params![record_id])?;

    add_operation(&format!("DELETE FROM CityDB WHERE id={};", record_id))?;
    Ok(())
}
