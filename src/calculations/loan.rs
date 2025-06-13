//! Loan and mortgage calculation functions

use crate::{FinanceError, FinanceResult, validate_positive, validate_non_negative};
use chrono::{Local, Months, NaiveDate};

/// Represents a single payment in an amortization schedule
#[derive(Debug, Clone)]
pub struct AmortizationPayment {
    pub month: u32,
    pub principal_payment: f64,
    pub interest_payment: f64,
    pub remaining_balance: f64,
}

/// Calculates monthly loan payment using the standard loan payment formula
/// 
/// Formula: M = P * [r(1+r)^n] / [(1+r)^n - 1]
/// Where:
/// - M = Monthly payment
/// - P = Principal amount
/// - r = Monthly interest rate
/// - n = Total number of payments
/// 
/// # Arguments
/// * `principal` - The loan amount
/// * `annual_interest_rate` - The annual interest rate as a percentage
/// * `loan_term_years` - The loan term in years
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_loan_payment;
/// 
/// let payment = calculate_loan_payment(100000.0, 5.0, 30.0).unwrap();
/// assert!((payment - 536.82).abs() < 0.01);
/// ```
pub fn calculate_loan_payment(
    principal: f64,
    annual_interest_rate: f64,
    loan_term_years: f64
) -> FinanceResult<f64> {
    validate_positive(principal, "Principal")?;
    validate_non_negative(annual_interest_rate, "Annual interest rate")?;
    validate_positive(loan_term_years, "Loan term")?;
    
    let monthly_rate = annual_interest_rate / 100.0 / 12.0;
    let num_payments = loan_term_years * 12.0;
    
    if monthly_rate == 0.0 {
        return Ok(principal / num_payments);
    }
    
    let monthly_payment = (principal * monthly_rate) / 
        (1.0 - (1.0 + monthly_rate).powf(-num_payments));
    
    Ok(monthly_payment)
}

/// Calculates mortgage payment details including total interest and payoff date
/// 
/// # Arguments
/// * `loan_amount` - The mortgage amount
/// * `annual_interest_rate` - The annual interest rate as a percentage
/// * `term_years` - The mortgage term in years
/// 
/// # Returns
/// * A tuple containing (monthly_payment, total_interest, payoff_date)
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_mortgage_details;
/// 
/// let (payment, total_interest, _) = calculate_mortgage_details(200000.0, 4.5, 30).unwrap();
/// assert!((payment - 1013.37).abs() < 0.01);
/// ```
pub fn calculate_mortgage_details(
    loan_amount: f64,
    annual_interest_rate: f64,
    term_years: i32
) -> FinanceResult<(f64, f64, NaiveDate)> {
    validate_positive(loan_amount, "Loan amount")?;
    validate_positive(annual_interest_rate, "Annual interest rate")?;
    
    if term_years <= 0 {
        return Err(FinanceError::InvalidInput("Term must be positive".into()));
    }
    
    let monthly_payment = calculate_loan_payment(loan_amount, annual_interest_rate, term_years as f64)?;
    let total_payments = term_years * 12;
    let total_amount_paid = monthly_payment * total_payments as f64;
    let total_interest = total_amount_paid - loan_amount;
    
    // Calculate payoff date
    let current_date = Local::now().naive_local().date();
    let payoff_date = current_date + Months::new(total_payments as u32);
    
    Ok((monthly_payment, total_interest, payoff_date))
}

/// Generates a complete amortization schedule
/// 
/// # Arguments
/// * `loan_amount` - The initial loan amount
/// * `annual_interest_rate` - The annual interest rate as a percentage
/// * `term_years` - The loan term in years
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::generate_amortization_schedule;
/// 
/// let schedule = generate_amortization_schedule(100000.0, 5.0, 30).unwrap();
/// assert_eq!(schedule.len(), 360); // 30 years * 12 months
/// ```
pub fn generate_amortization_schedule(
    loan_amount: f64,
    annual_interest_rate: f64,
    term_years: i32
) -> FinanceResult<Vec<AmortizationPayment>> {
    validate_positive(loan_amount, "Loan amount")?;
    validate_positive(annual_interest_rate, "Annual interest rate")?;
    
    if term_years <= 0 {
        return Err(FinanceError::InvalidInput("Term must be positive".into()));
    }
    
    let monthly_payment = calculate_loan_payment(loan_amount, annual_interest_rate, term_years as f64)?;
    let monthly_rate = annual_interest_rate / 100.0 / 12.0;
    let total_payments = term_years * 12;
    
    let mut schedule = Vec::with_capacity(total_payments as usize);
    let mut remaining_balance = loan_amount;
    
    for month in 1..=total_payments {
        let interest_payment = remaining_balance * monthly_rate;
        let principal_payment = monthly_payment - interest_payment;
        remaining_balance -= principal_payment;
        
        // Handle final payment rounding
        if month == total_payments {
            remaining_balance = 0.0;
        }
        
        schedule.push(AmortizationPayment {
            month: month as u32,
            principal_payment,
            interest_payment,
            remaining_balance,
        });
    }
    
    Ok(schedule)
}

