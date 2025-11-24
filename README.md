# Rust Data Inspector
**Course:** Safe Systems Programming  
**Repository:** tapssp-project
**Author:** Lincon Medina Jr  

Project Description:
Rust Data Inspector is a command-line tool for exploring, filtering, and summarizing CSV datasets. The program allows users to apply custom filter expressions (for example, age > 30 and salary < 100000) and compute summary statistics such as count, mean, minimum, and maximum values for selected columns.

The project demonstrates how Rust’s ownership, borrowing, error handling, and type safety features can be applied to real-world data processing tasks. It also highlights the use of traits, generics, and modular design to create efficient and reliable systems software.

I'm aiming for a less complex version by week 7 and following it with a future version that will include advanced features like saving filtered outputs, visualizing distributions directly in the terminal, and possibly building a text-based interface for interactive data analysis.

Link to Demo: https://youtu.be/2krMbCEHz2U?si=VVvCZxu0fyfoauOR

Post Project Discussion:
Overall goal in making this project was to replicate Data analysis work I've done in the past but in Rust. The Project lets you load CSV files, filter them using expressions like > AND ==, do basic statistics, visualize data with a histogram, and even run SQL queries on the dataset.

What I learned about Rust through the project:
I learned how Rust handles ownership and borrowing, especially when doing CSV data row-by-row. I also learned how to write a custom expression parser, handle errors cleanly with anyhow, and interact with external crates like clap, csv, and rusqlite. It was really cool to learn how to run SQL commands through Rust.

What was easy in the project:
Rust’s packages was surprisingly easy to use. Crates like clap, csv, and rusqlite made it very simple to build the main features. Also, once I understood ownership, writing code with small files felt very easy and allowed clean coding practices.

What was difficult:

- Debugging Windows-specific issues like file locks and toolchain problems. I had a major issue that lead to my Rust files being corrupted and needing to be reinstalled. Plus specific errors were completely unknown to me and were a headache.

- Making SQLite display results correctly, including reading different value types.

Problems I encountered and how I solved them:

- Multiversion crate conflicts with DuckDB: I originally tried using DuckDB for my SQL but I removed DuckDB and switched to SQLite, which was simpler and more reliable. DuckDB had an error with arrow so I had to change.

- Parser errors due to string operators and parentheses: I rewrote the parser using a top-level splitting approach.

- Ownership and other errors when inserting rows into SQLite: fixed by converting rows into a vector of &dyn ToSql. It worked because we converted each CSV value into a form &dyn ToSql that the rusqlite crate can safely insert into the database.

- Incorrect column names causing filters to fail: added better error messages to allow me to determine where the error was and why.

Overall, the project taught me how to build a real, production-style data tool in Rust and how to debug more technical issues. I really enjoyed this project allowing me to use my love for data analysis in a Rust project. I really hope I did a good job of showing off Rust and it's capabilites on the data analysis front.