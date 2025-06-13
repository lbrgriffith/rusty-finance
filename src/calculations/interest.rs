//! Interest calculation functions

use crate::{FinanceError, FinanceResult, validate_positive, validate_non_negative, validate_calculation_range, safe_multiply, safe_power, safe_divide};

/// Calculates simple interest
/// 
/// Formula: Interest = Principal × Rate × Time
/// 
/// # Arguments
/// * `principal` - The initial amount of money
/// * `rate` - The interest rate (as a decimal, e.g., 0.05 for 5%)
/// * `time` - The time period in years
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_simple_interest;
/// 
/// let interest = calculate_simple_interest(1000.0, 0.05, 2.0).unwrap();
/// assert_eq!(interest, 100.0);
/// ```
pub fn calculate_simple_interest(principal: f64, rate: f64, time: f64) -> FinanceResult<f64> {
    validate_positive(principal, "Principal")?;
    validate_non_negative(rate, "Interest rate")?;
    validate_non_negative(time, "Time")?;
    validate_calculation_range(principal, "Principal")?;
    
    let temp = safe_multiply(principal, rate)?;
    safe_multiply(temp, time)
}

/// Calculates compound interest amount for a given year
/// 
/// Formula: A = P(1 + r/n)^(nt)
/// 
/// # Arguments
/// * `principal` - The initial amount of money
/// * `rate` - The annual interest rate (as a decimal)
/// * `compound_frequency` - Number of times interest is compounded per year
/// * `years` - Number of years
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_compound_interest;
/// 
/// let amount = calculate_compound_interest(1000.0, 0.05, 12, 1).unwrap();
/// assert!((amount - 1051.16).abs() < 0.01);
/// ```
pub fn calculate_compound_interest(
    principal: f64, 
    rate: f64, 
    compound_frequency: i32, 
    years: i32
) -> FinanceResult<f64> {
    validate_positive(principal, "Principal")?;
    validate_non_negative(rate, "Interest rate")?;
    validate_calculation_range(principal, "Principal")?;
    
    if compound_frequency <= 0 {
        return Err(FinanceError::InvalidInput("Compound frequency must be positive".into()));
    }
    
    if years < 0 {
        return Err(FinanceError::InvalidInput("Years must be non-negative".into()));
    }
    
    let rate_per_period = rate / compound_frequency as f64;
    let total_periods = compound_frequency * years;
    
    let base = 1.0 + rate_per_period;
    let exponent = total_periods as f64;
    let power_result = safe_power(base, exponent)?;
    safe_multiply(principal, power_result)
}

/// Calculates the present value of a future amount
/// 
/// Formula: PV = FV / (1 + r)^t
/// 
/// # Arguments
/// * `future_value` - The future value
/// * `rate` - The discount rate (as a decimal)
/// * `time` - Number of periods
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_present_value;
/// 
/// let pv = calculate_present_value(1000.0, 0.05, 2.0).unwrap();
/// assert!((pv - 907.03).abs() < 0.01);
/// ```
pub fn calculate_present_value(future_value: f64, rate: f64, time: f64) -> FinanceResult<f64> {
    validate_positive(future_value, "Future value")?;
    validate_non_negative(rate, "Discount rate")?;
    validate_non_negative(time, "Time")?;
    validate_calculation_range(future_value, "Future value")?;
    
    if rate >= 1.0 {
        return Err(FinanceError::InvalidInput("Discount rate should be less than 100%".into()));
    }
    
    let base = 1.0 + rate;
    let power_result = safe_power(base, time)?;
    safe_divide(future_value, power_result)
}

/// Calculates the future value of a present amount
/// 
/// Formula: FV = PV × (1 + r)^t
/// 
/// # Arguments
/// * `present_value` - The present value
/// * `rate` - The interest rate (as a decimal)
/// * `time` - Number of periods
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_future_value;
/// 
/// let fv = calculate_future_value(1000.0, 0.05, 2.0).unwrap();
/// assert!((fv - 1102.50).abs() < 0.01);
/// ```
pub fn calculate_future_value(present_value: f64, rate: f64, time: f64) -> FinanceResult<f64> {
    validate_positive(present_value, "Present value")?;
    validate_non_negative(rate, "Interest rate")?;
    validate_non_negative(time, "Time")?;
    validate_calculation_range(present_value, "Present value")?;
    
    let base = 1.0 + rate;
    let power_result = safe_power(base, time)?;
    safe_multiply(present_value, power_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_interest() {
        let result = calculate_simple_interest(1000.0, 0.05, 2.0).unwrap();
        assert_eq!(result, 100.0);
    }

    #[test]
    fn test_simple_interest_zero_rate() {
        let result = calculate_simple_interest(1000.0, 0.0, 2.0).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_simple_interest_invalid_principal() {
        assert!(calculate_simple_interest(-1000.0, 0.05, 2.0).is_err());
        assert!(calculate_simple_interest(0.0, 0.05, 2.0).is_err());
    }

    #[test]
    fn test_compound_interest() {
        let result = calculate_compound_interest(1000.0, 0.05, 12, 1).unwrap();
        assert!((result - 1051.16).abs() < 0.01);
    }

    #[test]
    fn test_compound_interest_invalid_frequency() {
        assert!(calculate_compound_interest(1000.0, 0.05, 0, 1).is_err());
        assert!(calculate_compound_interest(1000.0, 0.05, -1, 1).is_err());
    }

    #[test]
    fn test_present_value() {
        let result = calculate_present_value(1102.50, 0.05, 2.0).unwrap();
        assert!((result - 1000.0).abs() < 0.01);
    }

    #[test]
    fn test_present_value_high_rate() {
        assert!(calculate_present_value(1000.0, 1.5, 2.0).is_err());
    }

    #[test]
    fn test_future_value() {
        let result = calculate_future_value(1000.0, 0.05, 2.0).unwrap();
        assert!((result - 1102.50).abs() < 0.01);
    }

    #[test]
    fn test_future_value_zero_time() {
        let result = calculate_future_value(1000.0, 0.05, 0.0).unwrap();
        assert_eq!(result, 1000.0);
    }
}