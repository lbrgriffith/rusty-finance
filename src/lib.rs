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

/// Validates that a number is within safe calculation range
pub fn validate_calculation_range(value: f64, name: &str) -> FinanceResult<()> {
    validate_finite(value, name)?;
    
    // Check for values that could cause overflow in financial calculations
    if value.abs() > 1e15 {
        return Err(FinanceError::InvalidInput(
            format!("{} value too large for safe calculation: {}", name, value)
        ));
    }
    
    Ok(())
}

/// Safe multiplication with overflow check
pub fn safe_multiply(a: f64, b: f64) -> FinanceResult<f64> {
    validate_finite(a, "first operand")?;
    validate_finite(b, "second operand")?;
    
    let result = a * b;
    
    if !result.is_finite() {
        return Err(FinanceError::Overflow);
    }
    
    Ok(result)
}

/// Safe division with zero and overflow check
pub fn safe_divide(a: f64, b: f64) -> FinanceResult<f64> {
    validate_finite(a, "dividend")?;
    validate_finite(b, "divisor")?;
    
    if b == 0.0 {
        return Err(FinanceError::DivisionByZero);
    }
    
    let result = a / b;
    
    if !result.is_finite() {
        return Err(FinanceError::Overflow);
    }
    
    Ok(result)
}

/// Safe power operation with overflow check
pub fn safe_power(base: f64, exponent: f64) -> FinanceResult<f64> {
    validate_finite(base, "base")?;
    validate_finite(exponent, "exponent")?;
    
    // Check for potentially dangerous exponential operations
    if exponent.abs() > 100.0 {
        return Err(FinanceError::InvalidInput(
            format!("Exponent too large for safe calculation: {}", exponent)
        ));
    }
    
    let result = base.powf(exponent);
    
    if !result.is_finite() {
        return Err(FinanceError::Overflow);
    }
    
    Ok(result)
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

    #[test]
    fn test_validate_calculation_range() {
        assert!(validate_calculation_range(1000.0, "test").is_ok());
        assert!(validate_calculation_range(1e14, "test").is_ok());
        assert!(validate_calculation_range(1e16, "test").is_err());
        assert!(validate_calculation_range(f64::NAN, "test").is_err());
    }

    #[test]
    fn test_safe_multiply() {
        assert!(safe_multiply(1000.0, 1000.0).is_ok());
        assert!(safe_multiply(1e308, 2.0).is_err()); // Should overflow
        assert!(safe_multiply(f64::NAN, 1.0).is_err());
    }

    #[test]
    fn test_safe_divide() {
        assert!(safe_divide(1000.0, 10.0).is_ok());
        assert!(safe_divide(1000.0, 0.0).is_err()); // Division by zero
        assert!(safe_divide(f64::NAN, 1.0).is_err());
    }

    #[test]
    fn test_safe_power() {
        assert!(safe_power(2.0, 10.0).is_ok());
        assert!(safe_power(2.0, 1000.0).is_err()); // Exponent too large
        assert!(safe_power(1e200, 2.0).is_err()); // Would overflow
        assert!(safe_power(f64::NAN, 1.0).is_err());
    }
}