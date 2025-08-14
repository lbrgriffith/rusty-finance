use anyhow::{Context, Result};
use chrono::{Local, Months};
use clap::{CommandFactory, Parser};
use clap_complete::{generate, shells::{Bash, Fish, Zsh, PowerShell}};
use std::io;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use comfy_table::{Cell, CellAlignment, Color, ContentArrangement, Table};
use env_logger::Env;
use log::{debug, info, warn};
use owo_colors::OwoColorize;
use rust_decimal::prelude::*;
use thiserror::Error;

/// Custom finance calculation errors
#[derive(Error, Debug)]
enum FinanceError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Division by zero")]
    DivisionByZero,
}

/// Financial calculation tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Opts {
    /// Verbosity level for logging
    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,
    
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    /// Calculates simple interest.
    Interest(Interest),
    
    /// Calculates compound interest.
    CompoundInterest(CompoundInterest),
    
    /// Calculates present value.
    PresentValue(PresentValue),
    
    /// Calculates future value.
    FutureValue(FutureValue),
    
    /// Calculates net present value.
    NPV(NPV),
    
    /// Calculates an amortization schedule.
    Amortization(Amortization),
    
    /// Calculates the return on investment (ROI).
    ROI(ROI),
    
    /// Calculates the average of a series of numbers.
    Average(Average),
    
    /// Calculates the mode of a series of numbers.
    Mode(Mode),
    
    /// Calculates the median of a series of numbers.
    Medium(Medium),
    
    /// Calculates the payback period.
    PaybackPeriod(PaybackPeriod),
    
    /// Performs break-even analysis.
    BreakEven(BreakEven),
    
    /// Calculates the depreciation of an asset.
    Depreciation(Depreciation),
    
    /// Calculates the internal rate of return (IRR).
    IRR(IRR),
    
    /// Calculates the variance of a series of numbers.
    Variance(Variance),
    
    /// Calculates the standard deviation of a series of numbers.
    StandardDeviation(StandardDeviation),
    
    /// Calculates probability.
    Probability(Probability),
    
    /// Calculates the expected return on an investment based on its risk and market factors.
    CAPM(CAPM),
    
    /// Calculate loan payments, including the monthly payment amount, total interest paid, and the loan payoff date.
    LoanPayment(LoanPayment),
    
    /// Calculate the number of units a business needs to sell to break even.
    BreakEvenUnits(BreakEvenUnits),
    
    /// Calculate the discounted cash flow.
    DCF(DCF),
    
    /// Calculates mortgage payments, total interest paid, and loan payoff date
    Mortgage(Mortgage),
    
    /// Calculates the weighted average of a series of numbers.
    #[clap(name = "weighted-average")]
    WeightedAverage(WeightedAverage),
    
    /// Calculates the weighted average cost of capital (WACC).
    WACC(WACC),
    
    /// Calculates the dividend yield.
    DividendYield(DividendYield),
    
    /// Calculates the return on equity (ROE).
    ReturnOnEquity(ReturnOnEquity),
    
    /// Generate shell completions.
    Completion(Completion),
}

#[derive(Parser, Debug)]
pub struct ReturnOnEquity {
    /// Net Income
    #[clap(short, long)]
    net_income: f64,

    /// Equity
    #[clap(short, long)]
    equity: f64,
}

#[derive(Parser, Debug)]
struct Interest {
    /// The principal amount (initial money)
    #[clap(short, long)]
    principal: f64,

    /// The rate of interest (per period)
    #[clap(short, long)]
    rate: f64,

    /// The time the money is invested for
    #[clap(short, long)]
    time: f64,
}

#[derive(Parser, Debug)]
struct CompoundInterest {
    /// The principal amount (initial money)
    #[clap(short, long)]
    principal: f64,

    /// The annual interest rate
    #[clap(short, long)]
    rate: f64,

    /// The number of times interest is compounded per year
    #[clap(short, long)]
    n: i32,

    /// The time the money is invested for in years
    #[clap(short, long)]
    t: i32,
}

impl ReturnOnEquity {
    /// Calculate ROE and display results with error handling
    fn execute(&self) -> Result<()> {
        debug!("Calculating ROE with: {:?}", self);
        
        // Validate inputs are finite numbers
        if !self.net_income.is_finite() {
            return Err(FinanceError::InvalidInput(format!("Net income must be a valid number: {}", self.net_income)).into());
        }
        
        if !self.equity.is_finite() {
            return Err(FinanceError::InvalidInput(format!("Equity must be a valid number: {}", self.equity)).into());
        }
        
        // Convert net_income and equity to Decimal with proper error handling
        let net_income = Decimal::from_f64(self.net_income)
            .ok_or_else(|| FinanceError::InvalidInput(format!("Invalid net income: {}", self.net_income)))?;
        
        let equity = Decimal::from_f64(self.equity)
            .ok_or_else(|| FinanceError::InvalidInput(format!("Invalid equity: {}", self.equity)))?;
            
        // Validate inputs
        if equity.is_zero() {
            return Err(FinanceError::DivisionByZero.into());
        }
        
        // Equity should be positive for a meaningful ROE calculation
        if equity < Decimal::ZERO {
            return Err(FinanceError::InvalidInput(format!("Equity should be positive: {}", self.equity)).into());
        }

        // Calculate the return on equity
        let roe = (net_income / equity) * Decimal::from_f64(100.0).unwrap();
        info!("Calculated ROE: {:.4}%", roe);

        // Create the table with modern styling
        let mut table = create_table(vec!["Net Income", "Equity", "Return on Equity"]);
        
        // Add data row with colorful formatting
        table.add_row(vec![
            Cell::new(&format_currency(self.net_income)),
            Cell::new(&format_currency(self.equity)),
            Cell::new(&format!("{:.2}%", roe)).fg(Color::Green).set_alignment(CellAlignment::Right),
        ]);

        // Print the table
        println!("{table}");
        
        info!("ROE calculation completed successfully");
        Ok(())
    }
}

#[derive(Parser, Debug)]
struct PresentValue {
    /// The future value of the investment.
    #[clap(short, long)]
    future_value: f64,

    /// The interest rate per period.
    #[clap(short, long)]
    rate: f64,

    /// The number of periods.
    #[clap(short, long)]
    time: f64,
}

#[derive(Parser, Debug)]
struct FutureValue {
    /// The present value of the investment.
    #[clap(short, long)]
    present_value: f64,

    /// The interest rate per period.
    #[clap(short, long)]
    rate: f64,

    /// The number of periods.
    #[clap(short, long)]
    time: f64,
}

#[derive(Parser, Debug)]
struct DividendYield {
    /// The dividend
    #[clap(short, long)]
    dividend: f64,

