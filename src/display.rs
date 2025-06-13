//! Display and formatting utilities for financial data

use comfy_table::{Cell, Color, ContentArrangement, Table};
use rust_decimal::prelude::*;
use log::warn;

/// Creates a styled table with the given headers
/// 
/// # Arguments
/// * `headers` - Vector of header strings
/// 
/// # Returns
/// * A configured `Table` instance ready for data rows
pub fn create_table(headers: Vec<&str>) -> Table {
    let mut table = Table::new();
    
    // Set up simple table styling for better alignment
    table
        .set_header(headers)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .load_preset(comfy_table::presets::UTF8_BORDERS_ONLY);
    
    table
}

/// Formats a number as currency with colored output
/// 
/// # Arguments
/// * `number` - The number to format as currency
/// 
/// # Returns
/// * A formatted currency string with appropriate coloring
/// 
/// # Examples
/// ```
/// use rusty_finance::display::format_currency;
/// 
/// let formatted = format_currency(1234.56);
/// // Returns something like "$1,234.56" in green (for positive) or red (for negative)
/// ```
pub fn format_currency(number: f64) -> String {
    format_currency_plain(number)
}

/// Formats a number as currency without colors for better table alignment
/// 
/// # Arguments
/// * `number` - The number to format as currency
/// 
/// # Returns
/// * A formatted currency string without color codes
pub fn format_currency_plain(number: f64) -> String {
    // Convert the f64 to a Decimal for accurate handling
    let decimal = Decimal::from_f64(number)
        .unwrap_or_else(|| {
            warn!("Invalid number for currency formatting: {}", number);
            Decimal::ZERO
        });
    
    // Round to 2 decimal places
    let rounded = decimal.round_dp(2);
    
    // Use the Decimal formatting functionality directly
    let formatted = rounded.to_string();
    
    // Split into whole and decimal parts
    let parts: Vec<&str> = formatted.split('.').collect();
    let whole_part = parts[0];
    let decimal_part = parts.get(1).map_or("00", |&s| {
        if s.len() >= 2 { &s[0..2] } else { s }
    });
    
    // Add commas to the whole part
    let whole_with_commas = add_thousands_separators(whole_part);
    
    // Format as currency without colors
    format!("${}.{}", whole_with_commas, decimal_part)
}

/// Formats a percentage with appropriate coloring
/// 
/// # Arguments
/// * `value` - The decimal value to format as percentage (0.05 = 5%)
/// * `decimal_places` - Number of decimal places to show
/// 
/// # Returns
/// * A formatted percentage string with appropriate coloring
pub fn format_percentage(value: f64, decimal_places: usize) -> String {
    format_percentage_plain(value, decimal_places)
}

/// Formats a percentage without colors for better table alignment
/// 
/// # Arguments
/// * `value` - The decimal value to format as percentage (0.05 = 5%)
/// * `decimal_places` - Number of decimal places to show
/// 
/// # Returns
/// * A formatted percentage string without color codes
pub fn format_percentage_plain(value: f64, decimal_places: usize) -> String {
    let percentage = value * 100.0;
    format!("{:.1$}%", percentage, decimal_places)
}

/// Formats a number with commas as thousands separators
/// 
/// # Arguments
/// * `number` - The number to format
/// * `decimal_places` - Number of decimal places to show
/// 
/// # Returns
/// * A formatted number string with commas
pub fn format_number(number: f64, decimal_places: usize) -> String {
    let formatted = format!("{:.1$}", number, decimal_places);
    let parts: Vec<&str> = formatted.split('.').collect();
    let whole_part = parts[0];
    let decimal_part = parts.get(1);
    
    let whole_with_commas = add_thousands_separators(whole_part);
    
    if let Some(decimals) = decimal_part {
        format!("{}.{}", whole_with_commas, decimals)
    } else {
        whole_with_commas
    }
}

