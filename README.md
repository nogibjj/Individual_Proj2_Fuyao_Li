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
+ Build: `cargo build`
+ Run: `cargo run`
+ Test: `cargo test`

### Sample CRUD Operations:
+ Create
    ``` python 
    create_query1 = ("6/27/2018", "Durham South", "Durham", "NC", 35.99, 78.90)
    create_subject(create_query1)
    ```
+ Read
    ``` python
    read_data()
    ```
+ Update
    ``` python 
    update_query = (107, "8/29/2018", "Durham South", "Durham", "NC", 35.99, 78.90)
    update_subject(update_query)
    ```
+ Delete
    ``` python
    delete_subject(99)
    ```
### Result:
All CRUD Operations are shown in `db_log.md`.

### References:
https://github.com/nogibjj/sqlite-lab