    /// The price
    #[clap(short, long)]
    price: f64,
}

#[derive(Parser, Debug)]
struct NPV {
    /// The initial investment or cost
    #[clap(short, long, name = "initial-investment")]
    initial_investment: f64,

    /// The annual cash inflow
    #[clap(short, long, name = "cash-inflow")]
    cash_inflow: f64,

    /// The discount rate
    #[clap(short, long, name = "discount-rate")]
    discount_rate: f64,

    /// The lifespan of the investment in years
    #[clap(short, long, name = "lifespan")]
    lifespan: i32,
}

#[derive(Parser, Debug)]
struct Amortization {
    /// The initial loan amount
    #[clap(short = 'a', long)]
    loan_amount: f64,

    /// The annual interest rate
    #[clap(short = 'i', long)]
    annual_interest_rate: f64,

    /// The loan term in years
    #[clap(short = 't', long)]
    loan_term_years: i32,
}

/// Calculate present value
fn calculate_present_value(future_value: f64, rate: f64, time: f64) -> Result<f64> {
    if !future_value.is_finite() || !rate.is_finite() || !time.is_finite() {
        return Err(FinanceError::InvalidInput("All inputs must be valid numbers".into()).into());
    }
    
    if rate < 0.0 {
        return Err(FinanceError::InvalidInput("Rate must be positive".into()).into());
    }
    
    if time < 0.0 {
        return Err(FinanceError::InvalidInput("Time must be positive".into()).into());
    }
    
    let result = future_value / (1.0 + rate).powf(time);
    Ok(result)
}

/// Calculate future value
fn calculate_future_value(present_value: f64, rate: f64, time: f64) -> Result<f64> {
    if !present_value.is_finite() || !rate.is_finite() || !time.is_finite() {
        return Err(FinanceError::InvalidInput("All inputs must be valid numbers".into()).into());
    }
    
    if rate < 0.0 {
        return Err(FinanceError::InvalidInput("Rate must be positive".into()).into());
    }
    
    if time < 0.0 {
        return Err(FinanceError::InvalidInput("Time must be positive".into()).into());
    }
    
    let result = present_value * (1.0 + rate).powf(time);
    Ok(result)
}

/// Calculate dividend yield with proper error handling
fn calculate_dividend_yield(dividend_yield: &DividendYield) -> Result<f64> {
    // Validate inputs
    if dividend_yield.price <= 0.0 {
        return Err(FinanceError::InvalidInput(format!("Price must be positive: {}", dividend_yield.price)).into());
    }
    
    // Dividend can be negative for stocks that lose money, but let's validate it's not NaN or infinite
    if !dividend_yield.dividend.is_finite() {
        return Err(FinanceError::InvalidInput(format!("Dividend must be a valid number: {}", dividend_yield.dividend)).into());
    }
    
    // Calculate dividend yield
    let result = dividend_yield.dividend / dividend_yield.price;
    
    debug!("Calculated dividend yield: {:.4}", result);
    Ok(result)
}

/// Create a styled table with the given headers
#[derive(Parser, Debug)]
struct ROI {
    /// The net profit
    #[clap(short, long, name = "net-profit")]
    net_profit: f64,

    /// The cost of investment
    #[clap(short, long, name = "cost-of-investment")]
    cost_of_investment: f64,
}

#[derive(Parser, Debug)]
struct Average {
    /// The numbers to calculate the average of
    #[clap(name = "numbers")]
    numbers: Vec<f64>,
}

#[derive(Parser, Debug)]
struct Mode {
    /// The numbers to calculate the mode
    #[clap(required = true)]
    numbers: Vec<f64>,
}

#[derive(Parser, Debug)]
struct Medium {
    /// The numbers to calculate the median
    #[clap(name = "numbers", required = true)]
    numbers: Vec<f64>,
}

fn create_table(headers: Vec<&str>) -> Table {
    let mut table = Table::new();
    
    // Set up table styling
    table
        .set_header(headers)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(80)
        .load_preset(comfy_table::presets::UTF8_BORDERS_ONLY);
    
    table
}

#[derive(Parser, Debug)]
struct PaybackPeriod {
    /// The list of cash flows
    #[clap(short = 'c', long = "cash-flows", name = "cash-flows")]
    cash_flows: Vec<f64>,

    /// The initial cost of the investment
    #[clap(short = 'i', long = "initial-cost", name = "initial-cost")]
    initial_cost: f64,
}

#[derive(Parser, Debug)]
struct BreakEven {
    /// The total fixed costs incurred by the business
    #[clap(short, long, name = "fixed-costs")]
    fixed_costs: f64,

    /// The variable costs per unit
    #[clap(short = 'c', long, name = "variable-costs")]
    variable_costs: f64,

    /// The price per unit of the product or service
    #[clap(short, long, name = "price-per-unit")]
    price_per_unit: f64,
}

#[derive(Parser, Debug)]
struct Depreciation {
    /// The initial value of the asset
    #[clap(short, long, name = "initial-value")]
    initial_value: f64,

    /// The salvage value of the asset
    #[clap(short, long, name = "salvage-value")]
    salvage_value: f64,

    /// The useful life of the asset
    #[clap(short, long, name = "useful-life")]
    useful_life: f64,

    /// The method of depreciation (e.g., straight-line, double-declining-balance)
    #[clap(short, long, name = "depreciation-method")]
    depreciation_method: String,
}

#[derive(Parser, Debug)]
struct IRR {
    /// The cash flows for the investment/project
    #[clap(name = "cash-flows")]
    cash_flows: Vec<f64>,
}

/// Format a number as currency with colored output
fn format_currency(number: f64) -> String {
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
    
    // Format as currency
    if number >= 0.0 {
        format!("${}.{}", whole_with_commas, decimal_part).green().to_string()
    } else {
        format!("${}.{}", whole_with_commas, decimal_part).red().to_string()
    }
}

#[derive(Parser, Debug)]
struct Variance {
    /// The numbers to calculate the variance
    #[clap(name = "numbers")]
    numbers: Vec<String>,
}

#[derive(Parser, Debug)]
struct StandardDeviation {
    /// The numbers to calculate the standard deviation of.
    #[clap(name = "numbers")]
    numbers: Vec<f64>,
}

#[derive(Parser, Debug)]
struct Probability {
    /// The number of successful outcomes
    #[clap(short, long)]
    successes: u32,

    /// The number of total trials
    #[clap(short, long)]
    trials: u32,
}

#[derive(Parser, Debug)]
struct CAPM {
    /// The risk-free rate
    #[clap(short, long)]
    risk_free_rate: f64,

    /// The asset's beta coefficient
    #[clap(short, long)]
    beta: f64,