/// Adds thousands separators (commas) to a number string
/// 
/// # Arguments
/// * `number_str` - The number string to add separators to
/// 
/// # Returns
/// * The number string with comma separators
fn add_thousands_separators(number_str: &str) -> String {
    let is_negative = number_str.starts_with('-');
    let digits = if is_negative { &number_str[1..] } else { number_str };
    
    let mut result = String::new();
    let len = digits.len();
    
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    
    if is_negative {
        format!("-{}", result)
    } else {
        result
    }
}

/// Creates a summary table for financial calculations
/// 
/// # Arguments
/// * `title` - The title of the summary
/// * `items` - Vector of (label, value) pairs
/// 
/// # Returns
/// * A formatted table showing the summary
pub fn create_summary_table(title: &str, items: Vec<(&str, String)>) -> Table {
    let mut table = create_table(vec![title, "Value"]);
    
    for (label, value) in items {
        table.add_row(vec![
            label.to_string(),
            value,
        ]);
    }
    
    table
}

/// Creates a colored cell based on a numeric value
/// 
/// # Arguments
/// * `value` - The text content of the cell
/// * `numeric_value` - The numeric value to determine color
/// 
/// # Returns
/// * A `Cell` with appropriate color formatting
pub fn create_colored_cell(value: &str, numeric_value: f64) -> Cell {
    if numeric_value > 0.0 {
        Cell::new(value).fg(Color::Green)
    } else if numeric_value < 0.0 {
        Cell::new(value).fg(Color::Red)
    } else {
        Cell::new(value).fg(Color::Yellow)
    }
}

/// Creates a progress indicator for amortization schedules
/// 
/// # Arguments
/// * `current_payment` - Current payment number
/// * `total_payments` - Total number of payments
/// 
/// # Returns
/// * A formatted progress string
pub fn format_progress(current_payment: u32, total_payments: u32) -> String {
    let percentage = (current_payment as f64 / total_payments as f64) * 100.0;
    format!("{}/{} ({:.1}%)", current_payment, total_payments, percentage)
}

/// Formats years in a human-readable way
/// 
/// # Arguments
/// * `years` - Number of years as a float
/// 
/// # Returns
/// * A human-readable year string
pub fn format_years(years: f64) -> String {
    if years == 1.0 {
        "1 year".to_string()
    } else if years.fract() == 0.0 {
        format!("{} years", years as i32)
    } else {
        format!("{:.1} years", years)
    }
}

/// Formats a decimal rate as a percentage
/// 
/// # Arguments
/// * `rate` - The decimal rate (e.g., 0.05 for 5%)
/// 
/// # Returns
/// * A formatted percentage string
pub fn format_rate_as_percentage(rate: f64) -> String {
    format!("{:.2}%", rate * 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_currency_positive() {
        let result = format_currency(1234.56);
        // We can't test the actual color codes, but we can test the format
        assert!(result.contains("$1,234.56"));
    }

    #[test]
    fn test_format_currency_negative() {
        let result = format_currency(-1234.56);
        assert!(result.contains("$-1,234.56"));
    }

    #[test]
    fn test_format_currency_zero() {
        let result = format_currency(0.0);
        assert!(result.contains("$0.00"));
    }

    #[test]
    fn test_add_thousands_separators() {
        assert_eq!(add_thousands_separators("1234567"), "1,234,567");
        assert_eq!(add_thousands_separators("123"), "123");
        assert_eq!(add_thousands_separators("-1234567"), "-1,234,567");
    }

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(1234.567, 2), "1,234.57");
        assert_eq!(format_number(123.0, 0), "123");
    }

    #[test]
    fn test_format_years() {
        assert_eq!(format_years(1.0), "1 year");
        assert_eq!(format_years(5.0), "5 years");
        assert_eq!(format_years(2.5), "2.5 years");
    }

    #[test]
    fn test_format_rate_as_percentage() {
        assert_eq!(format_rate_as_percentage(0.05), "5.00%");
        assert_eq!(format_rate_as_percentage(0.1234), "12.34%");
    }

    #[test]
    fn test_format_progress() {
        assert_eq!(format_progress(25, 100), "25/100 (25.0%)");
        assert_eq!(format_progress(1, 3), "1/3 (33.3%)");
    }
}