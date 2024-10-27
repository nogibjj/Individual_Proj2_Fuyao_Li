use Individual_Proj2_Fuyao_Li::{extract, load_data, create_subject, read_data, update_subject, delete_subject};

fn main() {
    // Example usage of extract
    let url = "https://github.com/fivethirtyeight/data/blob/master/presidential-campaign-trail/trump.csv?raw=true";
    let file_path = "data/trump.csv";
    if let Err(e) = extract(url, file_path) {
        eprintln!("Error in extract: {}", e);
    }

    // Load data into the database
    if let Err(e) = load_data(file_path) {
        eprintln!("Error in load_data: {}", e);
    }

    // Read data from the database
    match read_data() {
        Ok(data) => println!("Data: {:?}", data),
        Err(e) => eprintln!("Error in read_data: {}", e),
    }

    // Insert a new subject
    if let Err(e) = create_subject("10/25/2022", "Durham South", "Durham", "NC", 34.05, -118.25) {
        eprintln!("Error in create_subject: {}", e);
    }

    // Update an existing subject
    if let Err(e) = update_subject(107, "10/26/2020", "Durham South", "Durham", "NC", 34.05, -118.25) {
        eprintln!("Error in update_subject: {}", e);
    }

    // Delete a subject
    if let Err(e) = delete_subject(1) {
        eprintln!("Error in delete_subject: {}", e);
    }
}

