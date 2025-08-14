//! Financial ratio calculation functions

use crate::{FinanceError, FinanceResult, validate_positive, validate_non_negative, to_decimal};
use rust_decimal::prelude::*;

/// Calculates Return on Equity (ROE)
/// 
/// Formula: ROE = (Net Income / Shareholders' Equity) × 100%
/// 
/// # Arguments
/// * `net_income` - The company's net income
/// * `shareholders_equity` - The shareholders' equity
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_roe;
/// 
/// let roe = calculate_roe(1000000.0, 5000000.0).unwrap();
/// assert_eq!(roe, 20.0);
/// ```
pub fn calculate_roe(net_income: f64, shareholders_equity: f64) -> FinanceResult<f64> {
    validate_positive(shareholders_equity, "Shareholders' equity")?;
    
    if !net_income.is_finite() {
        return Err(FinanceError::InvalidInput(format!("Net income must be a valid number: {}", net_income)));
    }
    
    let net_income_decimal = to_decimal(net_income, "net income")?;
    let equity_decimal = to_decimal(shareholders_equity, "shareholders' equity")?;
    
    let roe_decimal = (net_income_decimal / equity_decimal) * Decimal::from(100);
    
    Ok(roe_decimal.to_f64().unwrap_or(0.0))
}

/// Calculates dividend yield
/// 
/// Formula: Dividend Yield = (Annual Dividend per Share / Stock Price) × 100%
/// 
/// # Arguments
/// * `annual_dividend` - The annual dividend per share
/// * `stock_price` - The current stock price
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_dividend_yield;
/// 
/// let yield_rate = calculate_dividend_yield(2.5, 50.0).unwrap();
/// assert_eq!(yield_rate, 5.0);
/// ```
pub fn calculate_dividend_yield(annual_dividend: f64, stock_price: f64) -> FinanceResult<f64> {
    validate_positive(stock_price, "Stock price")?;
    validate_non_negative(annual_dividend, "Annual dividend")?;
    
    Ok((annual_dividend / stock_price) * 100.0)
}

/// Calculates Weighted Average Cost of Capital (WACC)
/// 
/// Formula: WACC = (E/V × Cost of Equity) + (D/V × Cost of Debt × (1 - Tax Rate))
/// Where:
/// - E = Market value of equity
/// - D = Market value of debt
/// - V = E + D (Total value)
/// 
/// # Arguments
/// * `cost_of_equity` - Cost of equity (as a decimal)
/// * `cost_of_debt` - Cost of debt (as a decimal)
/// * `tax_rate` - Corporate tax rate (as a decimal)
/// * `market_value_equity` - Market value of equity
/// * `market_value_debt` - Market value of debt
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_wacc;
/// 
/// let wacc = calculate_wacc(0.10, 0.05, 0.30, 1000000.0, 500000.0).unwrap();
/// assert!((wacc - 0.0783).abs() < 0.001);
/// ```
pub fn calculate_wacc(
    cost_of_equity: f64,
    cost_of_debt: f64,
    tax_rate: f64,
    market_value_equity: f64,
    market_value_debt: f64
) -> FinanceResult<f64> {
    validate_non_negative(cost_of_equity, "Cost of equity")?;
    validate_non_negative(cost_of_debt, "Cost of debt")?;
    validate_non_negative(tax_rate, "Tax rate")?;
    validate_non_negative(market_value_equity, "Market value of equity")?;
    validate_non_negative(market_value_debt, "Market value of debt")?;
    
    if tax_rate > 1.0 {
        return Err(FinanceError::InvalidInput("Tax rate should be expressed as a decimal (0-1)".into()));
    }
    
    let total_value = market_value_equity + market_value_debt;
    if total_value == 0.0 {
        return Err(FinanceError::DivisionByZero);
    }
    
    let equity_weight = market_value_equity / total_value;
    let debt_weight = market_value_debt / total_value;
    
    let wacc = (equity_weight * cost_of_equity) + 
               (debt_weight * cost_of_debt * (1.0 - tax_rate));
    
    Ok(wacc)
}

/// Calculates debt-to-equity ratio
/// 
/// Formula: Debt-to-Equity = Total Debt / Total Equity
/// 
/// # Arguments
/// * `total_debt` - Total debt
/// * `total_equity` - Total equity
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_debt_to_equity;
/// 
/// let ratio = calculate_debt_to_equity(500000.0, 1000000.0).unwrap();
/// assert_eq!(ratio, 0.5);
/// ```
pub fn calculate_debt_to_equity(total_debt: f64, total_equity: f64) -> FinanceResult<f64> {
    validate_non_negative(total_debt, "Total debt")?;
    validate_positive(total_equity, "Total equity")?;
    
    Ok(total_debt / total_equity)
}

/// Calculates current ratio (liquidity ratio)
/// 
/// Formula: Current Ratio = Current Assets / Current Liabilities
/// 
/// # Arguments
/// * `current_assets` - Current assets
/// * `current_liabilities` - Current liabilities
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_current_ratio;
/// 
/// let ratio = calculate_current_ratio(200000.0, 100000.0).unwrap();
/// assert_eq!(ratio, 2.0);
/// ```
pub fn calculate_current_ratio(current_assets: f64, current_liabilities: f64) -> FinanceResult<f64> {
    validate_non_negative(current_assets, "Current assets")?;
    validate_positive(current_liabilities, "Current liabilities")?;
    
    Ok(current_assets / current_liabilities)
}

