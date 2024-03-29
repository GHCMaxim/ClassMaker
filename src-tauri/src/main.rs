// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use core::time;
use office::{DataType, Excel};
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use std::clone::Clone;
use std::sync::Mutex;
use tauri::utils::config::parse;
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
#[derive(Serialize, Copy, Clone, Deserialize)]
struct Time {
    start: i32,
    end: i32,
    day: i32,
}

#[derive(Serialize, Clone, Deserialize)]
struct TableTime {
    day: String,
    render_top: f32,
    render_height: f32,
    subject_name: String,
    subject_id: String,
    class_id: String,
    class_type: String,
    display_date: String,
    room: String,
}

#[derive(Serialize, Clone, Deserialize)]
struct ResultData {
    subject_id: String,
    class_id: String,
    included_id: String,
    class_title: String,
    credit: String,
    note: String,
    data: Vec<Time>,
    location: String,
    lab: String,
    class_type: String,
    validity: String,
}

struct SharedState {
    // excel is a mutex of either an excel file, or null
    excel: Mutex<Option<Excel>>,
    available_positions: Mutex<Vec<Vec<i32>>>,
    chosen_classes: Mutex<Vec<ResultData>>,
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
                    let lab = if let DataType::String(value) = &row[17] {
                        value.to_string()
                    } else {
                        format!("{:?}", row[17])
                    };
                    let class_type = if let DataType::String(value) = &row[21] {
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
                        lab,
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
    print!("Hello");
    print!("{}", included_id);
    let included_id = included_id.parse::<f64>().unwrap();
    if let Ok(range) = excel.as_mut().unwrap().worksheet_range("Sheet1") {
        print!("The excel file is opened");
        let mut result: ResultData = ResultData {
            subject_id: "".to_string(),
            class_id: "".to_string(),
            included_id: "".to_string(),
            class_title: "".to_string(),
            credit: "".to_string(),
            note: "".to_string(),
            data: Vec::new(),
            lab: "".to_string(),
            location: "".to_string(),
            class_type: "".to_string(),
            validity: "".to_string(),
        };
        let mut i = 0;
        for row in range.rows() {
            if let DataType::Float(value) = &row[2] {
                if value == &included_id && i == 0 {
                    println!("There exists a class!");
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
                    let lab = if let DataType::String(value) = &row[17] {
                        value.to_string()
                    } else {
                        format!("{:?}", row[17])
                    };
                    let class_type = if let DataType::String(value) = &row[21] {
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
                        lab,
                        data: parsed_time,
                        location,
                        class_type,
                        validity,
                    };
                    result = data;
                    i += 1;
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
                } else {
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
                valid = true;
                break;
            }
            valid = false;
        }
        if !valid{
            let index = available_positions[time[i].day as usize - 2]
                .iter()
                .position(|x| *x == time[i].start)
                .unwrap();
            available_positions[time[i].day as usize - 2].remove(index);
            let index = available_positions[time[i].day as usize - 2]
                .iter()
                .position(|x| *x == time[i].end)
                .unwrap();
            available_positions[time[i].day as usize - 2].remove(index);
        }
    }
    valid
}

#[tauri::command]
fn add_chosen_class(
    state: tauri::State<'_, SharedState>,
    data: ResultData,
) -> Result<Vec<TableTime>, String> {
    let mut failed_classes = Vec::new();
    let mut chosen_classes = state.chosen_classes.lock().unwrap();
    let mut classes = Vec::new();
    let mut chosen_ids = Vec::new();
    let mut return_value: Vec<TableTime> = Vec::new();
    let CELL_OFFSET = 60.0;
    let HOUR_OFFSET = 6.0;
    classes.push(data.clone());
    if data.class_id != data.included_id && data.included_id != "NULL" {
        let included = get_included_class(state.clone(), data.included_id).unwrap();
        classes.push(included);
    }
    for i in 0..classes.len() {
        if !check_validity(state.clone(), classes[i].clone().data) {
            failed_classes.push(classes[i].clone().class_id);
        }
    }
    if failed_classes.len() == 0 {
        for i in 0..classes.len() {
            chosen_classes.push(classes[i].clone());
            for j in 0..parse_to_TableTime(classes[i].clone()).len() {
                return_value.push(parse_to_TableTime(classes[i].clone())[j].clone());
            }
        }
        for i in 0..chosen_classes.len() {
            chosen_ids.push(chosen_classes[i].clone().class_id);
        }
        Ok(return_value)
    } else {
        Err(format!("Failed class(es): {:?}", failed_classes))
    }
}

fn parse_day(day: i32) -> String {
    match day {
        2 => "Mon",
        3 => "Tue",
        4 => "Wed",
        5 => "Thu",
        6 => "Fri",
        7 => "Sat",
        _ => "Sun",
    }
    .to_string()
}

fn parse_start_end(start: i32, end: i32) -> Vec<f32> {
    let mut result = Vec::new();
    let mut hh: f32 = 0.0;
    let mut mm: f32 = 0.0;
    let parse_start = start.to_string();
    if parse_start.len() < 4 {
        hh = parse_start[0..1].parse::<f32>().unwrap();
        mm = parse_start[1..3].parse::<f32>().unwrap() / 60.0;
    } else {
        hh = parse_start[0..2].parse::<f32>().unwrap();
        mm = parse_start[2..4].parse::<f32>().unwrap() / 60.0;
    }
    let time_start = hh + mm;
    let parse_end = end.to_string();
    if parse_end.len() < 4 {
        hh = parse_end[0..1].parse::<f32>().unwrap();
        mm = parse_end[1..3].parse::<f32>().unwrap() / 60.0;
    } else {
        hh = parse_end[0..2].parse::<f32>().unwrap();
        mm = parse_end[2..4].parse::<f32>().unwrap() / 60.0;
    }
    let time_end = hh + mm;
    result.push(time_start);
    result.push(time_end);
    result
}

fn parse_to_TableTime(data: ResultData) -> Vec<TableTime> {
    let mut result = Vec::new();
    let CELL_OFFSET = 60.0;
    let HOUR_OFFSET = 6.0;
    for i in 0..data.data.len() {
        let parsed_time = parse_start_end(data.data[i].start, data.data[i].end);
        let tuple = TableTime {
            day: parse_day(data.data[i].day),
            render_top: (parsed_time[0] - HOUR_OFFSET) * CELL_OFFSET,
            render_height: (parsed_time[1] - parsed_time[0]) * CELL_OFFSET,
            subject_name: data.class_title.clone(),
            subject_id: data.subject_id.clone(),
            class_id: data.class_id.clone(),
            class_type: data.class_type.clone(),
            display_date: format!("{}-{}", data.data[i].start, data.data[i].end),
            room: data.location.clone(),
        };
        result.push(tuple);
    }
    result
}

#[tauri::command]
fn remove_chosen_class(
    state: tauri::State<'_, SharedState>,
    class_id: String,
) -> Result<String, String> {
    let mut chosen_classes = state.chosen_classes.lock().unwrap();
    let mut available_positions = state.available_positions.lock().unwrap();
    let mut data = ResultData {
        subject_id: "".to_string(),
        class_id: "".to_string(),
        included_id: "".to_string(),
        class_title: "".to_string(),
        credit: "".to_string(),
        note: "".to_string(),
        data: Vec::new(),
        lab: "".to_string(),
        location: "".to_string(),
        class_type: "".to_string(),
        validity: "".to_string(),
    };
    let mut class_removed = false;
    for i in 0..chosen_classes.len() {
        if chosen_classes[i].class_id == class_id {
            class_removed = true;
            data = chosen_classes[i].clone();
            chosen_classes.remove(i);
            break;
        }
    }
    if !class_removed {
        return Err(format!("Class not found"));
    }
    let mut position_removed = false;
    for i in 0..data.data.len() {
        for j in 0..available_positions[data.data[i].day as usize - 2].len() {
            let mut current_length = available_positions[data.data[i].day as usize - 2].len();
            if available_positions[data.data[i].day as usize - 2][j] == data.data[i].start {
                available_positions[data.data[i].day as usize - 2].remove(j);
                available_positions[data.data[i].day as usize - 2].remove(j);
                position_removed = true;
                current_length -= 2;
            }
            if j >= current_length {
                break;
            }
        }
        if !position_removed {
            return Err(format!("Position not found"));
        }
    }

    Ok(format!("Class removed"))
}
#[tauri::command]
fn get_chosen_classes(state: tauri::State<'_, SharedState>) -> Vec<ResultData> {
    let chosen_classes = state.chosen_classes.lock().unwrap();
    chosen_classes.clone()
}

fn main() {
    let initial_positions: Vec<Vec<i32>> = vec![Vec::new(); 6];
    let chosen_classes: Vec<ResultData> = Vec::new();
    tauri::Builder::default()
        .manage(SharedState {
            excel: Mutex::new(None),
            available_positions: Mutex::new(initial_positions),
            chosen_classes: Mutex::new(chosen_classes),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            parse_file,
            sort_by_class_id,
            add_chosen_class,
            remove_chosen_class,
            get_chosen_classes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
