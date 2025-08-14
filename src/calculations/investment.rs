//! Investment analysis functions

use crate::{FinanceError, FinanceResult, validate_positive, validate_non_negative};

/// Calculates Net Present Value (NPV)
/// 
/// # Arguments
/// * `initial_investment` - The initial cost of the investment
/// * `cash_flows` - Vector of future cash flows
/// * `discount_rate` - The discount rate (as a decimal)
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_npv;
/// 
/// let cash_flows = vec![100.0, 200.0, 300.0];
/// let npv = calculate_npv(500.0, &cash_flows, 0.10).unwrap();
/// assert!(npv < 0.0); // Investment not profitable at this rate
/// ```
pub fn calculate_npv(
    initial_investment: f64,
    cash_flows: &[f64],
    discount_rate: f64
) -> FinanceResult<f64> {
    validate_positive(initial_investment, "Initial investment")?;
    validate_non_negative(discount_rate, "Discount rate")?;
    
    if cash_flows.is_empty() {
        return Err(FinanceError::InvalidInput("Cash flows cannot be empty".into()));
    }
    
    let mut npv = -initial_investment;
    
    for (year, &cash_flow) in cash_flows.iter().enumerate() {
        let discounted_value = cash_flow / (1.0 + discount_rate).powf((year + 1) as f64);
        npv += discounted_value;
    }
    
    Ok(npv)
}

/// Calculates Return on Investment (ROI)
/// 
/// Formula: ROI = (Net Profit / Cost of Investment) × 100
/// 
/// # Arguments
/// * `net_profit` - The net profit from the investment
/// * `cost_of_investment` - The total cost of the investment
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_roi;
/// 
/// let roi = calculate_roi(500.0, 2000.0).unwrap();
/// assert_eq!(roi, 25.0);
/// ```
pub fn calculate_roi(net_profit: f64, cost_of_investment: f64) -> FinanceResult<f64> {
    validate_positive(cost_of_investment, "Cost of investment")?;
    
    // Net profit can be negative (loss)
    if !net_profit.is_finite() {
        return Err(FinanceError::InvalidInput("Net profit must be a valid number".into()));
    }
    
    Ok((net_profit / cost_of_investment) * 100.0)
}

/// Calculates Discounted Cash Flow (DCF) value
/// 
/// # Arguments
/// * `cash_flows` - Vector of future cash flows
/// * `discount_rate` - The discount rate (as a decimal)
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_dcf;
/// 
/// let cash_flows = vec![1000.0, 2000.0, 3000.0];
/// let dcf = calculate_dcf(&cash_flows, 0.10).unwrap();
/// assert!(dcf > 0.0);
/// ```
pub fn calculate_dcf(cash_flows: &[f64], discount_rate: f64) -> FinanceResult<f64> {
    validate_non_negative(discount_rate, "Discount rate")?;
    
    if cash_flows.is_empty() {
        return Err(FinanceError::InvalidInput("Cash flows cannot be empty".into()));
    }
    
    if discount_rate <= -1.0 {
        return Err(FinanceError::InvalidInput("Discount rate must be greater than -1".into()));
    }
    
    let mut dcf_value = 0.0;
    
    for (year, &cash_flow) in cash_flows.iter().enumerate() {
        if !cash_flow.is_finite() {
            return Err(FinanceError::InvalidInput(format!("Cash flow at year {} is invalid", year + 1)));
        }
        
        let present_value = cash_flow / (1.0 + discount_rate).powf((year + 1) as f64);
        dcf_value += present_value;
    }
    
    Ok(dcf_value)
}

/// Calculates payback period for an investment
/// 
/// # Arguments
/// * `initial_cost` - The initial cost of the investment
/// * `cash_flows` - Vector of future cash flows
/// 
/// # Returns
/// * The payback period in years, or None if the investment never pays back
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_payback_period;
/// 
/// let cash_flows = vec![100.0, 200.0, 300.0];
/// let payback = calculate_payback_period(300.0, &cash_flows).unwrap();
/// assert_eq!(payback, Some(3.0));
/// ```
pub fn calculate_payback_period(
    initial_cost: f64,
    cash_flows: &[f64]
) -> FinanceResult<Option<f64>> {
    validate_positive(initial_cost, "Initial cost")?;
    
    if cash_flows.is_empty() {
        return Err(FinanceError::InvalidInput("Cash flows cannot be empty".into()));
    }
    
    let mut cumulative_cash_flow = 0.0;
    
    for (year, &cash_flow) in cash_flows.iter().enumerate() {
        if !cash_flow.is_finite() {
            return Err(FinanceError::InvalidInput(format!("Cash flow at year {} is invalid", year + 1)));
        }
        
        cumulative_cash_flow += cash_flow;
        
        if cumulative_cash_flow >= initial_cost {
            // Calculate the exact payback period (with interpolation)
            let previous_cumulative = cumulative_cash_flow - cash_flow;
            let remaining_amount = initial_cost - previous_cumulative;
            let fraction_of_year = remaining_amount / cash_flow;
            
            return Ok(Some((year as f64) + fraction_of_year + 1.0));
        }
    }
    
    // Investment never pays back within the given cash flows
    Ok(None)
}

