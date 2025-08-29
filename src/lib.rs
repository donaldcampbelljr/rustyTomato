use wasm_bindgen::prelude::*;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

// Import the console.log function from the browser
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Define a macro for easier logging
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Re-export your enums for WASM
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum WasmTimeUnits {
    Minutes,
    Seconds,
    Days,
    Months,
}

// WASM-compatible timer state
#[wasm_bindgen]
pub struct WasmPomodoroTimer {
    duration_seconds: usize,
    start_time: Option<DateTime<Utc>>,
}

#[wasm_bindgen]
impl WasmPomodoroTimer {
    #[wasm_bindgen(constructor)]
    pub fn new(time_number: usize, time_units: WasmTimeUnits) -> WasmPomodoroTimer {
        console_error_panic_hook::set_once();
        
        let duration_seconds = match time_units {
            WasmTimeUnits::Seconds => time_number,
            WasmTimeUnits::Minutes => time_number * 60,
            WasmTimeUnits::Days => time_number * 24 * 60 * 60,
            WasmTimeUnits::Months => time_number * 30 * 24 * 60 * 60,
        };

        WasmPomodoroTimer {
            duration_seconds,
            start_time: None,
        }
    }

    #[wasm_bindgen]
    pub fn start(&mut self) {
        self.start_time = Some(Utc::now());
        console_log!("Timer started for {} seconds", self.duration_seconds);
    }

    #[wasm_bindgen]
    pub fn get_remaining_time(&self) -> Option<usize> {
        if let Some(start) = self.start_time {
            let elapsed = Utc::now().signed_duration_since(start).num_seconds() as usize;
            if elapsed < self.duration_seconds {
                Some(self.duration_seconds - elapsed)
            } else {
                Some(0)
            }
        } else {
            Some(self.duration_seconds)
        }
    }

    #[wasm_bindgen]
    pub fn is_finished(&self) -> bool {
        if let Some(remaining) = self.get_remaining_time() {
            remaining == 0
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub fn get_session_data(&self) -> String {
        if let Some(start) = self.start_time {
            let end = Utc::now();
            let duration = end.signed_duration_since(start);
            format!("{}  {}", start, duration)
        } else {
            String::new()
        }
    }
}

// Export utility functions
#[wasm_bindgen]
pub fn create_wasm_time(time_numbers: usize, time_units: WasmTimeUnits) -> usize {
    match time_units {
        WasmTimeUnits::Seconds => time_numbers,
        WasmTimeUnits::Minutes => time_numbers * 60,
        WasmTimeUnits::Days => time_numbers * 24 * 60 * 60,
        WasmTimeUnits::Months => time_numbers * 30 * 24 * 60 * 60,
    }
}