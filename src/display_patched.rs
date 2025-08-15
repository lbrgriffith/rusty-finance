//! Display and formatting utilities for financial data
//!
//! This version removes *manual* space padding from data rows and instead
//! relies on `comfy_table`'s dynamic layout and per-cell alignment so rows
//! expand/contract automatically based on actual contents.

use comfy_table::{
    presets::UTF8_FULL,
    modifiers::UTF8_ROUND_CORNERS,
    Cell, CellAlignment, Color, ContentArrangement, Table,
};
use rust_decimal::prelude::*;
use log::warn;

/// Creates a styled table with the given headers.
///
/// The table uses `ContentArrangement::Dynamic` so that column widths
/// are computed from the real content (no manual padding required).
pub fn create_table(headers: &[&str]) -> Table {
    let mut table = Table::new();

    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);

    // Build header row without manual padding. Use centering for headers by default.
    let header_cells = headers.iter().map(|h| {
        let mut c = Cell::new(*h);
        c.set_alignment(CellAlignment::Center);
        c
    });
    table.set_header(header_cells);

    table
}

/// Add a row of values with per-cell alignment (no manual spacing needed).
///
/// # Example
/// ```ignore
/// add_row(&mut table, &[("Account", Left), ("1234.56", Right), ("USD", Center)]);
/// ```
pub fn add_row(table: &mut Table, cols: &[(&str, CellAlignment)]) {
    let mut cells = Vec::with_capacity(cols.len());
    for (text, align) in cols.iter() {
        let mut cell = Cell::new(*text);
        cell.set_alignment(*align);
        cells.push(cell);
    }
    table.add_row(cells);
}

/// Convenience: add a row from `String`s (already formatted text), with alignment.
pub fn add_row_string(table: &mut Table, cols: &[(String, CellAlignment)]) {
    let mut cells = Vec::with_capacity(cols.len());
    for (text, align) in cols.iter() {
        let mut cell = Cell::new(text);
        cell.set_alignment(*align);
        cells.push(cell);
    }
    table.add_row(cells);
}

/// Format a Decimal as currency with 2 dp, without adding any padding.
pub fn fmt_money(d: Decimal) -> String {
    // Round to 2 dp; do not right-pad or left-pad
    let v = d.round_dp(2);
    format!("{}", v)
}

/// Format a Decimal with a given number of fractional digits (no padding).
pub fn fmt_decimal(d: Decimal, scale: u32) -> String {
    let v = d.round_dp(scale);
    format!("{}", v)
}

/// Format an f64 as a percentage with 2 dp (no padding).
pub fn fmt_percent(p: f64) -> String {
    format!("{:.2}%", p * 100.0)
}

/// Format a rate (e.g., 0.0525 -> "5.25%") with flexible precision.
pub fn fmt_rate_as_percentage(rate: f64, dp: usize) -> String {
    format!("{:.dp$}%", rate * 100.0, dp = dp)
}

/// Helper to safely build a table and return it as a String ready for printing.
/// (This ensures the final width is computed after all rows are in.)
pub fn render_table<F>(headers: &[&str], build_rows: F) -> String
where
    F: FnOnce(&mut Table),
{
    let mut table = create_table(headers);
    build_rows(&mut table);
    table.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dynamic_rows_expand() {
        // When adding vastly different length values, the table should widen automatically
        let out = render_table(&["Col A", "Amount", "Pct"], |t| {
            add_row(t, &[("Short", CellAlignment::Left),
                         ("12.34", CellAlignment::Right),
                         ("1.0%", CellAlignment::Right)]);

            add_row(t, &[("A very long description that should widen the column",
                          CellAlignment::Left),
                         ("123,456,789.01", CellAlignment::Right),
                         ("99.99%", CellAlignment::Right)]);
        });

        // Smoke check: both strings should be present (no truncation, no manual padding)
        assert!(out.contains("Short"));
        assert!(out.contains("A very long description"));
        assert!(out.contains("123,456,789.01") || out.contains("123456789.01"));
    }

    #[test]
    fn formats_are_unpadded() {
        let money = fmt_money(Decimal::new(12345, 2)); // 123.45
        assert_eq!(money, "123.45");

        let pct = fmt_percent(0.1234);
        assert_eq!(pct, "12.34%");

        let d = fmt_decimal(Decimal::new(314159, 5), 3); // 3.14159 -> 3.142
        assert_eq!(d, "3.142");
    }

    #[test]
    fn rate_percentage_custom_dp() {
        assert_eq!(fmt_rate_as_percentage(0.05, 2), "5.00%");
        assert_eq!(fmt_rate_as_percentage(0.1234, 1), "12.3%");
    }
}