/// Calculates Expected Return using CAPM (Capital Asset Pricing Model)
/// 
/// Formula: Expected Return = Risk-free Rate + Beta × (Market Return - Risk-free Rate)
/// 
/// # Arguments
/// * `risk_free_rate` - The risk-free rate (as a decimal)
/// * `beta` - The asset's beta coefficient
/// * `market_return` - The expected market return (as a decimal)
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_capm;
/// 
/// let expected_return = calculate_capm(0.05, 1.2, 0.10).unwrap();
/// assert!((expected_return - 0.11).abs() < 0.001);
/// ```
pub fn calculate_capm(
    risk_free_rate: f64,
    beta: f64,
    market_return: f64
) -> FinanceResult<f64> {
    validate_non_negative(risk_free_rate, "Risk-free rate")?;
    
    if !beta.is_finite() {
        return Err(FinanceError::InvalidInput("Beta must be a valid number".into()));
    }
    
    if !market_return.is_finite() {
        return Err(FinanceError::InvalidInput("Market return must be a valid number".into()));
    }
    
    Ok(risk_free_rate + beta * (market_return - risk_free_rate))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_npv_positive() {
        let cash_flows = vec![1000.0, 1000.0, 1000.0];
        let npv = calculate_npv(2000.0, &cash_flows, 0.05).unwrap();
        assert!(npv > 0.0);
    }

    #[test]
    fn test_npv_negative() {
        let cash_flows = vec![100.0, 100.0, 100.0];
        let npv = calculate_npv(1000.0, &cash_flows, 0.10).unwrap();
        assert!(npv < 0.0);
    }

    #[test]
    fn test_npv_empty_cash_flows() {
        let cash_flows = vec![];
        assert!(calculate_npv(1000.0, &cash_flows, 0.10).is_err());
    }

    #[test]
    fn test_roi_positive() {
        let roi = calculate_roi(500.0, 2000.0).unwrap();
        assert_eq!(roi, 25.0);
    }

    #[test]
    fn test_roi_negative() {
        let roi = calculate_roi(-500.0, 2000.0).unwrap();
        assert_eq!(roi, -25.0);
    }

    #[test]
    fn test_dcf() {
        let cash_flows = vec![1000.0, 2000.0, 3000.0];
        let dcf = calculate_dcf(&cash_flows, 0.10).unwrap();
        assert!(dcf > 4000.0); // Should be less than sum due to discounting
    }

    #[test]
    fn test_payback_period_exact() {
        let cash_flows = vec![100.0, 200.0, 300.0];
        let payback = calculate_payback_period(300.0, &cash_flows).unwrap();
        assert_eq!(payback, Some(3.0)); // Pays back exactly at end of year 3 with 100+200+0 = 300
    }

    #[test]
    fn test_payback_period_interpolated() {
        let cash_flows = vec![100.0, 200.0, 300.0];
        let payback = calculate_payback_period(250.0, &cash_flows).unwrap();
        assert!((payback.unwrap() - 2.75).abs() < 0.001); // 100 + 150/200 = 1.75, then +1 = 2.75 years
    }

    #[test]
    fn test_payback_period_never() {
        let cash_flows = vec![100.0, 100.0, 100.0];
        let payback = calculate_payback_period(1000.0, &cash_flows).unwrap();
        assert_eq!(payback, None);
    }

    #[test]
    fn test_capm() {
        let expected_return = calculate_capm(0.05, 1.2, 0.10).unwrap();
        assert!((expected_return - 0.11).abs() < 0.001);
    }

    #[test]
    fn test_capm_zero_beta() {
        let expected_return = calculate_capm(0.05, 0.0, 0.10).unwrap();
        assert_eq!(expected_return, 0.05);
    }
}