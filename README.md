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

### References:
https://github.com/nogibjj/sqlite-lab
### Data resource:
https://github.com/fivethirtyeight/data/blob/master/presidential-campaign-trail/trump.csv
