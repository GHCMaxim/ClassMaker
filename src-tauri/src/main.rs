// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use office::{DataType, Excel};
use serde::Serialize;
use serde_json::json;
use std::clone::Clone;
use std::sync::Mutex;

trait DataTypeDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl DataTypeDisplay for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Int(value) => write!(f, "{}", value),
            DataType::Float(value) => write!(f, "{}", value),
            DataType::String(value) => write!(f, "{}", value),
            DataType::Bool(value) => write!(f, "{}", value),
            DataType::Error(error_type) => write!(f, "{:?}", error_type),
            DataType::Empty => write!(f, ""),
        }
    }
}
#[derive(Serialize, Copy, Clone)]
struct Time {
    start: i32,
    end: i32,
    day: i32,
}

#[derive(Serialize)]
struct ResultData {
    subject_id: String,
    class_id: String,
    included_id: String,
    class_title: String,
    credit: String,
    note: String,
    data: Vec<Time>,
    location: String,
    class_type: String,
    validity: String,
}

struct SharedState {
    // excel is a mutex of either an excel file, or null
    excel: Mutex<Option<Excel>>,
    available_positions: Mutex<Vec<Vec<i32>>>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn parse_file(state: tauri::State<'_, SharedState>, path: String) -> Result<String, String> {
    let mut shared_state = state.excel.lock().unwrap();
    *shared_state = Some(Excel::open(path).unwrap());
    if let Ok(range) = shared_state.as_mut().unwrap().worksheet_range("Sheet1") {
        let total_cells = range.get_size().0 * range.get_size().1;
        let non_empty_cells: usize = range
            .rows()
            .map(|r| r.iter().filter(|cell| cell != &&DataType::Empty).count())
            .sum();
        Ok(format!(
            "{} cells, {} non-empty cells",
            total_cells, non_empty_cells
        ))
    } else {
        Err("Error".to_string())
    }
}

fn parse_time(date: String, time: String) -> Time {
    let parsed_date = date.parse::<i32>().unwrap();
    let parsed_time = time.split('-').collect::<Vec<&str>>();
    let parsed_start_time = parsed_time[0].parse::<i32>().unwrap();
    let parsed_end_time = parsed_time[1].parse::<i32>().unwrap();
    Time {
        start: parsed_start_time,
        end: parsed_end_time,
        day: parsed_date,
    }
}

#[tauri::command]
fn sort_by_class_id(
    state: tauri::State<'_, SharedState>,
    subject_id: String,
) -> Result<serde_json::value::Value, String> {
    let mut excel = state.excel.lock().unwrap();
    if let Ok(range) = excel.as_mut().unwrap().worksheet_range("Sheet1") {
        let mut result: Vec<ResultData> = Vec::new();
        let mut i = 0;
        for row in range.rows() {
            if let DataType::String(value) = &row[4] {
                if value == &subject_id {
                    let s = if let DataType::String(value) = &row[4] {
                        value.to_string()
                    } else {
                        format!("{:?}", row[4])
                    };
                    let class_id = if let DataType::Float(value) = &row[2] {
                        (*value as i32).to_string()
                    } else {
                        format!("{:?}", row[2])
                    };
                    let included_id = if let DataType::Float(value) = &row[3] {
                        (*value as i32).to_string()
                    } else {
                        format!("{:?}", row[3])
                    };
                    let class_title = if let DataType::String(value) = &row[5] {
                        value.to_string()
                    } else {
                        format!("{:?}", row[5])
                    };
                    let credit = if let DataType::String(value) = &row[7] {
                        value.to_string()
                    } else {
                        format!("{:?}", row[7])
                    };
                    let note = if let DataType::String(value) = &row[8] {
                        value.to_string()
                    } else {
                        format!("{:?}", row[8])
                    };
                    let date = if let DataType::Float(value) = &row[10] {
                        (*value as i32).to_string()
                    } else {
                        format!("{:?}", row[10])
                    };
                    let time = if let DataType::String(value) = &row[11] {
                        value.to_string()
                    } else {
                        format!("{:?}", row[11])
                    };
                    let location = if let DataType::String(value) = &row[16] {
                        value.to_string()
                    } else {
                        format!("{:?}", row[16])
                    };
                    let class_type = if let DataType::String(value) = &row[22] {
                        value.to_string()
                    } else {
                        format!("{:?}", row[21])
                    };
                    let validity = if let DataType::String(value) = &row[23] {
                        value.to_string()
                    } else {
                        format!("{:?}", row[23])
                    };
                    let parsed_time = vec![parse_time(date, time)];
                    let data = ResultData {
                        subject_id: s,
                        class_id,
                        included_id,
                        class_title,
                        credit,
                        note,
                        data: parsed_time,
                        location,
                        class_type,
                        validity,
                    };
                    if i == 0 {
                        result.push(data);
                        i += 1;
                        continue;
                    }
                    if result[i - 1].class_id == data.class_id {
                        result[i - 1].data.push(data.data[0]);
                        continue;
                    }
                    result.push(data);
                    i += 1;
                }
            }
        }
        Ok(json!(result))
    } else {
        Err("Error".to_string())
    }
}

fn get_included_class(
    state: tauri::State<'_, SharedState>,
    included_id: String,
) -> Result<ResultData, String> {
    let mut excel = state.excel.lock().unwrap();
    if let Ok(range) = excel.as_mut().unwrap().worksheet_range("Sheet1") {
        let mut result: ResultData = ResultData {
            subject_id: "".to_string(),
            class_id: "".to_string(),
            included_id: "".to_string(),
            class_title: "".to_string(),
            credit: "".to_string(),
            note: "".to_string(),
            data: Vec::new(),
            location: "".to_string(),
            class_type: "".to_string(),
            validity: "".to_string(),
        };
        let mut i = 0;
        for row in range.rows() {
            if let DataType::String(value) = &row[2] {
                if value == &included_id && i == 0 {
                    let s = if let DataType::String(value) = &row[4] {
                        value.to_string()
                    } else {
                        format!("{:?}", row[4])
                    };
                    let class_id = if let DataType::Float(value) = &row[2] {
                        (*value as i32).to_string()
                    } else {
                        format!("{:?}", row[2])
                    };
                    let included_id = if let DataType::Float(value) = &row[3] {
                        (*value as i32).to_string()
                    } else {
                        format!("{:?}", row[3])
                    };
                    let class_title = if let DataType::String(value) = &row[5] {
                        value.to_string()
                    } else {
                        format!("{:?}", row[5])
                    };
                    let credit = if let DataType::String(value) = &row[7] {
                        value.to_string()
                    } else {
                        format!("{:?}", row[7])
                    };
                    let note = if let DataType::String(value) = &row[8] {
                        value.to_string()
                    } else {
                        format!("{:?}", row[8])
                    };
                    let date = if let DataType::Float(value) = &row[10] {
                        (*value as i32).to_string()
                    } else {
                        format!("{:?}", row[10])
                    };
                    let time = if let DataType::String(value) = &row[11] {
                        value.to_string()
                    } else {
                        format!("{:?}", row[11])
                    };
                    let location = if let DataType::String(value) = &row[16] {
                        value.to_string()
                    } else {
                        format!("{:?}", row[16])
                    };
                    let class_type = if let DataType::String(value) = &row[22] {
                        value.to_string()
                    } else {
                        format!("{:?}", row[21])
                    };
                    let validity = if let DataType::String(value) = &row[23] {
                        value.to_string()
                    } else {
                        format!("{:?}", row[23])
                    };
                    let parsed_time = vec![parse_time(date, time)];
                    let data = ResultData {
                        subject_id: s,
                        class_id,
                        included_id,
                        class_title,
                        credit,
                        note,
                        data: parsed_time,
                        location,
                        class_type,
                        validity,
                    };
                    result = data;
                } else if value == &included_id && i != 0 {
                    let date = if let DataType::Float(value) = &row[10] {
                        (*value as i32).to_string()
                    } else {
                        format!("{:?}", row[10])
                    };
                    let time = if let DataType::String(value) = &row[11] {
                        value.to_string()
                    } else {
                        format!("{:?}", row[11])
                    };
                    let parsed_time = vec![parse_time(date, time)];
                    result.data.push(parsed_time[0]);
                    continue;
                }
            }
        }
        Ok(result)
    } else {
        Err("Error".to_string())
    }
}

fn check_validity(state: tauri::State<'_, SharedState>, time: Vec<Time>) -> bool {
    let mut valid = true;
    let mut available_positions = state.available_positions.lock().unwrap();
    for i in 0..time.len() {
        if available_positions[time[i].day as usize - 2].contains(&time[i].start)
            || available_positions[time[i].day as usize - 2].contains(&time[i].end)
        {
            valid = false;
            break;
        }
        available_positions[time[i].day as usize - 2].push(time[i].start);
        available_positions[time[i].day as usize - 2].push(time[i].end);
        available_positions[time[i].day as usize - 2].sort();
        for j in 0..available_positions[time[i].day as usize - 2].len() {
            if available_positions[time[i].day as usize - 2][j] == time[i].start
                && available_positions[time[i].day as usize - 2][j + 1] == time[i].end
            {
                break;
            }
            valid = false;
            available_positions[time[i].day as usize - 2].remove(j);
            let index = available_positions[time[i].day as usize - 2]
                .iter()
                .position(|x| *x == time[i].end)
                .unwrap();
            available_positions[time[i].day as usize - 2].remove(index);
        }
    }
    valid
}

fn main() {
    let initial_positions: Vec<Vec<i32>> = vec![Vec::new(); 6];
    tauri::Builder::default()
        .manage(SharedState {
            excel: Mutex::new(None),
            available_positions: Mutex::new(initial_positions),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            parse_file,
            sort_by_class_id
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
