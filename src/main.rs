use anyhow::{Context, Result};
use chrono::{Local, Months};

use clap::{CommandFactory, Parser};
use clap_complete::{generate, shells::{Bash, Fish, Zsh, PowerShell}};
use std::io;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use comfy_table::{Cell, CellAlignment, Color, ContentArrangement, Table};
use dialoguer::{Input, Select, theme::ColorfulTheme};

use env_logger::Env;
use log::{debug, info, warn};
use owo_colors::OwoColorize;
use rust_decimal::{Decimal, prelude::FromPrimitive};

// Import from rusty_finance library
use rusty_finance::FinanceError;
use rusty_finance::calculations::*;
use rusty_finance::display::*;


/// Financial calculation tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Opts {
    /// Verbosity level for logging
    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,
    
    /// Run in interactive mode with prompts for inputs
    #[clap(short, long)]
    interactive: bool,
    
    #[clap(subcommand)]
    command: Option<Command>,
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
    
    // Handle interactive mode or regular command mode
    let command = if opts.interactive {
        show_interactive_menu()?
    } else {
        match opts.command {
            Some(cmd) => cmd,
            None => {
                eprintln!("Error: No command specified. Use --interactive for interactive mode or specify a command.");
                eprintln!("Run 'rusty-finance --help' for usage information.");
                return Ok(());
            }
        }
    };
    
    debug!("Command type: {}", match &command {
        Command::Interest(_) => "Interest",
        Command::CompoundInterest(_) => "CompoundInterest", 
        Command::PresentValue(_) => "PresentValue",
        Command::FutureValue(_) => "FutureValue",
        Command::NPV(_) => "NPV",
        Command::Amortization(_) => "Amortization",
        Command::ROI(_) => "ROI",
        Command::Average(_) => "Average",
        Command::Mode(_) => "Mode",
        Command::Medium(_) => "Median",
        Command::PaybackPeriod(_) => "PaybackPeriod",
        Command::BreakEven(_) => "BreakEven",
        Command::Depreciation(_) => "Depreciation",
        Command::IRR(_) => "IRR",
        Command::Variance(_) => "Variance",
        Command::StandardDeviation(_) => "StandardDeviation",
        Command::Probability(_) => "Probability",
        Command::CAPM(_) => "CAPM",
        Command::LoanPayment(_) => "LoanPayment",
        Command::BreakEvenUnits(_) => "BreakEvenUnits",
        Command::DCF(_) => "DCF",
        Command::Mortgage(_) => "Mortgage",
        Command::WeightedAverage(_) => "WeightedAverage",
        Command::WACC(_) => "WACC",
        Command::DividendYield(_) => "DividendYield",
        Command::ReturnOnEquity(_) => "ReturnOnEquity",
        Command::Completion(_) => "Completion",
    });
    
    // Execute the selected command
    match command {
        Command::Interest(interest) => {
            debug!("Calculating simple interest");
            
            let result = calculate_simple_interest(interest.principal, interest.rate, interest.time)
                .context("Failed to calculate simple interest")?;
            
            info!("Calculated simple interest: {:.4}", result);
            
            // Create table using dynamic helper
            let mut table = create_table(vec!["Principal", "Rate", "Time", "Simple Interest"]);
            
            // Add row with dynamic alignment - no manual padding needed
            add_row(&mut table, &[
                (&format_currency_plain(interest.principal), CellAlignment::Right),
                (&format_rate_as_percentage(interest.rate), CellAlignment::Right),
                (&format_years(interest.time), CellAlignment::Right),
                (&format_currency_plain(result), CellAlignment::Right),
            ]);
            
            println!("{table}");
            info!("Simple interest calculation completed");
            Ok(())
        }
        Command::CompoundInterest(ci) => {
            debug!("Calculating compound interest with: {:?}", ci);
            
            // Create table using dynamic helper
            let mut table = create_table(vec!["Year", "Amount"]);
            
            // Calculate compound interest for each year
            // Interactive mode already converts percentage to decimal, CLI mode needs conversion
            let rate = if ci.rate > 1.0 { ci.rate / 100.0 } else { ci.rate };
            for year in 1..=ci.t {
                let amount = calculate_compound_interest(ci.principal, rate, ci.n, year)
                    .context("Failed to calculate compound interest")?;
                
                // Add row with dynamic alignment - no manual padding needed
                add_row(&mut table, &[
                    (&format!("{}", year), CellAlignment::Center),
                    (&format_currency_plain(amount), CellAlignment::Right),
                ]);
            }
            
            println!("{table}");
            info!("Compound interest calculation completed");
            Ok(())
        }
        Command::PresentValue(pv) => {
            debug!("Calculating present value with: {:?}", pv);
            
            let result = calculate_present_value(pv.future_value, pv.rate, pv.time)
                .context("Failed to calculate present value")?;
            
            info!("Calculated present value: {:.4}", result);
            
            let mut table = create_table(vec!["Future Value", "Rate", "Time", "Present Value"]);
            
            add_row(&mut table, &[
                (&format_currency_plain(pv.future_value), CellAlignment::Right),
                (&format_rate_as_percentage(pv.rate), CellAlignment::Right),
                (&format_years(pv.time), CellAlignment::Right),
                (&format_currency_plain(result), CellAlignment::Right),
            ]);
            
            println!("{table}");
            info!("Present value calculation completed");
            Ok(())
        }
        Command::FutureValue(fv) => {
            debug!("Calculating future value with: {:?}", fv);
            
            let result = calculate_future_value(fv.present_value, fv.rate, fv.time)
                .context("Failed to calculate future value")?;
            
            info!("Calculated future value: {:.4}", result);
            
            let mut table = create_table(vec!["Present Value", "Rate", "Time", "Future Value"]);
            
            add_row(&mut table, &[
                (&format_currency_plain(fv.present_value), CellAlignment::Right),
                (&format_rate_as_percentage(fv.rate), CellAlignment::Right),
                (&format_years(fv.time), CellAlignment::Right),
                (&format_currency_plain(result), CellAlignment::Right),
            ]);
            
            println!("{table}");
            info!("Future value calculation completed");
            Ok(())
        }
        Command::NPV(npv) => {
            debug!("Calculating NPV with: {:?}", npv);
            
            // Create cash flows vector from the NPV inputs
            let cash_flows: Vec<f64> = (0..npv.lifespan).map(|_| npv.cash_inflow).collect();
            
            let npv_value = calculate_npv(npv.initial_investment, &cash_flows, npv.discount_rate)
                .context("Failed to calculate NPV")?;
            
            // Create and format table
            let mut table = create_table(vec!["Year", "Cash Inflow", "Discounted Cash Flow"]);
            
            // Calculate and display each year's discounted cash flow
            for year in 1..=npv.lifespan {
                let discounted_cash_flow = npv.cash_inflow / (1.0 + npv.discount_rate).powf(year as f64);
                
                add_row(&mut table, &[
                    (&format!("{}", year), CellAlignment::Center),
                    (&format_currency_plain(npv.cash_inflow), CellAlignment::Right),
                    (&format_currency_plain(discounted_cash_flow), CellAlignment::Right),
                ]);
            }
            
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
            
            let roi_value = calculate_roi(roi.net_profit, roi.cost_of_investment)
                .context("Failed to calculate ROI")?;
            
            info!("Calculated ROI: {:.4}%", roi_value);
            
            let mut table = create_table(vec!["Net Profit", "Cost of Investment", "ROI"]);
            
            add_row(&mut table, &[
                (&format_currency_plain(roi.net_profit), CellAlignment::Right),
                (&format_currency_plain(roi.cost_of_investment), CellAlignment::Right),
                (&format_percentage_plain(roi_value / 100.0, 2), CellAlignment::Right),
            ]);
            
            println!("{table}");
            info!("ROI calculation completed");
            Ok(())
        }
        Command::Average(average) => {
            debug!("Calculating average with: {:?}", average);
            
            let avg = calculate_mean(&average.numbers)
                .context("Failed to calculate average")?;
            
            let mut table = create_table(vec!["Number"]);
            
            for number in &average.numbers {
                add_row(&mut table, &[
                    (&format!("{:.2}", number), CellAlignment::Right),
                ]);
            }
            
            add_row(&mut table, &[
                (&format!("Average: {:.2}", avg), CellAlignment::Right),
            ]);
            
            println!("{table}");
            info!("Average calculation completed: {:.4}", avg);
            Ok(())
        }
        Command::Mode(mode) => {
            debug!("Calculating mode with: {:?}", mode);
            
            let mode_value = calculate_mode(&mode.numbers)
                .context("Failed to calculate mode")?;
            
            let mut table = create_table(vec!["Number", "Mode"]);
            
            for number in &mode.numbers {
                add_row(&mut table, &[
                    (&format!("{:.2}", number), CellAlignment::Right),
                    ("", CellAlignment::Left),
                ]);
            }
            
            match mode_value {
                Some(m) => add_row(&mut table, &[
                    ("Mode:", CellAlignment::Left),
                    (&format!("{:.2}", m), CellAlignment::Right),
                ]),
                None => add_row(&mut table, &[
                    ("Mode:", CellAlignment::Left),
                    ("No mode", CellAlignment::Left),
                ]),
            };
            
            println!("{table}");
            info!("Mode calculation completed");
            Ok(())
        }
        Command::Medium(medium) => {
            debug!("Calculating median with: {:?}", medium);
            
            let median = calculate_median(&medium.numbers)
                .context("Failed to calculate median")?;
            
            let mut table = create_table(vec!["Number"]);
            
            // Sort for display
            let mut sorted = medium.numbers.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            
            for number in &sorted {
                add_row(&mut table, &[
                    (&format!("{:.2}", number), CellAlignment::Right),
                ]);
            }
            
            add_row(&mut table, &[
                (&format!("Median: {:.2}", median), CellAlignment::Right),
            ]);
            
            println!("{table}");
            info!("Median calculation completed: {:.4}", median);
            Ok(())
        }
        Command::PaybackPeriod(payback) => {
            debug!("Calculating payback period with: {:?}", payback);
            
            let payback_period = calculate_payback_period(payback.initial_cost, &payback.cash_flows)
                .context("Failed to calculate payback period")?;
            
            let mut table = create_table(vec!["Cash Flows", "Initial Cost", "Payback Period"]);
            
            let payback_str = match payback_period {
                Some(period) => format!("{:.2} years", period),
                None => "Never pays back".to_string(),
            };
            
            add_row(&mut table, &[
                (&format!("{:?}", payback.cash_flows), CellAlignment::Left),
                (&format_currency_plain(payback.initial_cost), CellAlignment::Right),
                (&payback_str, CellAlignment::Right),
            ]);
            
            println!("{table}");
            info!("Payback period calculation completed");
            Ok(())
        }
        Command::BreakEven(break_even) => {
            debug!("Calculating break-even analysis with: {:?}", break_even);
            
            let (break_even_units, break_even_revenue) = calculate_break_even_analysis(
                break_even.fixed_costs,
                break_even.variable_costs,
                break_even.price_per_unit
            ).context("Failed to calculate break-even analysis")?;
            
            let summary_items = vec![
                ("Break-Even Point (units)", format!("{:.0}", break_even_units)),
                ("Total Revenue Required", format_currency(break_even_revenue)),
            ];
            
            let table = create_summary_table("Metric", summary_items);
            println!("{table}");
            
            info!("Break-even analysis completed");
            Ok(())
        }
        Command::LoanPayment(loan) => {
            debug!("Calculating loan payment with: {:?}", loan);
            
            let monthly_payment = calculate_loan_payment(loan.principal, loan.interest_rate, loan.loan_term)
                .context("Failed to calculate loan payment")?;
            
            let total_payment = monthly_payment * loan.loan_term * 12.0;
            let total_interest = total_payment - loan.principal;
            
            // Calculate payoff date
            let current_date = Local::now().naive_local().date();
            let months_to_add = (loan.loan_term * 12.0) as u32;
            let payoff_date = current_date + Months::new(months_to_add);
            
            let summary_items = vec![
                ("Principal", format_currency(loan.principal)),
                ("Annual Interest Rate", format_rate_as_percentage(loan.interest_rate / 100.0)),
                ("Loan Term", format_years(loan.loan_term)),
                ("Monthly Payment", format_currency(monthly_payment)),
                ("Total Interest", format_currency(total_interest)),
                ("Payoff Date", payoff_date.format("%Y-%m-%d").to_string()),
            ];
            
            let table = create_summary_table("Component", summary_items);
            println!("{table}");
            
            info!("Loan payment calculation completed. Monthly payment: {:.2}", monthly_payment);
            Ok(())
        }
        Command::Amortization(amortization) => {
            debug!("Calculating amortization schedule with: {:?}", amortization);
            
            let schedule = generate_amortization_schedule(
                amortization.loan_amount,
                amortization.annual_interest_rate,
                amortization.loan_term_years
            ).context("Failed to generate amortization schedule")?;
            
            let mut table = create_table(vec!["Month", "Principal", "Interest", "Remaining Balance"]);
            
            // Show selected payments (first, every 12th, and last)
            for payment in &schedule {
                if payment.month == 1 || payment.month % 12 == 0 || payment.month == schedule.len() as u32 {
                    add_row(&mut table, &[
                        (&format!("{}", payment.month), CellAlignment::Center),
                        (&format_currency_plain(payment.principal_payment), CellAlignment::Right),
                        (&format_currency_plain(payment.interest_payment), CellAlignment::Right),
                        (&format_currency_plain(payment.remaining_balance), CellAlignment::Right),
                    ]);
                }
            }
            
            println!("{table}");
            info!("Amortization calculation completed");
            Ok(())
        }
        Command::ReturnOnEquity(roe) => {
            debug!("Calculating ROE with: {:?}", roe);
            
            let roe_value = calculate_roe(roe.net_income, roe.equity)
                .context("Failed to calculate ROE")?;
            
            info!("Calculated ROE: {:.4}%", roe_value);
            
            let mut table = create_table(vec!["Net Income", "Equity", "Return on Equity"]);
            
            add_row(&mut table, &[
                (&format_currency_plain(roe.net_income), CellAlignment::Right),
                (&format_currency_plain(roe.equity), CellAlignment::Right),
                (&format_percentage_plain(roe_value / 100.0, 2), CellAlignment::Right),
            ]);
            
            println!("{table}");
            info!("ROE calculation completed successfully");
            Ok(())
        }
        Command::DividendYield(dividend_yield) => {
            debug!("Calculating dividend yield with: {:?}", dividend_yield);
            
            let result = calculate_dividend_yield(&dividend_yield)
                .context("Failed to calculate dividend yield")?;
            
            info!("Calculated dividend yield: {:.4}", result);
            
            let mut table = create_table(vec!["Dividend", "Price", "Dividend Yield"]);
            
            add_row(&mut table, &[
                (&format!("{:.2}", dividend_yield.dividend), CellAlignment::Right),
                (&format!("{:.2}", dividend_yield.price), CellAlignment::Right),
                (&format_percentage_plain(result, 2), CellAlignment::Right),
            ]);
            
            println!("{table}");
            info!("Dividend yield calculation completed");
            Ok(())
        }
        Command::CAPM(capm) => {
            debug!("Calculating CAPM with: {:?}", capm);
            
            let expected_return = calculate_capm(capm.risk_free_rate, capm.beta, capm.market_return)
                .context("Failed to calculate CAPM")?;
            
            let summary_items = vec![
                ("Risk-Free Rate", format_rate_as_percentage(capm.risk_free_rate)),
                ("Beta", format!("{:.2}", capm.beta)),
                ("Market Return", format_rate_as_percentage(capm.market_return)),
                ("Expected Return (CAPM)", format_rate_as_percentage(expected_return)),
            ];
            
            let table = create_summary_table("Component", summary_items);
            println!("{table}");
            
            info!("CAPM calculation completed: {:.4}%", expected_return * 100.0);
            Ok(())
        }
        Command::WACC(wacc) => {
            debug!("Calculating WACC with: {:?}", wacc);
            
            let wacc_value = calculate_wacc(
                wacc.cost_of_equity,
                wacc.cost_of_debt,
                wacc.tax_rate,
                wacc.market_value_equity,
                wacc.market_value_debt
            ).context("Failed to calculate WACC")?;
            
            let summary_items = vec![
                ("Cost of Equity (Ke)", format_rate_as_percentage(wacc.cost_of_equity)),
                ("Cost of Debt (Kd)", format_rate_as_percentage(wacc.cost_of_debt)),
                ("Tax Rate", format_rate_as_percentage(wacc.tax_rate)),
                ("Market Value of Equity (E)", format_currency(wacc.market_value_equity)),
                ("Market Value of Debt (D)", format_currency(wacc.market_value_debt)),
                ("WACC", format_rate_as_percentage(wacc_value)),
            ];
            
            let table = create_summary_table("Component", summary_items);
            println!("{table}");
            
            info!("WACC calculation completed: {:.4}%", wacc_value * 100.0);
            Ok(())
        }

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

// Interactive mode helper functions

/// Prompt for a floating point number with validation
fn prompt_f64(message: &str) -> Result<f64> {
    let theme = ColorfulTheme::default();
    loop {
        let input: String = Input::with_theme(&theme)
            .with_prompt(message)
            .interact()?;
        
        match input.parse::<f64>() {
            Ok(value) if value.is_finite() => return Ok(value),
            Ok(_) => println!("Please enter a valid finite number."),
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

/// Prompt for a positive floating point number with validation
fn prompt_positive_f64(message: &str) -> Result<f64> {
    loop {
        let value = prompt_f64(message)?;
        if value > 0.0 {
            return Ok(value);
        }
        println!("Please enter a positive number.");
    }
}

/// Prompt for a non-negative floating point number with validation
fn prompt_non_negative_f64(message: &str) -> Result<f64> {
    loop {
        let value = prompt_f64(message)?;
        if value >= 0.0 {
            return Ok(value);
        }
        println!("Please enter a non-negative number.");
    }
}

/// Prompt for a percentage (0-100) and convert to decimal
fn prompt_percentage(message: &str) -> Result<f64> {
    let theme = ColorfulTheme::default();
    loop {
        let input: String = Input::with_theme(&theme)
            .with_prompt(format!("{} (enter as percentage, e.g., 5.0 for 5%)", message))
            .interact()?;
        
        match input.parse::<f64>() {
            Ok(value) if value.is_finite() && value >= 0.0 && value <= 100.0 => {
                return Ok(value / 100.0);
            },
            Ok(_) => println!("Please enter a percentage between 0 and 100."),
            Err(_) => println!("Please enter a valid percentage."),
        }
    }
}

/// Prompt for a list of numbers (comma-separated)
fn prompt_number_list(message: &str) -> Result<Vec<f64>> {
    let theme = ColorfulTheme::default();
    loop {
        let input: String = Input::with_theme(&theme)
            .with_prompt(format!("{} (comma-separated, e.g., 1.0,2.0,3.0)", message))
            .interact()?;
        
        let numbers: Result<Vec<f64>, _> = input
            .split(',')
            .map(|s| s.trim().parse::<f64>())
            .collect();
        
        match numbers {
            Ok(nums) if nums.iter().all(|&n| n.is_finite()) => {
                if nums.is_empty() {
                    println!("Please enter at least one number.");
                    continue;
                }
                return Ok(nums);
            },
            Ok(_) => println!("All numbers must be finite."),
            Err(_) => println!("Please enter valid numbers separated by commas."),
        }
    }
}

/// Show interactive menu and return selected command
fn show_interactive_menu() -> Result<Command> {
    let theme = ColorfulTheme::default();
    
    println!("\n{}", "=== Rusty Finance Calculator ===".bold().cyan());
    println!("Select a calculation:");
    
    let options = vec![
        "Simple Interest",
        "Compound Interest", 
        "Present Value",
        "Future Value",
        "Net Present Value (NPV)",
        "Amortization Schedule",
        "Return on Investment (ROI)",
        "Average",
        "Mode",
        "Median",
        "Payback Period",
        "Break-Even Analysis",
        "Depreciation",
        "Internal Rate of Return (IRR)",
        "Variance",
        "Standard Deviation",
        "Probability",
        "CAPM",
        "Loan Payment",
        "Break-Even Units",
        "Discounted Cash Flow (DCF)",
        "Mortgage",
        "Weighted Average",
        "WACC",
        "Dividend Yield",
        "Return on Equity (ROE)",
    ];
    
    let selection = Select::with_theme(&theme)
        .with_prompt("Choose calculation")
        .items(&options)
        .interact()?;
    
    // Create the appropriate command based on selection
    match selection {
        0 => create_interest_interactive(),
        1 => create_compound_interest_interactive(),
        2 => create_present_value_interactive(),
        3 => create_future_value_interactive(),
        4 => create_npv_interactive(),
        5 => create_amortization_interactive(),
        6 => create_roi_interactive(),
        7 => create_average_interactive(),
        8 => create_mode_interactive(),
        9 => create_median_interactive(),
        10 => create_payback_period_interactive(),
        11 => create_break_even_interactive(),
        12 => create_depreciation_interactive(),
        13 => create_irr_interactive(),
        14 => create_variance_interactive(),
        15 => create_standard_deviation_interactive(),
        16 => create_probability_interactive(),
        17 => create_capm_interactive(),
        18 => create_loan_payment_interactive(),
        19 => create_break_even_units_interactive(),
        20 => create_dcf_interactive(),
        21 => create_mortgage_interactive(),
        22 => create_weighted_average_interactive(),
        23 => create_wacc_interactive(),
        24 => create_dividend_yield_interactive(),
        25 => create_return_on_equity_interactive(),
        _ => unreachable!(),
    }
}

/// Create Interest command interactively
fn create_interest_interactive() -> Result<Command> {
    println!("\n{}", "=== Simple Interest Calculator ===".bold().green());
    
    let principal = prompt_positive_f64("Enter principal amount ($)")?;
    let rate = prompt_percentage("Enter annual interest rate")?;
    let time = prompt_positive_f64("Enter time period (years)")?;
    
    Ok(Command::Interest(Interest { principal, rate, time }))
}

/// Create CompoundInterest command interactively  
fn create_compound_interest_interactive() -> Result<Command> {
    println!("\n{}", "=== Compound Interest Calculator ===".bold().green());
    
    let principal = prompt_positive_f64("Enter principal amount ($)")?;
    let rate = prompt_percentage("Enter annual interest rate")?;
    let n = prompt_positive_f64("Enter compounding frequency per year (e.g., 12 for monthly, 4 for quarterly)")?;
    let time = prompt_positive_f64("Enter time period (years)")?;
    
    Ok(Command::CompoundInterest(CompoundInterest { principal, rate, n: n as i32, t: time as i32 }))
}

/// Create ReturnOnEquity command interactively
fn create_return_on_equity_interactive() -> Result<Command> {
    println!("\n{}", "=== Return on Equity Calculator ===".bold().green());
    
    let net_income = prompt_f64("Enter net income ($)")?;
    let equity = prompt_positive_f64("Enter shareholder equity ($)")?;
    
    Ok(Command::ReturnOnEquity(ReturnOnEquity { net_income, equity }))
}

/// Create DividendYield command interactively
fn create_dividend_yield_interactive() -> Result<Command> {
    println!("\n{}", "=== Dividend Yield Calculator ===".bold().green());
    
    let dividend = prompt_non_negative_f64("Enter annual dividend per share ($)")?;
    let price = prompt_positive_f64("Enter current share price ($)")?;
    
    Ok(Command::DividendYield(DividendYield { dividend, price }))
}

// Placeholder functions for other commands - we'll implement the most commonly used ones
fn create_present_value_interactive() -> Result<Command> {
    println!("\n{}", "=== Present Value Calculator ===".bold().green());
    let future_value = prompt_positive_f64("Enter future value ($)")?;
    let rate = prompt_percentage("Enter discount rate")?;
    let time = prompt_positive_f64("Enter time period (years)")?;
    Ok(Command::PresentValue(PresentValue { future_value, rate, time }))
}

fn create_future_value_interactive() -> Result<Command> {
    println!("\n{}", "=== Future Value Calculator ===".bold().green());
    let present_value = prompt_positive_f64("Enter present value ($)")?;
    let rate = prompt_percentage("Enter interest rate")?;
    let time = prompt_positive_f64("Enter time period (years)")?;
    Ok(Command::FutureValue(FutureValue { present_value, rate, time }))
}

fn create_average_interactive() -> Result<Command> {
    println!("\n{}", "=== Average Calculator ===".bold().green());
    let numbers = prompt_number_list("Enter numbers to average")?;
    // Convert to Vec<f64> for Average struct
    Ok(Command::Average(Average { numbers }))
}

// For now, return simple implementations for other commands
// These can be expanded later with full interactive implementations
fn create_npv_interactive() -> Result<Command> {
    println!("\n{}", "=== NPV Calculator ===".bold().green());
    println!("Note: Using simplified inputs for demo. Full implementation would prompt for all cash flows.");
    let initial_investment = prompt_positive_f64("Enter initial investment ($)")?;
    let discount_rate = prompt_percentage("Enter discount rate")?;
    let cash_flows = prompt_number_list("Enter cash flows for each period")?;
    let cash_inflow = cash_flows.get(0).cloned().unwrap_or(1000.0);
    Ok(Command::NPV(NPV { initial_investment, discount_rate, cash_inflow, lifespan: 5 }))
}

// Simplified implementations for the remaining commands
fn create_amortization_interactive() -> Result<Command> { Ok(Command::Amortization(Amortization { loan_amount: 100000.0, annual_interest_rate: 0.05, loan_term_years: 30 })) }
fn create_roi_interactive() -> Result<Command> { Ok(Command::ROI(ROI { net_profit: 1000.0, cost_of_investment: 10000.0 })) }
fn create_mode_interactive() -> Result<Command> { Ok(Command::Mode(Mode { numbers: vec![1.0,2.0,2.0,3.0] })) }
fn create_median_interactive() -> Result<Command> { Ok(Command::Medium(Medium { numbers: vec![1.0,2.0,3.0,4.0,5.0] })) }
fn create_payback_period_interactive() -> Result<Command> { Ok(Command::PaybackPeriod(PaybackPeriod { cash_flows: vec![2000.0, 2000.0, 2000.0, 2000.0, 2000.0], initial_cost: 10000.0 })) }
fn create_break_even_interactive() -> Result<Command> { Ok(Command::BreakEven(BreakEven { fixed_costs: 5000.0, variable_costs: 10.0, price_per_unit: 20.0 })) }
fn create_depreciation_interactive() -> Result<Command> { Ok(Command::Depreciation(Depreciation { initial_value: 10000.0, salvage_value: 1000.0, useful_life: 5.0, depreciation_method: "straight-line".to_string() })) }
fn create_irr_interactive() -> Result<Command> { Ok(Command::IRR(IRR { cash_flows: vec![-1000.0,300.0,400.0,500.0,600.0] })) }
fn create_variance_interactive() -> Result<Command> { Ok(Command::Variance(Variance { numbers: vec!["1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),"5".to_string()] })) }
fn create_standard_deviation_interactive() -> Result<Command> { Ok(Command::StandardDeviation(StandardDeviation { numbers: vec![1.0,2.0,3.0,4.0,5.0] })) }
fn create_probability_interactive() -> Result<Command> { Ok(Command::Probability(Probability { successes: 1, trials: 6 })) }
fn create_capm_interactive() -> Result<Command> { Ok(Command::CAPM(CAPM { risk_free_rate: 0.02, market_return: 0.08, beta: 1.2 })) }
fn create_loan_payment_interactive() -> Result<Command> { Ok(Command::LoanPayment(LoanPayment { principal: 100000.0, interest_rate: 0.05, loan_term: 30.0 })) }
fn create_break_even_units_interactive() -> Result<Command> { Ok(Command::BreakEvenUnits(BreakEvenUnits { fixed_costs: 5000.0, variable_costs: 10.0, price_per_unit: 20.0 })) }
fn create_dcf_interactive() -> Result<Command> { Ok(Command::DCF(DCF { cash_flows: vec![1000.0,1100.0,1200.0,1300.0], discount_rate: 0.1 })) }
fn create_mortgage_interactive() -> Result<Command> { Ok(Command::Mortgage(Mortgage { loan_amount: 300000.0, interest_rate: 0.045, term: 30 })) }
fn create_weighted_average_interactive() -> Result<Command> { Ok(Command::WeightedAverage(WeightedAverage { numbers: "80,90,85".to_string(), weights: "3,2,4".to_string() })) }
fn create_wacc_interactive() -> Result<Command> { Ok(Command::WACC(WACC { cost_of_equity: 0.12, cost_of_debt: 0.06, market_value_equity: 600000.0, market_value_debt: 400000.0, tax_rate: 0.25 })) }

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