/// Calculates quick ratio (acid-test ratio)
/// 
/// Formula: Quick Ratio = (Current Assets - Inventory) / Current Liabilities
/// 
/// # Arguments
/// * `current_assets` - Current assets
/// * `inventory` - Inventory value
/// * `current_liabilities` - Current liabilities
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_quick_ratio;
/// 
/// let ratio = calculate_quick_ratio(200000.0, 50000.0, 100000.0).unwrap();
/// assert_eq!(ratio, 1.5);
/// ```
pub fn calculate_quick_ratio(
    current_assets: f64, 
    inventory: f64, 
    current_liabilities: f64
) -> FinanceResult<f64> {
    validate_non_negative(current_assets, "Current assets")?;
    validate_non_negative(inventory, "Inventory")?;
    validate_positive(current_liabilities, "Current liabilities")?;
    
    if inventory > current_assets {
        return Err(FinanceError::InvalidInput("Inventory cannot exceed current assets".into()));
    }
    
    let liquid_assets = current_assets - inventory;
    Ok(liquid_assets / current_liabilities)
}

/// Calculates Return on Assets (ROA)
/// 
/// Formula: ROA = (Net Income / Total Assets) × 100%
/// 
/// # Arguments
/// * `net_income` - Net income
/// * `total_assets` - Total assets
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_roa;
/// 
/// let roa = calculate_roa(500000.0, 5000000.0).unwrap();
/// assert_eq!(roa, 10.0);
/// ```
pub fn calculate_roa(net_income: f64, total_assets: f64) -> FinanceResult<f64> {
    validate_positive(total_assets, "Total assets")?;
    
    if !net_income.is_finite() {
        return Err(FinanceError::InvalidInput("Net income must be a valid number".into()));
    }
    
    Ok((net_income / total_assets) * 100.0)
}

/// Calculates Price-to-Earnings (P/E) ratio
/// 
/// Formula: P/E Ratio = Stock Price / Earnings per Share
/// 
/// # Arguments
/// * `stock_price` - Current stock price
/// * `earnings_per_share` - Earnings per share
/// 
/// # Examples
/// ```
/// use rusty_finance::calculations::calculate_pe_ratio;
/// 
/// let pe = calculate_pe_ratio(50.0, 5.0).unwrap();
/// assert_eq!(pe, 10.0);
/// ```
pub fn calculate_pe_ratio(stock_price: f64, earnings_per_share: f64) -> FinanceResult<f64> {
    validate_positive(stock_price, "Stock price")?;
    validate_positive(earnings_per_share, "Earnings per share")?;
    
    Ok(stock_price / earnings_per_share)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_roe() {
        let roe = calculate_roe(1000000.0, 5000000.0).unwrap();
        assert_eq!(roe, 20.0);
    }

    #[test]
    fn test_calculate_roe_negative_income() {
        let roe = calculate_roe(-500000.0, 5000000.0).unwrap();
        assert_eq!(roe, -10.0);
    }

    #[test]
    fn test_calculate_roe_zero_equity() {
        assert!(calculate_roe(1000000.0, 0.0).is_err());
    }

    #[test]
    fn test_calculate_dividend_yield() {
        let yield_rate = calculate_dividend_yield(2.5, 50.0).unwrap();
        assert_eq!(yield_rate, 5.0);
    }

    #[test]
    fn test_calculate_dividend_yield_zero_dividend() {
        let yield_rate = calculate_dividend_yield(0.0, 50.0).unwrap();
        assert_eq!(yield_rate, 0.0);
    }

    #[test]
    fn test_calculate_wacc() {
        let wacc = calculate_wacc(0.10, 0.05, 0.30, 1000000.0, 500000.0).unwrap();
        assert!((wacc - 0.0783333).abs() < 0.0001);
    }

    #[test]
    fn test_calculate_wacc_zero_values() {
        assert!(calculate_wacc(0.10, 0.05, 0.30, 0.0, 0.0).is_err());
    }

    #[test]
    fn test_calculate_debt_to_equity() {
        let ratio = calculate_debt_to_equity(500000.0, 1000000.0).unwrap();
        assert_eq!(ratio, 0.5);
    }

    #[test]
    fn test_calculate_current_ratio() {
        let ratio = calculate_current_ratio(200000.0, 100000.0).unwrap();
        assert_eq!(ratio, 2.0);
    }

    #[test]
    fn test_calculate_quick_ratio() {
        let ratio = calculate_quick_ratio(200000.0, 50000.0, 100000.0).unwrap();
        assert_eq!(ratio, 1.5);
    }

    #[test]
    fn test_calculate_quick_ratio_invalid_inventory() {
        assert!(calculate_quick_ratio(100000.0, 150000.0, 50000.0).is_err());
    }

    #[test]
    fn test_calculate_roa() {
        let roa = calculate_roa(500000.0, 5000000.0).unwrap();
        assert_eq!(roa, 10.0);
    }

    #[test]
    fn test_calculate_pe_ratio() {
        let pe = calculate_pe_ratio(50.0, 5.0).unwrap();
        assert_eq!(pe, 10.0);
    }
}