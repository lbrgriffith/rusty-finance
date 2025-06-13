//! Command-line interface structures and definitions

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};

/// Financial calculation tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Opts {
    /// Verbosity level for logging
    #[clap(flatten)]
    pub verbose: Verbosity<InfoLevel>,
    
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Parser, Debug)]
pub enum Command {
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
}

#[derive(Parser, Debug)]
pub struct Interest {
    /// The principal amount (initial money)
    #[clap(short, long)]
    pub principal: f64,

    /// The rate of interest (per period)
    #[clap(short, long)]
    pub rate: f64,

    /// The time the money is invested for
    #[clap(short, long)]
    pub time: f64,
}

#[derive(Parser, Debug)]
pub struct CompoundInterest {
    /// The principal amount (initial money)
    #[clap(short, long)]
    pub principal: f64,

    /// The annual interest rate
    #[clap(short, long)]
    pub rate: f64,

    /// The number of times interest is compounded per year
    #[clap(short, long)]
    pub n: i32,

    /// The time the money is invested for in years
    #[clap(short, long)]
    pub t: i32,
}

#[derive(Parser, Debug)]
pub struct PresentValue {
    /// The future value of the investment.
    #[clap(short, long)]
    pub future_value: f64,

    /// The interest rate per period.
    #[clap(short, long)]
    pub rate: f64,

    /// The number of periods.
    #[clap(short, long)]
    pub time: f64,
}

#[derive(Parser, Debug)]
pub struct FutureValue {
    /// The present value of the investment.
    #[clap(short, long)]
    pub present_value: f64,

    /// The interest rate per period.
    #[clap(short, long)]
    pub rate: f64,

    /// The number of periods.
    #[clap(short, long)]
    pub time: f64,
}

#[derive(Parser, Debug)]
pub struct NPV {
    /// The initial investment or cost
    #[clap(short, long, name = "initial-investment")]
    pub initial_investment: f64,

    /// The annual cash inflow
    #[clap(short, long, name = "cash-inflow")]
    pub cash_inflow: f64,

    /// The discount rate
    #[clap(short, long, name = "discount-rate")]
    pub discount_rate: f64,

    /// The lifespan of the investment in years
    #[clap(short, long, name = "lifespan")]
    pub lifespan: i32,
}

#[derive(Parser, Debug)]
pub struct Amortization {
    /// The initial loan amount
    #[clap(short = 'a', long)]
    pub loan_amount: f64,

    /// The annual interest rate
    #[clap(short = 'i', long)]
    pub annual_interest_rate: f64,

    /// The loan term in years
    #[clap(short = 't', long)]
    pub loan_term_years: i32,
}

#[derive(Parser, Debug)]
pub struct ROI {
    /// The net profit
    #[clap(short, long, name = "net-profit")]
    pub net_profit: f64,

    /// The cost of investment
    #[clap(short, long, name = "cost-of-investment")]
    pub cost_of_investment: f64,
}

#[derive(Parser, Debug)]
pub struct Average {
    /// The numbers to calculate the average of
    #[clap(name = "numbers")]
    pub numbers: Vec<f64>,
}

#[derive(Parser, Debug)]
pub struct Mode {
    /// The numbers to calculate the mode
    #[clap(required = true)]
    pub numbers: Vec<f64>,
}

#[derive(Parser, Debug)]
pub struct Medium {
    /// The numbers to calculate the median
    #[clap(name = "numbers", required = true)]
    pub numbers: Vec<f64>,
}

#[derive(Parser, Debug)]
pub struct PaybackPeriod {
    /// The list of cash flows
    #[clap(short = 'c', long = "cash-flows", name = "cash-flows")]
    pub cash_flows: Vec<f64>,

    /// The initial cost of the investment
    #[clap(short = 'i', long = "initial-cost", name = "initial-cost")]
    pub initial_cost: f64,
}

#[derive(Parser, Debug)]
pub struct BreakEven {
    /// The total fixed costs incurred by the business
    #[clap(short, long, name = "fixed-costs")]
    pub fixed_costs: f64,

    /// The variable costs per unit
    #[clap(short, long, name = "variable-costs")]
    pub variable_costs: f64,

    /// The price per unit of the product or service
    #[clap(short, long, name = "price-per-unit")]
    pub price_per_unit: f64,
}

