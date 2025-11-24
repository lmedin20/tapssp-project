mod cli;
mod csv_reader;
mod filter;
mod stats;
mod errors;
mod viz;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};
use rusqlite::{Connection, params, ToSql};
use rusqlite::types::ValueRef;
use std::time::Instant;

fn main() -> Result<()> {
    let args = Cli::parse();
    match args.cmd {
        Command::Filter { file, filter } =>
            run_filter(&file, &filter),
        Command::Stats { file, col, filter } =>
            run_stats(&file, &col, filter.as_deref()),
        Command::Viz { file, col, bins } =>
            run_viz(&file, &col, bins),
        Command::Bench { file } =>
            run_bench(&file),
        Command::SqlExport { csv_file, db_file } =>
            run_sql_export(&csv_file, &db_file),
        Command::SqlQuery { db_file, query } =>
            run_sql_query(&db_file, &query),
    }
}
fn run_filter(file: &str, filter: &str) -> Result<()> {
    let start = Instant::now();
    let mut rdr = csv_reader::open(file)?;
    let expr = filter::parse(filter, rdr.headers())?;
    let mut count = 0;
    while let Some(rec) = rdr.next_record()? {
        if expr.eval(&rec)? {
            println!("{}", rec.join(","));
            count += 1;
        }
    }
    println!("Rows matched: {count}");
    println!("Filter time: {:?}", start.elapsed());
    Ok(())
}
fn run_stats(file: &str, col: &str, filt: Option<&str>) -> Result<()> {
    let start = Instant::now();
    let mut rdr = csv_reader::open(file)?;
    let col_idx = rdr.index_of(col)?;
    let expr = filt
        .map(|f| filter::parse(f, rdr.headers()))
        .transpose()?;
    let mut values = Vec::new();
    let mut s = stats::Running::default();
    while let Some(rec) = rdr.next_record()? {
        if expr.as_ref()
            .map(|e| e.eval(&rec))
            .transpose()?
            .unwrap_or(true)
        {
            if let Ok(x) = rec[col_idx].parse::<f64>() {
                s.update(x);
                values.push(x);
            }
        }
    }
    println!("count: {}", s.count);
    println!("min: {}", s.min);
    println!("max: {}", s.max);
    println!("mean: {}", s.mean());
    stats::full_stats(&values);
    println!("Stats time: {:?}", start.elapsed());
    Ok(())
}
fn run_viz(file: &str, col: &str, bins: usize) -> Result<()> {
    let start = Instant::now();
    let mut rdr = csv_reader::open(file)?;
    let col_idx = rdr.index_of(col)?;
    let mut values = Vec::new();
    while let Some(rec) = rdr.next_record()? {
        if let Ok(x) = rec[col_idx].parse::<f64>() {
            values.push(x);
        }
    }
    viz::draw_histogram(&values, bins);
    println!("Viz time: {:?}", start.elapsed());
    Ok(())
}
fn run_bench(file: &str) -> Result<()> {
    println!("--- Performance Benchmark ---");
    let t_load = Instant::now();
    let rdr = csv_reader::open(file)?;
    println!("CSV load: {:?}", t_load.elapsed());
    let t_stats = Instant::now();
    drop(rdr);
    let mut rdr = csv_reader::open(file)?;
    let mut values = Vec::new();
    let col_idx = 3;
    while let Some(rec) = rdr.next_record()? {
        if let Ok(v) = rec[col_idx].parse::<f64>() {
            values.push(v);
        }
    }
    stats::full_stats(&values);
    println!("Stats: {:?}", t_stats.elapsed());
    Ok(())
}
fn run_sql_export(csv_file: &str, db_file: &str) -> Result<()> {
    println!("Exporting {} → {}", csv_file, db_file);
    let mut rdr = csv_reader::open(csv_file)?;
    let headers = rdr.headers().to_vec();
    let conn = Connection::open(db_file)?;
    let cols: Vec<String> =
        headers.iter().map(|h| format!("\"{}\" TEXT", h)).collect();
    let ddl = format!("CREATE TABLE IF NOT EXISTS data ({})", cols.join(", "));
    conn.execute(&ddl, [])?;
    let placeholders = vec!["?"; headers.len()].join(", ");
    let insert_sql = format!("INSERT INTO data VALUES ({})", placeholders);
    let mut stmt = conn.prepare(&insert_sql)?;
    while let Some(rec) = rdr.next_record()? {
        let row: Vec<String> = rec.iter().cloned().collect();
        let refs: Vec<&dyn ToSql> = row.iter().map(|s| s as &dyn ToSql).collect();
        stmt.execute(&refs[..])?;
    }
    println!("Done.");
    Ok(())
}
fn run_sql_query(db_file: &str, query: &str) -> Result<()> {
    let conn = Connection::open(db_file)?;
    println!("Running SQL:\n{}", query);
    let mut stmt = conn.prepare(query)?;
    let col_count = stmt.column_count();
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let mut values = Vec::new();
        for i in 0..col_count {
            let v = row.get_ref(i)?;
            let text = match v {
                ValueRef::Null => "NULL".to_string(),
                ValueRef::Integer(x) => x.to_string(),
                ValueRef::Real(x) => x.to_string(),
                ValueRef::Text(x) => String::from_utf8_lossy(x).to_string(),
                ValueRef::Blob(_) => "<BLOB>".to_string(),
            };
            values.push(text);
        }
        println!("{}", values.join(" | "));
    }
    Ok(())
}






