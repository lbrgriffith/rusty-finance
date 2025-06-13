//! Rusty Finance Library
//! 
//! A comprehensive financial calculation library providing functions for:
//! - Interest calculations (simple and compound)
//! - Present and future value calculations
//! - Investment analysis (NPV, IRR, DCF)
//! - Loan and mortgage calculations
//! - Statistical calculations (mean, median, mode, variance)
//! - Financial ratios and analysis

use anyhow::Result;
use rust_decimal::prelude::*;
use thiserror::Error;

pub mod calculations;
pub mod cli;
pub mod display;

/// Custom finance calculation errors
#[derive(Error, Debug)]
pub enum FinanceError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Division by zero")]
    DivisionByZero,
    
    #[error("Calculation overflow")]
    Overflow,
    
    #[error("Convergence failed")]
    ConvergenceFailed,
}

/// Result type alias for finance calculations
pub type FinanceResult<T> = Result<T, FinanceError>;

/// Validates that a number is finite and positive
pub fn validate_positive(value: f64, name: &str) -> FinanceResult<()> {
    if !value.is_finite() {
        return Err(FinanceError::InvalidInput(format!("{} must be a valid number: {}", name, value)));
    }
    if value <= 0.0 {
        return Err(FinanceError::InvalidInput(format!("{} must be positive: {}", name, value)));
    }
    Ok(())
}

/// Validates that a number is finite and non-negative
pub fn validate_non_negative(value: f64, name: &str) -> FinanceResult<()> {
    if !value.is_finite() {
        return Err(FinanceError::InvalidInput(format!("{} must be a valid number: {}", name, value)));
    }
    if value < 0.0 {
        return Err(FinanceError::InvalidInput(format!("{} must be non-negative: {}", name, value)));
    }
    Ok(())
}

/// Validates that a number is finite
pub fn validate_finite(value: f64, name: &str) -> FinanceResult<()> {
    if !value.is_finite() {
        return Err(FinanceError::InvalidInput(format!("{} must be a valid number: {}", name, value)));
    }
    Ok(())
}

/// Converts f64 to Decimal with error handling
pub fn to_decimal(value: f64, name: &str) -> FinanceResult<Decimal> {
    Decimal::from_f64(value)
        .ok_or_else(|| FinanceError::InvalidInput(format!("Invalid {} value: {}", name, value)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_positive() {
        assert!(validate_positive(1.0, "test").is_ok());
        assert!(validate_positive(0.1, "test").is_ok());
        assert!(validate_positive(0.0, "test").is_err());
        assert!(validate_positive(-1.0, "test").is_err());
        assert!(validate_positive(f64::NAN, "test").is_err());
        assert!(validate_positive(f64::INFINITY, "test").is_err());
    }

    #[test]
    fn test_validate_non_negative() {
        assert!(validate_non_negative(1.0, "test").is_ok());
        assert!(validate_non_negative(0.0, "test").is_ok());
        assert!(validate_non_negative(-1.0, "test").is_err());
        assert!(validate_non_negative(f64::NAN, "test").is_err());
    }

    #[test]
    fn test_validate_finite() {
        assert!(validate_finite(1.0, "test").is_ok());
        assert!(validate_finite(0.0, "test").is_ok());
        assert!(validate_finite(-1.0, "test").is_ok());
        assert!(validate_finite(f64::NAN, "test").is_err());
        assert!(validate_finite(f64::INFINITY, "test").is_err());
    }

    #[test]
    fn test_to_decimal() {
        assert!(to_decimal(1.0, "test").is_ok());
        assert!(to_decimal(0.0, "test").is_ok());
        assert!(to_decimal(-1.0, "test").is_ok());
        // NaN should fail
        assert!(to_decimal(f64::NAN, "test").is_err());
    }
}