    /// The expected return of the market
    #[clap(short, long)]
    market_return: f64,
}

/// Add thousands separators (commas) to a number string
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

#[derive(Parser, Debug)]
struct LoanPayment {
    /// The principal amount of the loan
    #[clap(short, long)]
    principal: f64,

    /// The annual interest rate of the loan
    #[clap(short, long)]
    interest_rate: f64,

    /// The loan term in years
    #[clap(short, long)]
    loan_term: f64,
}

#[derive(Parser, Debug)]
struct BreakEvenUnits {
    /// The fixed costs incurred by the business
    #[clap(short, long, name = "fixed-costs")]
    fixed_costs: f64,

    /// The variable costs per unit
    #[clap(short = 'c', long, name = "variable-costs")]
    variable_costs: f64,

    /// The price per unit of the product or service
    #[clap(short, long, name = "price-per-unit")]
    price_per_unit: f64,
}

#[derive(Parser, Debug)]
struct DCF {
    /// The discount rate
    #[clap(short, long, name = "discount-rate")]
    discount_rate: f64,

    /// The cash flows for the investment/project
    #[clap(name = "cash-flows")]
    cash_flows: Vec<f64>,
}

#[derive(Parser, Debug)]
struct Mortgage {
    /// The loan amount
    #[clap(short, long, name = "loan-amount")]
    loan_amount: f64,

    /// The annual interest rate
    #[clap(short, long, name = "interest-rate")]
    interest_rate: f64,

    /// The loan term in years
    #[clap(short, long)]
    term: i32,
}

#[derive(Parser, Debug)]
struct WeightedAverage {
    /// The numbers to calculate the weighted average of
    #[clap(short, long)]
    numbers: String,

    /// The weights for each number
    #[clap(short, long)]
    weights: String,
}

#[derive(Parser, Debug)]
struct WACC {
    /// The cost of equity (Ke)
    #[clap(long)]
    cost_of_equity: f64,

    /// The cost of debt (Kd)
    #[clap(long)]
    cost_of_debt: f64,

    /// The tax rate
    #[clap(long)]
    tax_rate: f64,

    /// The market value of equity (E)
    #[clap(long)]
    market_value_equity: f64,

    /// The market value of debt (D)
    #[clap(long)]
    market_value_debt: f64,
}