/// Calculates break-even point in units
/// 
/// Formula: Break-even units = Fixed Costs / (Price per Unit - Variable Cost per Unit)
/// 
/// # Arguments
/// * `fixed_costs` - Total fixed costs
/// * `variable_cost_per_unit` - Variable cost per unit
/// * `price_per_unit` - Selling price per unit
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_break_even_units;
/// 
/// let units = calculate_break_even_units(1000.0, 10.0, 20.0).unwrap();
/// assert_eq!(units, 100.0);
/// ```
pub fn calculate_break_even_units(
    fixed_costs: f64,
    variable_cost_per_unit: f64,
    price_per_unit: f64
) -> FinanceResult<f64> {
    validate_positive(fixed_costs, "Fixed costs")?;
    validate_positive(variable_cost_per_unit, "Variable cost per unit")?;
    validate_positive(price_per_unit, "Price per unit")?;
    
    if price_per_unit <= variable_cost_per_unit {
        return Err(FinanceError::InvalidInput(
            "Price per unit must be greater than variable cost per unit".into()
        ));
    }
    
    let contribution_margin = price_per_unit - variable_cost_per_unit;
    Ok(fixed_costs / contribution_margin)
}

/// Calculates break-even analysis (revenue and units)
/// 
/// # Arguments
/// * `fixed_costs` - Total fixed costs
/// * `variable_cost_per_unit` - Variable cost per unit
/// * `price_per_unit` - Selling price per unit
/// 
/// # Returns
/// * A tuple containing (break_even_units, break_even_revenue)
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_break_even_analysis;
/// 
/// let (units, revenue) = calculate_break_even_analysis(5000.0, 10.0, 20.0).unwrap();
/// assert_eq!(units, 500.0);
/// assert_eq!(revenue, 10000.0);
/// ```
pub fn calculate_break_even_analysis(
    fixed_costs: f64,
    variable_cost_per_unit: f64,
    price_per_unit: f64
) -> FinanceResult<(f64, f64)> {
    let break_even_units = calculate_break_even_units(fixed_costs, variable_cost_per_unit, price_per_unit)?;
    let break_even_revenue = break_even_units * price_per_unit;
    
    Ok((break_even_units, break_even_revenue))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loan_payment() {
        let payment = calculate_loan_payment(100000.0, 5.0, 30.0).unwrap();
        assert!((payment - 536.82).abs() < 0.01);
    }

    #[test]
    fn test_loan_payment_zero_interest() {
        let payment = calculate_loan_payment(120000.0, 0.0, 10.0).unwrap();
        assert_eq!(payment, 1000.0); // 120000 / (10 * 12)
    }

    #[test]
    fn test_loan_payment_invalid_inputs() {
        assert!(calculate_loan_payment(0.0, 5.0, 30.0).is_err());
        assert!(calculate_loan_payment(100000.0, -5.0, 30.0).is_err());
        assert!(calculate_loan_payment(100000.0, 5.0, 0.0).is_err());
    }

    #[test]
    fn test_mortgage_details() {
        let (payment, total_interest, _payoff_date) = calculate_mortgage_details(200000.0, 4.5, 30).unwrap();
        assert!((payment - 1013.37).abs() < 0.01);
        assert!(total_interest > 0.0);
    }

    #[test]
    fn test_amortization_schedule() {
        let schedule = generate_amortization_schedule(100000.0, 5.0, 30).unwrap();
        assert_eq!(schedule.len(), 360);
        
        // First payment should have more interest than principal
        assert!(schedule[0].interest_payment > schedule[0].principal_payment);
        
        // Last payment should have more principal than interest
        let last_payment = &schedule[schedule.len() - 1];
        assert!(last_payment.principal_payment > last_payment.interest_payment);
        assert!((last_payment.remaining_balance).abs() < 0.01);
    }

    #[test]
    fn test_break_even_units() {
        let units = calculate_break_even_units(1000.0, 10.0, 20.0).unwrap();
        assert_eq!(units, 100.0);
    }

    #[test]
    fn test_break_even_units_invalid_margin() {
        // Price equal to variable cost
        assert!(calculate_break_even_units(1000.0, 20.0, 20.0).is_err());
        
        // Price less than variable cost
        assert!(calculate_break_even_units(1000.0, 25.0, 20.0).is_err());
    }

    #[test]
    fn test_break_even_analysis() {
        let (units, revenue) = calculate_break_even_analysis(5000.0, 10.0, 20.0).unwrap();
        assert_eq!(units, 500.0);
        assert_eq!(revenue, 10000.0);
    }
}