use csv::ReaderBuilder;
use csv::WriterBuilder;
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn split_csv(input_file: &str, split_after_line: usize) -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(input_file)?;

    let headers = rdr.headers()?.clone();
    let mut record_iter = rdr.records();

    let mut file_count = 1;
    let mut record_count = 0;

    let binding = input_file.split("/").collect::<Vec<&str>>();
    let (file, dir) = binding.split_last().unwrap();
    let file_name = file.split("/").last().unwrap().replace(".csv", "");
    let dir_name = dir.join("/");

    let path = format!("{}/output", dir_name);
    fs::create_dir_all(path)?;

    let mut wtr = create_writer(&file_name, &dir_name, file_count, &headers)?;

    while let Some(result) = record_iter.next() {
        let record = result?;
        if record_count == split_after_line {
            file_count += 1;
            record_count = 0;
            wtr.flush()?;
            wtr = create_writer(&file_name, &dir_name, file_count, &headers)?;
        }
        wtr.write_record(&record)?;
        record_count += 1;
    }
    wtr.flush()?;

    Ok(())
}

fn create_writer(
    file_name: &str,
    dir_name: &str,
    file_count: usize,
    headers: &csv::StringRecord,
) -> Result<csv::Writer<File>, Box<dyn Error>> {
    let new_file = format!("{}/output/{}_part{}.csv", dir_name, file_name, file_count);
    println!("{}", new_file);
    let file = File::create(&new_file)?;
    let mut wtr = WriterBuilder::new().from_writer(file);
    wtr.write_record(headers)?;
    Ok(wtr)
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn split(file: String, rowN: usize) -> String {
    println!("split kicked, file path: {}, row: {}", &file, &rowN);
    let input_file = file;
    let split_after_line: usize = rowN;

    if let Err(err) = split_csv(input_file.as_str(), split_after_line) {
        format!("CSVの分割に失敗しました。: {}", err)
    } else {
        "CSVファイルの分割が完了しました。".into()
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![split])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