#[derive(Parser, Debug)]
struct Completion {
    /// Shell to generate completions for
    #[clap(value_enum)]
    shell: Shell,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Shell {
    Bash,
    Fish,
    Zsh,
    PowerShell,
}

/// Run the application with proper error handling
fn run() -> Result<()> {
    // Parse command line arguments
    let opts: Opts = Opts::parse();
    
    // Configure logging based on verbosity
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .filter_level(opts.verbose.log_level_filter())
        .init();
    
    info!("Starting rusty-finance");
    debug!("Using options: {:?}", opts);
    
    // Execute the selected command
    match opts.command {
        Command::Interest(interest) => {
            debug!("Calculating simple interest with: {:?}", interest);
            
            // Validate inputs
            if !interest.principal.is_finite() || !interest.rate.is_finite() || !interest.time.is_finite() {
                return Err(FinanceError::InvalidInput("All inputs must be valid numbers".into()).into());
            }
            
            if interest.principal < 0.0 {
                return Err(FinanceError::InvalidInput("Principal must be positive".into()).into());
            }
            
            if interest.rate < 0.0 {
                return Err(FinanceError::InvalidInput("Rate must be positive".into()).into());
            }
            
            if interest.time < 0.0 {
                return Err(FinanceError::InvalidInput("Time must be positive".into()).into());
            }
            
            // Calculate simple interest
            let result = interest.principal * interest.rate * interest.time;
            info!("Calculated simple interest: {:.4}", result);
            
            // Create and format table
            let mut table = create_table(vec!["Principal", "Rate", "Time", "Simple Interest"]);
            
            // Add data row
            table.add_row(vec![
                Cell::new(&format_currency(interest.principal)),
                Cell::new(&format!("{:.2}%", interest.rate * 100.0)),
                Cell::new(&format!("{:.2} years", interest.time)),
                Cell::new(&format_currency(result)).fg(Color::Green),
            ]);
            
            // Print the table
            println!("{table}");
            
            info!("Simple interest calculation completed");
            Ok(())
        }
        Command::CompoundInterest(ci) => {
            debug!("Calculating compound interest with: {:?}", ci);
            
            // Validate inputs
            if !ci.principal.is_finite() || !ci.rate.is_finite() {
                return Err(FinanceError::InvalidInput("All inputs must be valid numbers".into()).into());
            }
            
            if ci.principal < 0.0 {
                return Err(FinanceError::InvalidInput("Principal must be positive".into()).into());
            }
            
            if ci.rate < 0.0 {
                return Err(FinanceError::InvalidInput("Rate must be positive".into()).into());
            }
            
            if ci.n <= 0 {
                return Err(FinanceError::InvalidInput("Compounding periods must be positive".into()).into());
            }
            
            if ci.t <= 0 {
                return Err(FinanceError::InvalidInput("Time must be positive".into()).into());
            }
            
            // Create and format table
            let mut table = create_table(vec!["Year", "Amount"]);
            
            // Calculate compound interest for each year
            let mut amount = ci.principal;
            let rate_per_period = ci.rate / ci.n as f64;
            
            for year in 1..=ci.t {
                for _ in 0..ci.n {
                    amount *= 1.0 + rate_per_period;
                }
                
                table.add_row(vec![
                    Cell::new(&format!("{}", year)),
                    Cell::new(&format_currency(amount)).fg(Color::Green),
                ]);
            }
            
            // Print the table
            println!("{table}");
            
            info!("Compound interest calculation completed");
            Ok(())
        }
        Command::DividendYield(dividend_yield) => {
            debug!("Calculating dividend yield with: {:?}", dividend_yield);
            
            // Calculate dividend yield with error handling
            let result = calculate_dividend_yield(&dividend_yield)
                .context("Failed to calculate dividend yield")?;
            
            info!("Calculated dividend yield: {:.4}", result);
            
            // Create and format table
            let mut table = create_table(vec!["Dividend", "Price", "Dividend Yield (%)"]);

            // Add data row
            table.add_row(vec![
                Cell::new(&format!("{:.2}", dividend_yield.dividend)),
                Cell::new(&format!("{:.2}", dividend_yield.price)),
                Cell::new(&format!("{:.2}%", result * 100.0)).fg(Color::Green),
            ]);

            // Print the table
            println!("{table}");
            
            info!("Dividend yield calculation completed");
            Ok(())
        }
        Command::PresentValue(pv) => {
            debug!("Calculating present value with: {:?}", pv);
            
            // Calculate present value with error handling
            let result = calculate_present_value(pv.future_value, pv.rate, pv.time)
                .context("Failed to calculate present value")?;
            
            info!("Calculated present value: {:.4}", result);
            
            // Create and format table
            let mut table = create_table(vec!["Future Value", "Rate", "Time", "Present Value"]);
            
            // Add data row
            table.add_row(vec![
                Cell::new(&format_currency(pv.future_value)),
                Cell::new(&format!("{:.2}%", pv.rate * 100.0)),
                Cell::new(&format!("{:.2} years", pv.time)),
                Cell::new(&format_currency(result)).fg(Color::Green),
            ]);
            
            // Print the table
            println!("{table}");
            
            info!("Present value calculation completed");
            Ok(())
        }
        Command::FutureValue(fv) => {
            debug!("Calculating future value with: {:?}", fv);
            
            // Calculate future value with error handling
            let result = calculate_future_value(fv.present_value, fv.rate, fv.time)
                .context("Failed to calculate future value")?;
            
            info!("Calculated future value: {:.4}", result);
            
            // Create and format table
            let mut table = create_table(vec!["Present Value", "Rate", "Time", "Future Value"]);
            
            // Add data row
            table.add_row(vec![
                Cell::new(&format_currency(fv.present_value)),
                Cell::new(&format!("{:.2}%", fv.rate * 100.0)),
                Cell::new(&format!("{:.2} years", fv.time)),
                Cell::new(&format_currency(result)).fg(Color::Green),
            ]);
            
            // Print the table
            println!("{table}");
            
            info!("Future value calculation completed");
            Ok(())
        }
        Command::NPV(npv) => {
            debug!("Calculating NPV with: {:?}", npv);
            
            // Validate inputs
            if !npv.initial_investment.is_finite() || !npv.cash_inflow.is_finite() || !npv.discount_rate.is_finite() {
                return Err(FinanceError::InvalidInput("All inputs must be valid numbers".into()).into());
            }
            
            if npv.initial_investment < 0.0 {
                return Err(FinanceError::InvalidInput("Initial investment must be positive".into()).into());
            }
            
            if npv.discount_rate < 0.0 {
                return Err(FinanceError::InvalidInput("Discount rate must be positive".into()).into());
            }
            
            if npv.lifespan <= 0 {
                return Err(FinanceError::InvalidInput("Lifespan must be positive".into()).into());
            }
            
            // Calculate NPV
            let mut npv_value = -npv.initial_investment;
            
            // Create and format table
            let mut table = create_table(vec!["Year", "Cash Inflow", "Discounted Cash Flow"]);
            
            // Calculate and display each year's discounted cash flow
            for year in 1..=npv.lifespan {
                let discounted_cash_flow = npv.cash_inflow / (1.0 + npv.discount_rate).powf(year as f64);
                npv_value += discounted_cash_flow;
                
                table.add_row(vec![
                    Cell::new(&format!("{}", year)),
                    Cell::new(&format_currency(npv.cash_inflow)),
                    Cell::new(&format_currency(discounted_cash_flow)),
                ]);
            }
            
            // Print the table
            println!("{table}");
            
            // Print the net present value
            println!("\n{}: {}", 
                "Net Present Value (NPV)".bold(), 
                format_currency(npv_value)
            );
            
            info!("NPV calculation completed. NPV: {:.2}", npv_value);
            Ok(())
        }
        Command::ROI(roi) => {
            debug!("Calculating ROI with: {:?}", roi);
            
            // Validate inputs
            if !roi.net_profit.is_finite() || !roi.cost_of_investment.is_finite() {
                return Err(FinanceError::InvalidInput("All inputs must be valid numbers".into()).into());
            }
            
            if roi.cost_of_investment == 0.0 {
                return Err(FinanceError::DivisionByZero.into());
            }
            
            // Calculate ROI
            let roi_value = (roi.net_profit / roi.cost_of_investment) * 100.0;
            info!("Calculated ROI: {:.4}%", roi_value);
            
            // Create and format table
            let mut table = create_table(vec!["Net Profit", "Cost of Investment", "ROI"]);
            
            // Add data row with color coding based on ROI value
            let roi_cell = if roi_value >= 0.0 {
                Cell::new(&format!("{:.2}%", roi_value)).fg(Color::Green)
            } else {
                Cell::new(&format!("{:.2}%", roi_value)).fg(Color::Red)
            };
            
            table.add_row(vec![
                Cell::new(&format_currency(roi.net_profit)),
                Cell::new(&format_currency(roi.cost_of_investment)),
                roi_cell,
            ]);
            
            // Print the table
            println!("{table}");
            
            info!("ROI calculation completed");
            Ok(())
        }
        Command::Amortization(amortization) => {
            debug!("Calculating amortization schedule with: {:?}", amortization);
            
            // Validate inputs
            if !amortization.loan_amount.is_finite() || !amortization.annual_interest_rate.is_finite() {
                return Err(FinanceError::InvalidInput("All inputs must be valid numbers".into()).into());
            }
            
            if amortization.loan_amount <= 0.0 {
                return Err(FinanceError::InvalidInput("Loan amount must be positive".into()).into());
            }
            
            if amortization.annual_interest_rate <= 0.0 {
                return Err(FinanceError::InvalidInput("Interest rate must be positive".into()).into());
            }
            
            if amortization.loan_term_years <= 0 {
                return Err(FinanceError::InvalidInput("Loan term must be positive".into()).into());
            }
            
            // Calculate amortization schedule
            let monthly_interest_rate = amortization.annual_interest_rate / 12.0;
            let total_payments = amortization.loan_term_years * 12;
            
            // Calculate monthly payment
            let monthly_payment = (monthly_interest_rate * amortization.loan_amount) / 
                (1.0 - (1.0 + monthly_interest_rate).powf(-total_payments as f64));
            
            // Create and format table
            let mut table = create_table(vec!["Month", "Principal", "Interest", "Remaining Balance"]);
            
            // Calculate amortization schedule
            let mut remaining_balance = amortization.loan_amount;
            
            for month in 1..=total_payments {
                let interest_payment = monthly_interest_rate * remaining_balance;
                let principal_payment = monthly_payment - interest_payment;
                remaining_balance -= principal_payment;
                
                // Add row to table (every 12 months to avoid huge output)
                if month % 12 == 0 || month == 1 || month == total_payments {
                    table.add_row(vec![
                        Cell::new(&format!("{}", month)),
                        Cell::new(&format_currency(principal_payment)),
                        Cell::new(&format_currency(interest_payment)),
                        Cell::new(&format_currency(remaining_balance)),
                    ]);
                }
            }
            
            // Print the table
            println!("{table}");
            
            info!("Amortization calculation completed");
            Ok(())
        }
        Command::Average(average) => {
            debug!("Calculating average with: {:?}", average);
            
            if average.numbers.is_empty() {
                return Err(FinanceError::InvalidInput("No numbers provided".into()).into());
            }
            
            // Calculate average
            let sum: f64 = average.numbers.iter().sum();
            let avg = sum / average.numbers.len() as f64;
            
            // Create and format table
            let mut table = create_table(vec!["Number"]);
            
            // Add each number to the table
            for number in &average.numbers {
                table.add_row(vec![
                    Cell::new(&format!("{:.2}", number)),
                ]);
            }
            
            // Add average row
            table.add_row(vec![
                Cell::new(&format!("Average: {:.2}", avg)).fg(Color::Green),
            ]);
            
            // Print the table
            println!("{table}");
            
            info!("Average calculation completed: {:.4}", avg);
            Ok(())
        }
        Command::Mode(mode) => {
            debug!("Calculating mode with: {:?}", mode);
            
            if mode.numbers.is_empty() {
                return Err(FinanceError::InvalidInput("No numbers provided".into()).into());
            }
            
            // Calculate mode using a different approach since f64 doesn't implement Hash/Eq
            // Use a Vec<(f64, u32)> to store numbers and their counts
            let mut counts: Vec<(f64, u32)> = Vec::new();
            
            for &number in mode.numbers.iter() {
                // Check if the number already exists in our counts
                match counts.iter_mut().find(|(n, _)| (n - number).abs() < f64::EPSILON) {
                    Some((_, count)) => *count += 1,
                    None => counts.push((number, 1)),
                }
            }
            
            // Find the highest frequency
            if let Some(max_freq) = counts.iter().map(|(_, count)| *count).max() {
                // Find all numbers with this frequency
                let mode_values: Vec<f64> = counts.iter()
                    .filter(|(_, count)| *count == max_freq)
                    .map(|(number, _)| *number)
                    .collect();
                
                // Check if there's a tie
                let has_tie = mode_values.len() > 1;
                
                // Create and format table
                let mut table = create_table(vec!["Number", "Frequency"]);
                
                // Add each number and its frequency to the table
                for (number, count) in &counts {
                    let freq_cell = if *count == max_freq {
                        Cell::new(&format!("{}", count)).fg(Color::Green)
                    } else {
                        Cell::new(&format!("{}", count))
                    };
                    
                    table.add_row(vec![
                        Cell::new(&format!("{:.2}", number)),
                        freq_cell,
                    ]);
                }
                
                // Add mode row
                if has_tie {
                    table.add_row(vec![
                        Cell::new("Mode:").fg(Color::Yellow),
                        Cell::new("Multiple modes").fg(Color::Yellow),
                    ]);
                } else {
                    table.add_row(vec![
                        Cell::new("Mode:").fg(Color::Green),
                        Cell::new(&format!("{:.2}", mode_values[0])).fg(Color::Green),
                    ]);
                }
                
                // Print the table
                println!("{table}");
                
                info!("Mode calculation completed");
                Ok(())
            } else {
                Err(FinanceError::InvalidInput("Failed to calculate mode".into()).into())
            }
        }
        Command::Medium(medium) => {
            debug!("Calculating median with: {:?}", medium);
            
            if medium.numbers.is_empty() {
                return Err(FinanceError::InvalidInput("No numbers provided".into()).into());
            }
            
            // Sort the numbers
            let mut sorted = medium.numbers.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            
            // Calculate median
            let len = sorted.len();
            let median = if len % 2 == 0 {
                // Even number of elements
                (sorted[len / 2 - 1] + sorted[len / 2]) / 2.0
            } else {
                // Odd number of elements
                sorted[len / 2]
            };
            
            // Create and format table
            let mut table = create_table(vec!["Number"]);
            
            // Add each number to the table
            for (idx, &number) in sorted.iter().enumerate() {
                let is_median = if len % 2 == 0 {
                    idx == len / 2 - 1 || idx == len / 2
                } else {
                    idx == len / 2
                };
                
                let cell = if is_median {
                    Cell::new(&format!("{:.2}", number)).fg(Color::Green)
                } else {
                    Cell::new(&format!("{:.2}", number))
                };
                
                table.add_row(vec![cell]);
            }
            
            // Add median row
            table.add_row(vec![
                Cell::new(&format!("Median: {:.2}", median)).fg(Color::Green),
            ]);
            
            // Print the table
            println!("{table}");
            
            info!("Median calculation completed: {:.4}", median);
            Ok(())
        }
        Command::WACC(wacc) => {
            debug!("Calculating WACC with: {:?}", wacc);
            
            // Validate inputs
            if !wacc.cost_of_equity.is_finite() || !wacc.cost_of_debt.is_finite() || 
               !wacc.tax_rate.is_finite() || !wacc.market_value_equity.is_finite() || 
               !wacc.market_value_debt.is_finite() {
                return Err(FinanceError::InvalidInput("All inputs must be valid numbers".into()).into());
            }
            
            if wacc.market_value_equity < 0.0 || wacc.market_value_debt < 0.0 {
                return Err(FinanceError::InvalidInput("Market values must be non-negative".into()).into());
            }
            
            if wacc.tax_rate < 0.0 || wacc.tax_rate > 1.0 {
                return Err(FinanceError::InvalidInput("Tax rate must be between 0 and 1".into()).into());
            }
            
            // Check for division by zero
            if wacc.market_value_equity + wacc.market_value_debt == 0.0 {
                return Err(FinanceError::DivisionByZero.into());
            }
            
            // Calculate WACC
            let wacc_value = (wacc.cost_of_equity * wacc.market_value_equity + 
                           wacc.cost_of_debt * (1.0 - wacc.tax_rate) * wacc.market_value_debt) / 
                          (wacc.market_value_equity + wacc.market_value_debt);
            
            // Create and format table
            let mut table = create_table(vec!["Component", "Value"]);
            
            // Add rows to table
            table.add_row(vec![
                Cell::new("Cost of Equity (Ke)"),
                Cell::new(&format!("{:.2}%", wacc.cost_of_equity * 100.0)),
            ]);
            
            table.add_row(vec![
                Cell::new("Cost of Debt (Kd)"),
                Cell::new(&format!("{:.2}%", wacc.cost_of_debt * 100.0)),
            ]);
            
            table.add_row(vec![
                Cell::new("Tax Rate"),
                Cell::new(&format!("{:.2}%", wacc.tax_rate * 100.0)),
            ]);
            
            table.add_row(vec![
                Cell::new("Market Value of Equity (E)"),
                Cell::new(&format_currency(wacc.market_value_equity)),
            ]);
            
            table.add_row(vec![
                Cell::new("Market Value of Debt (D)"),
                Cell::new(&format_currency(wacc.market_value_debt)),
            ]);
            
            table.add_row(vec![
                Cell::new("WACC"),
                Cell::new(&format!("{:.2}%", wacc_value * 100.0)).fg(Color::Green),
            ]);
            
            // Print the table
            println!("{table}");
            
            info!("WACC calculation completed: {:.4}%", wacc_value * 100.0);
            Ok(())
        }
        // This case is already handled above
        Command::Variance(variance) => {
            debug!("Calculating variance with: {:?}", variance);
            
            // Convert string numbers to f64
            let numbers: Result<Vec<f64>, _> = variance.numbers.iter()
                .map(|n| n.parse::<f64>())
                .collect();
            
            let numbers = match numbers {
                Ok(nums) => nums,
                Err(_) => return Err(FinanceError::InvalidInput("Invalid numbers provided".into()).into()),
            };
            
            if numbers.len() <= 1 {
                return Err(FinanceError::InvalidInput("At least two numbers are required to calculate variance".into()).into());
            }
            
            // Calculate variance
            let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;
            let sum_squared_diff: f64 = numbers.iter()
                .map(|&x| (x - mean).powi(2))
                .sum();
            let variance_value = sum_squared_diff / numbers.len() as f64;
            
            // Create and format table
            let mut table = create_table(vec!["Number", "Difference from Mean", "Squared Difference"]);
            
            // Add each number to the table
            for &number in &numbers {
                let diff = number - mean;
                let squared_diff = diff.powi(2);
                
                table.add_row(vec![
                    Cell::new(&format!("{:.2}", number)),
                    Cell::new(&format!("{:.2}", diff)),
                    Cell::new(&format!("{:.2}", squared_diff)),
                ]);
            }
            
            // Add variance row
            table.add_row(vec![
                Cell::new("Mean:").fg(Color::Blue),
                Cell::new(&format!("{:.2}", mean)).fg(Color::Blue),
                Cell::new(""),
            ]);
            
            table.add_row(vec![
                Cell::new("Variance:").fg(Color::Green),
                Cell::new(""),
                Cell::new(&format!("{:.2}", variance_value)).fg(Color::Green),
            ]);
            
            // Print the table
            println!("{table}");
            
            info!("Variance calculation completed: {:.4}", variance_value);
            Ok(())
        }
        Command::StandardDeviation(std_dev) => {
            debug!("Calculating standard deviation with: {:?}", std_dev);
            
            if std_dev.numbers.len() <= 1 {
                return Err(FinanceError::InvalidInput("At least two numbers are required to calculate standard deviation".into()).into());
            }
            
            // Calculate standard deviation
            let mean = std_dev.numbers.iter().sum::<f64>() / std_dev.numbers.len() as f64;
            let sum_squared_diff: f64 = std_dev.numbers.iter()
                .map(|&x| (x - mean).powi(2))
                .sum();
            let variance = sum_squared_diff / (std_dev.numbers.len() - 1) as f64;
            let std_dev_value = variance.sqrt();
            
            // Create and format table
            let mut table = create_table(vec!["Number", "Difference from Mean", "Squared Difference"]);
            
            // Add each number to the table
            for &number in &std_dev.numbers {
                let diff = number - mean;
                let squared_diff = diff.powi(2);
                
                table.add_row(vec![
                    Cell::new(&format!("{:.2}", number)),
                    Cell::new(&format!("{:.2}", diff)),
                    Cell::new(&format!("{:.2}", squared_diff)),
                ]);
            }
            
            // Add summary rows
            table.add_row(vec![
                Cell::new("Mean:").fg(Color::Blue),
                Cell::new(&format!("{:.2}", mean)).fg(Color::Blue),
                Cell::new(""),
            ]);
            
            table.add_row(vec![
                Cell::new("Variance:").fg(Color::Blue),
                Cell::new(""),
                Cell::new(&format!("{:.2}", variance)).fg(Color::Blue),
            ]);
            
            table.add_row(vec![
                Cell::new("Standard Deviation:").fg(Color::Green),
                Cell::new(""),
                Cell::new(&format!("{:.2}", std_dev_value)).fg(Color::Green),
            ]);
            
            // Print the table
            println!("{table}");
            
            info!("Standard deviation calculation completed: {:.4}", std_dev_value);
            Ok(())
        }
        Command::Probability(probability) => {
            debug!("Calculating probability with: {:?}", probability);
            
            // Validate inputs
            if probability.trials == 0 {
                return Err(FinanceError::DivisionByZero.into());
            }
            
            if probability.successes > probability.trials {
                return Err(FinanceError::InvalidInput("Successes cannot be greater than trials".into()).into());
            }
            
            // Calculate probability
            let probability_value = probability.successes as f64 / probability.trials as f64;
            
            // Create and format table
            let mut table = create_table(vec!["Successes", "Trials", "Probability"]);
            
            // Add row
            table.add_row(vec![
                Cell::new(&format!("{}", probability.successes)),
                Cell::new(&format!("{}", probability.trials)),
                Cell::new(&format!("{:.2}%", probability_value * 100.0)).fg(Color::Green),
            ]);
            
            // Print the table
            println!("{table}");
            
            info!("Probability calculation completed: {:.4}", probability_value);
            Ok(())
        }
        Command::CAPM(capm) => {
            debug!("Calculating CAPM with: {:?}", capm);
            
            // Validate inputs
            if !capm.risk_free_rate.is_finite() || !capm.beta.is_finite() || !capm.market_return.is_finite() {
                return Err(FinanceError::InvalidInput("All inputs must be valid numbers".into()).into());
            }
            
            // Calculate CAPM
            let expected_return = capm.risk_free_rate + capm.beta * (capm.market_return - capm.risk_free_rate);
            
            // Create and format table
            let mut table = create_table(vec!["Component", "Value"]);
            
            // Add rows
            table.add_row(vec![
                Cell::new("Risk-Free Rate"),
                Cell::new(&format!("{:.2}%", capm.risk_free_rate * 100.0)),
            ]);
            
            table.add_row(vec![
                Cell::new("Beta"),
                Cell::new(&format!("{:.2}", capm.beta)),
            ]);
            
            table.add_row(vec![
                Cell::new("Market Return"),
                Cell::new(&format!("{:.2}%", capm.market_return * 100.0)),
            ]);
            
            table.add_row(vec![
                Cell::new("Expected Return (CAPM)"),
                Cell::new(&format!("{:.2}%", expected_return * 100.0)).fg(Color::Green),
            ]);
            
            // Print the table
            println!("{table}");
            
            info!("CAPM calculation completed: {:.4}%", expected_return * 100.0);
            Ok(())
        }
        Command::LoanPayment(loan) => {
            debug!("Calculating loan payment with: {:?}", loan);
            
            // Validate inputs
            if !loan.principal.is_finite() || !loan.interest_rate.is_finite() || !loan.loan_term.is_finite() {
                return Err(FinanceError::InvalidInput("All inputs must be valid numbers".into()).into());
            }
            
            if loan.principal <= 0.0 {
                return Err(FinanceError::InvalidInput("Principal must be positive".into()).into());
            }
            
            if loan.interest_rate <= 0.0 {
                return Err(FinanceError::InvalidInput("Interest rate must be positive".into()).into());
            }
            
            if loan.loan_term <= 0.0 {
                return Err(FinanceError::InvalidInput("Loan term must be positive".into()).into());
            }
            
            // Calculate monthly payment
            let monthly_rate = loan.interest_rate / 100.0 / 12.0;
            let num_payments = loan.loan_term * 12.0;
            let monthly_payment = (loan.principal * monthly_rate) / (1.0 - (1.0 + monthly_rate).powf(-num_payments));
            
            // Calculate total payment
            let total_payment = monthly_payment * num_payments;
            let total_interest = total_payment - loan.principal;
            
            // Calculate payoff date
            let current_date = Local::now().naive_local().date();
            let months_to_add = num_payments as i32;
            let payoff_date = current_date + Months::new(months_to_add as u32);
            
            // Create and format table
            let mut table = create_table(vec!["Component", "Value"]);
            
            // Add rows
            table.add_row(vec![
                Cell::new("Principal"),
                Cell::new(&format_currency(loan.principal)),
            ]);
            
            table.add_row(vec![
                Cell::new("Annual Interest Rate"),
                Cell::new(&format!("{:.2}%", loan.interest_rate)),
            ]);
            
            table.add_row(vec![
                Cell::new("Loan Term"),
                Cell::new(&format!("{:.1} years", loan.loan_term)),
            ]);
            
            table.add_row(vec![
                Cell::new("Monthly Payment"),
                Cell::new(&format_currency(monthly_payment)).fg(Color::Green),
            ]);
            
            table.add_row(vec![
                Cell::new("Total Interest"),
                Cell::new(&format_currency(total_interest)).fg(Color::Green),
            ]);
            
            table.add_row(vec![
                Cell::new("Payoff Date"),
                Cell::new(&payoff_date.format("%Y-%m-%d").to_string()).fg(Color::Green),
            ]);
            
            // Print the table
            println!("{table}");
            
            info!("Loan payment calculation completed. Monthly payment: {:.2}", monthly_payment);
            Ok(())
        }
        Command::BreakEvenUnits(break_even) => {
            debug!("Calculating break-even units with: {:?}", break_even);
            
            // Validate inputs
            if !break_even.fixed_costs.is_finite() || !break_even.variable_costs.is_finite() || !break_even.price_per_unit.is_finite() {
                return Err(FinanceError::InvalidInput("All inputs must be valid numbers".into()).into());
            }
            
            if break_even.fixed_costs < 0.0 {
                return Err(FinanceError::InvalidInput("Fixed costs must be non-negative".into()).into());
            }
            
            if break_even.variable_costs < 0.0 {
                return Err(FinanceError::InvalidInput("Variable costs must be non-negative".into()).into());
            }
            
            if break_even.price_per_unit <= 0.0 {
                return Err(FinanceError::InvalidInput("Price per unit must be positive".into()).into());
            }
            
            // Check for division by zero
            if break_even.price_per_unit - break_even.variable_costs == 0.0 {
                return Err(FinanceError::DivisionByZero.into());
            }
            
            // Calculate break-even point
            let break_even_units = break_even.fixed_costs / (break_even.price_per_unit - break_even.variable_costs);
            let total_revenue = break_even.price_per_unit * break_even_units;
            
            // Create and format table
            let mut table = create_table(vec!["Component", "Value"]);
            
            // Add rows
            table.add_row(vec![
                Cell::new("Fixed Costs"),
                Cell::new(&format_currency(break_even.fixed_costs)),
            ]);
            
            table.add_row(vec![
                Cell::new("Variable Costs Per Unit"),
                Cell::new(&format_currency(break_even.variable_costs)),
            ]);
            
            table.add_row(vec![
                Cell::new("Price Per Unit"),
                Cell::new(&format_currency(break_even.price_per_unit)),
            ]);
            
            table.add_row(vec![
                Cell::new("Contribution Margin Per Unit"),
                Cell::new(&format_currency(break_even.price_per_unit - break_even.variable_costs)),
            ]);
            
            table.add_row(vec![
                Cell::new("Break-Even Point (Units)"),
                Cell::new(&format!("{:.0}", break_even_units)).fg(Color::Green),
            ]);
            
            table.add_row(vec![
                Cell::new("Total Revenue at Break-Even"),
                Cell::new(&format_currency(total_revenue)).fg(Color::Green),
            ]);
            
            // Print the table
            println!("{table}");
            
            info!("Break-even units calculation completed: {:.0} units", break_even_units);
            Ok(())
        }
        Command::Mortgage(mortgage) => {
            debug!("Calculating mortgage with: {:?}", mortgage);
            
            // Validate inputs
            if !mortgage.loan_amount.is_finite() || !mortgage.interest_rate.is_finite() {
                return Err(FinanceError::InvalidInput("All inputs must be valid numbers".into()).into());
            }
            
            if mortgage.loan_amount <= 0.0 {
                return Err(FinanceError::InvalidInput("Loan amount must be positive".into()).into());
            }
            
            if mortgage.interest_rate <= 0.0 {
                return Err(FinanceError::InvalidInput("Interest rate must be positive".into()).into());
            }
            
            if mortgage.term <= 0 {
                return Err(FinanceError::InvalidInput("Term must be positive".into()).into());
            }
            
            // Calculate monthly payment
            let monthly_interest_rate = mortgage.interest_rate / 12.0 / 100.0;
            let loan_term_months = mortgage.term * 12;
            
            let monthly_payment = (mortgage.loan_amount * monthly_interest_rate) /
                (1.0 - (1.0 + monthly_interest_rate).powi(-loan_term_months));
            
            // Calculate total payment
            let total_payment = monthly_payment * loan_term_months as f64;
            let total_interest = total_payment - mortgage.loan_amount;
            
            // Calculate payoff date
            let current_date = Local::now().naive_local().date();
            let payoff_date = current_date + Months::new(loan_term_months as u32);
            
            // Create and format table
            let mut table = create_table(vec!["Component", "Value"]);
            
            // Add rows
            table.add_row(vec![
                Cell::new("Loan Amount"),
                Cell::new(&format_currency(mortgage.loan_amount)),
            ]);
            
            table.add_row(vec![
                Cell::new("Annual Interest Rate"),
                Cell::new(&format!("{:.2}%", mortgage.interest_rate)),
            ]);
            
            table.add_row(vec![
                Cell::new("Loan Term"),
                Cell::new(&format!("{} years", mortgage.term)),
            ]);
            
            table.add_row(vec![
                Cell::new("Monthly Payment"),
                Cell::new(&format_currency(monthly_payment)).fg(Color::Green),
            ]);
            
            table.add_row(vec![
                Cell::new("Total Interest"),
                Cell::new(&format_currency(total_interest)).fg(Color::Green),
            ]);
            
            table.add_row(vec![
                Cell::new("Payoff Date"),
                Cell::new(&payoff_date.format("%Y-%m-%d").to_string()).fg(Color::Green),
            ]);
            
            // Print the table
            println!("{table}");
            
            info!("Mortgage calculation completed");
            Ok(())
        }
        Command::DCF(dcf) => {
            debug!("Calculating DCF with: {:?}", dcf);
            
            // Validate inputs
            if !dcf.discount_rate.is_finite() {
                return Err(FinanceError::InvalidInput("Discount rate must be a valid number".into()).into());
            }
            
            if dcf.cash_flows.is_empty() {
                return Err(FinanceError::InvalidInput("No cash flows provided".into()).into());
            }
            
            if dcf.discount_rate <= -1.0 {
                return Err(FinanceError::InvalidInput("Discount rate must be greater than -1".into()).into());
            }
            
            // Calculate DCF
            let mut present_values = Vec::new();
            
            for (i, &cash_flow) in dcf.cash_flows.iter().enumerate() {
                let present_value = cash_flow / (1.0 + dcf.discount_rate).powf((i + 1) as f64);
                present_values.push(present_value);
            }
            
            let dcf_value: f64 = present_values.iter().sum();
            
            // Create and format table
            let mut table = create_table(vec!["Year", "Cash Flow", "Present Value"]);
            
            // Add rows for each year
            for (i, (&cash_flow, &present_value)) in dcf.cash_flows.iter().zip(present_values.iter()).enumerate() {
                table.add_row(vec![
                    Cell::new(&format!("{}", i + 1)),
                    Cell::new(&format_currency(cash_flow)),
                    Cell::new(&format_currency(present_value)),
                ]);
            }
            
            // Add total row
            table.add_row(vec![
                Cell::new("Total").fg(Color::Green),
                Cell::new(""),
                Cell::new(&format_currency(dcf_value)).fg(Color::Green),
            ]);
            
            // Print the table
            println!("{table}");
            
            info!("DCF calculation completed: {:.2}", dcf_value);
            Ok(())
        }
        Command::WeightedAverage(wa) => {
            debug!("Calculating weighted average with: {:?}", wa);
            
            // Parse the numbers and weights
            let numbers: Result<Vec<f64>, _> = wa.numbers.split_whitespace().map(str::parse).collect();
            let weights: Result<Vec<f64>, _> = wa.weights.split_whitespace().map(str::parse).collect();
            
            let numbers = match numbers {
                Ok(nums) => nums,
                Err(_) => return Err(FinanceError::InvalidInput("Invalid numbers provided".into()).into()),
            };
            
            let weights = match weights {
                Ok(wts) => wts,
                Err(_) => return Err(FinanceError::InvalidInput("Invalid weights provided".into()).into()),
            };
            
            if numbers.is_empty() || weights.is_empty() {
                return Err(FinanceError::InvalidInput("Numbers and weights cannot be empty".into()).into());
            }
            
            if numbers.len() != weights.len() {
                return Err(FinanceError::InvalidInput("Number of numbers and weights should be the same".into()).into());
            }
            
            // Calculate weighted average
            let mut sum = 0.0;
            let mut total_weight = 0.0;
            
            for (number, weight) in numbers.iter().zip(weights.iter()) {
                sum += number * weight;
                total_weight += weight;
            }
            
            if total_weight == 0.0 {
                return Err(FinanceError::DivisionByZero.into());
            }
            
            let weighted_avg = sum / total_weight;
            
            // Create and format table
            let mut table = create_table(vec!["Number", "Weight", "Weighted Value"]);
            
            // Add rows for each number/weight pair
            for (&number, &weight) in numbers.iter().zip(weights.iter()) {
                let weighted_value = number * weight;
                
                table.add_row(vec![
                    Cell::new(&format!("{:.2}", number)),
                    Cell::new(&format!("{:.2}", weight)),
                    Cell::new(&format!("{:.2}", weighted_value)),
                ]);
            }
            
            // Add total row
            table.add_row(vec![
                Cell::new(""),
                Cell::new(&format!("Total: {:.2}", total_weight)).fg(Color::Blue),
                Cell::new(&format!("Sum: {:.2}", sum)).fg(Color::Blue),
            ]);
            
            // Add weighted average row
            table.add_row(vec![
                Cell::new("Weighted Average").fg(Color::Green),
                Cell::new(""),
                Cell::new(&format!("{:.2}", weighted_avg)).fg(Color::Green),
            ]);
            
            // Print the table
            println!("{table}");
            
            info!("Weighted average calculation completed: {:.4}", weighted_avg);
            Ok(())
        }
        Command::ReturnOnEquity(roe) => {
            roe.execute()
        },
        Command::Completion(completion) => {
            debug!("Generating shell completions for: {:?}", completion.shell);
            
            let mut app = Opts::command();
            let app_name = app.get_name().to_string();
            
            match completion.shell {
                Shell::Bash => generate(Bash, &mut app, &app_name, &mut io::stdout()),
                Shell::Fish => generate(Fish, &mut app, &app_name, &mut io::stdout()),
                Shell::Zsh => generate(Zsh, &mut app, &app_name, &mut io::stdout()),
                Shell::PowerShell => generate(PowerShell, &mut app, &app_name, &mut io::stdout()),
            }
            
            info!("Shell completions generated successfully");
            Ok(())
        },
        _ => {
            // Handle any other commands that might be added in the future
            Err(anyhow::anyhow!("This command hasn't been implemented in the modernized version yet"))
        }
    }
}

/// Application entry point with error handling
fn main() {
    // Run the application and handle any errors
    if let Err(err) = run() {
        // Print the error and its causes
        eprintln!("{}: {}", "Error".red().bold(), err);
        
        // Print the error chain for better debugging
        let mut cause = err.source();
        while let Some(source_err) = cause {
            eprintln!("  {}: {}", "Caused by".yellow().bold(), source_err);
            cause = source_err.source();
        }
        
        // Exit with a non-zero status code
        std::process::exit(1);
    }
}