[![rust_CICD](https://github.com/nogibjj/Individual_Proj2_Fuyao_Li/actions/workflows/cicd.yml/badge.svg)](https://github.com/nogibjj/Individual_Proj2_Fuyao_Li/actions/workflows/cicd.yml)
## Individual Project2

Author: Fuyao Li

### Requirements:
+ Rust source code: The code should comprehensively understand Rust's syntax and unique features.
+ Use of LLM: In your README, explain how you utilized an LLM in your coding process.
+ SQLite Database: Include a SQLite database and demonstrate CRUD (Create, Read, Update, Delete) operations.
+ Optimized Rust Binary: Include a process that generates an optimized Rust binary as a Gitlab Actions artifact that can be downloaded.
+ README.md: A file that clearly explains what the project does, its dependencies, how to run the program, and how Gitlab Copilot was used.
+ Github/Gitlab Actions: A workflow file that tests, builds, and lints your Rust code.
+ Video Demo: A YouTube link in README.md showing a clear, concise walkthrough and demonstration of your CLI binary.

### Preparation:
+ Init: `cargo init`
+ Build: `cargo build --release`
+ Run: `cargo run`
+ Test: `cargo test`

### `individual_proj2_fuyao_li` Structure  
```plaintext
individual_proj2_fuyao_li
├── Cargo.toml
├── src
│   ├── lib.rs
│   └── main.rs
└── tests
    └── integration_tests.rs
```

### Sample CRUD Operations:
+ Create
    ``` rust 
    create_subject("10/25/2022", "Durham South", "Durham", "NC", 34.05, -118.25)
    ```
+ Read
    ``` rust
    read_data()
    ```
+ Update
    ``` rust 
    update_subject(107, "10/26/2020", "Durham South", "Durham", "NC", 34.05, -118.25)
    ```
+ Delete
    ``` rust
    delete_subject(1)
    ```

### Result:
All CRUD Operations are shown in `db_log.md`.

### Github Copilot Usage:
GitHub Copilot can assist in generating functions by suggesting code snippets and completions based on the comments and code context provided. I used Github Copilot to generate the test function of this function:

> Original function
```rust
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
```

> Test function 
```rust
#[test]
fn test_read_data() {
    let result = read_data();
    assert!(result.is_ok(), "Read data should work without errors");

    let data = result.unwrap();
    assert!(
        !data.is_empty(),
        "Data should not be empty after reading from database"
    );
}
```

### Use of LLM:
The use of an LLM (Language Learning Model) like GitHub Copilot is specified for assisting in code generation. Specifically, Copilot helps by providing test functions of CRUD functions for SQLite. This model enhances efficiency with quick prototype generation based on function descriptions.

### Link of vedio
https://youtu.be/k9SGEq9PlPU

### References:
https://github.com/nogibjj/sqlite-lab
### Data resource:
https://github.com/fivethirtyeight/data/blob/master/presidential-campaign-trail/trump.csv
