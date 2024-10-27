use Individual_Proj2_Fuyao_Li::{extract, load_data, create_subject, read_data, update_subject, delete_subject};
use std::fs;

#[test]
fn test_extract() {
    let url = "https://github.com/fivethirtyeight/data/blob/master/presidential-campaign-trail/trump.csv?raw=true";
    let file_path = "data/trump.csv";
    
    let result = extract(url, file_path);
    assert!(result.is_ok(), "Extract function should work without errors");

    // Check if file exists
    let metadata = fs::metadata(file_path);
    assert!(metadata.is_ok(), "File should exist after extraction");
}

#[test]
fn test_load_data() {
    let file_path = "data/trump.csv";

    let result = load_data(file_path);
    assert!(result.is_ok(), "Load data should work without errors");
}

#[test]
fn test_create_subject() {
    let result = create_subject("2022-10-25", "Durham South", "Durham", "NC", 34.05, -118.25);
    assert!(result.is_ok(), "Create subject should work without errors");
}

#[test]
fn test_read_data() {
    let result = read_data();
    assert!(result.is_ok(), "Read data should work without errors");

    let data = result.unwrap();
    assert!(!data.is_empty(), "Data should not be empty after reading from database");
}

#[test]
fn test_update_subject() {
    // Insert a subject to update
    create_subject("2022-10-25", "Durham South", "Durham", "NC", 34.05, -118.25).expect("Failed to create subject");

    let result = update_subject(1, "2022-10-26", "Durham North", "Durham", "NC", 34.05, -118.25);
    assert!(result.is_ok(), "Update subject should work without errors");
}

#[test]
fn test_delete_subject() {
    // Insert a subject to delete
    create_subject("2022-10-25", "Durham South", "Durham", "NC", 34.05, -118.25).expect("Failed to create subject");

    let result = delete_subject(1);
    assert!(result.is_ok(), "Delete subject should work without errors");
}