#[derive(Parser, Debug)]
pub struct Depreciation {
    /// The initial value of the asset
    #[clap(short, long, name = "initial-value")]
    pub initial_value: f64,

    /// The salvage value of the asset
    #[clap(short, long, name = "salvage-value")]
    pub salvage_value: f64,

    /// The useful life of the asset
    #[clap(short, long, name = "useful-life")]
    pub useful_life: f64,

    /// The method of depreciation (e.g., straight-line, double-declining-balance)
    #[clap(short, long, name = "depreciation-method")]
    pub depreciation_method: String,
}

#[derive(Parser, Debug)]
pub struct IRR {
    /// The cash flows for the investment/project
    #[clap(name = "cash-flows")]
    pub cash_flows: Vec<f64>,
}

#[derive(Parser, Debug)]
pub struct Variance {
    /// The numbers to calculate the variance
    #[clap(name = "numbers")]
    pub numbers: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct StandardDeviation {
    /// The numbers to calculate the standard deviation of.
    #[clap(name = "numbers")]
    pub numbers: Vec<f64>,
}

#[derive(Parser, Debug)]
pub struct Probability {
    /// The number of successful outcomes
    #[clap(short, long)]
    pub successes: u32,

    /// The number of total trials
    #[clap(short, long)]
    pub trials: u32,
}

#[derive(Parser, Debug)]
pub struct CAPM {
    /// The risk-free rate
    #[clap(short, long)]
    pub risk_free_rate: f64,

    /// The asset's beta coefficient
    #[clap(short, long)]
    pub beta: f64,

    /// The expected return of the market
    #[clap(short, long)]
    pub market_return: f64,
}

#[derive(Parser, Debug)]
pub struct LoanPayment {
    /// The principal amount of the loan
    #[clap(short, long)]
    pub principal: f64,

    /// The annual interest rate of the loan
    #[clap(short, long)]
    pub interest_rate: f64,

    /// The loan term in years
    #[clap(short, long)]
    pub loan_term: f64,
}

#[derive(Parser, Debug)]
pub struct BreakEvenUnits {
    /// The fixed costs incurred by the business
    #[clap(short, long, name = "fixed-costs")]
    pub fixed_costs: f64,

    /// The variable costs per unit
    #[clap(short, long, name = "variable-costs")]
    pub variable_costs: f64,

    /// The price per unit of the product or service
    #[clap(short, long, name = "price-per-unit")]
    pub price_per_unit: f64,
}

#[derive(Parser, Debug)]
pub struct DCF {
    /// The discount rate
    #[clap(short, long, name = "discount-rate")]
    pub discount_rate: f64,

    /// The cash flows for the investment/project
    #[clap(name = "cash-flows")]
    pub cash_flows: Vec<f64>,
}

#[derive(Parser, Debug)]
pub struct Mortgage {
    /// The loan amount
    #[clap(short, long, name = "loan-amount")]
    pub loan_amount: f64,

    /// The annual interest rate
    #[clap(short, long, name = "interest-rate")]
    pub interest_rate: f64,

    /// The loan term in years
    #[clap(short, long)]
    pub term: i32,
}

#[derive(Parser, Debug)]
pub struct WeightedAverage {
    /// The numbers to calculate the weighted average of
    #[clap(short, long)]
    pub numbers: String,

    /// The weights for each number
    #[clap(short, long)]
    pub weights: String,
}

#[derive(Parser, Debug)]
pub struct WACC {
    /// The cost of equity (Ke)
    #[clap(long)]
    pub cost_of_equity: f64,

    /// The cost of debt (Kd)
    #[clap(long)]
    pub cost_of_debt: f64,

    /// The tax rate
    #[clap(long)]
    pub tax_rate: f64,

    /// The market value of equity (E)
    #[clap(long)]
    pub market_value_equity: f64,

    /// The market value of debt (D)
    #[clap(long)]
    pub market_value_debt: f64,
}

#[derive(Parser, Debug)]
pub struct DividendYield {
    /// The dividend
    #[clap(short, long)]
    pub dividend: f64,

    /// The price
    #[clap(short, long)]
    pub price: f64,
}

#[derive(Parser, Debug)]
pub struct ReturnOnEquity {
    /// Net Income
    #[clap(short, long)]
    pub net_income: f64,

    /// Equity
    #[clap(short, long)]
    pub equity: f64,
}