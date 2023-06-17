use clap::Parser;
use prettytable::{Table, Row, Cell, format};
use prettytable::row;
use std::collections::HashMap;
use rust_decimal::prelude::*;

/// Financial calculation tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Opts {
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
    StandardDeviation(StandardDeviation),
    Probability(Probability),
    /// Calculates the expected return on an investment based on its risk and market factors.
    CAPM(CAPM),
    /// Calculate loan payments, including the monthly payment amount, total interest paid, and the loan payoff date.
    LoanPayment(LoanPayment),
    /// Calculate the number of units a business needs to sell to break even, taking into account fixed costs, variable costs per unit, and selling price per unit.
    BreakEvenUnits(BreakEvenUnits),
    ///  Implement a DCF calculator that calculates the present value of future cash flows, considering the time value of money.
    DCF(DCF),
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


fn calculate_dcf(cash_flows: &[f64], discount_rate: f64) -> f64 {
    let present_values: Vec<f64> = cash_flows
        .iter()
        .enumerate()
        .map(|(index, &cash_flow)| cash_flow / (1.0 + discount_rate).powf((index + 1) as f64))
        .collect();

    present_values.iter().sum()
}


#[derive(Parser, Debug)]
struct BreakEvenUnits {
    /// The fixed costs incurred by the business
    #[clap(short, long, name = "fixed-costs")]
    fixed_costs: f64,

    /// The variable costs per unit
    #[clap(short, long, name = "variable-costs")]
    variable_costs: f64,

    /// The price per unit of the product or service
    #[clap(short, long, name = "price-per-unit")]
    price_per_unit: f64,
}

fn calculate_break_even_units(fixed_costs: f64, variable_costs: f64, price_per_unit: f64) -> f64 {
    fixed_costs / (price_per_unit - variable_costs)
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

impl LoanPayment {
    fn calculate_loan_payment(&self) -> f64 {
        // Perform loan payment calculation based on the input parameters
        // Return the calculated loan payment amount
        // You can use the formulas for loan payment calculations, such as the one below:
        let n = self.loan_term * 12.0; // Number of monthly payments
        let r = self.interest_rate / 100.0 / 12.0; // Monthly interest rate

        let loan_payment = (self.principal * r) / (1.0 - (1.0 + r).powf(-n));

        loan_payment
    }
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
    #[clap(name = "numbers", required = true)]
    numbers: Vec<f64>,
}

#[derive(Parser, Debug)]
struct PaybackPeriod {
    #[clap(short = 'c', long = "cash-flows", name = "cash-flows")]
    cash_flows: Vec<f64>,

    #[clap(short = 'i', long = "initial-cost", name = "initial-cost")]
    initial_cost: f64,
}

#[derive(Parser, Debug)]
struct BreakEven {
    /// The total fixed costs incurred by the business
    #[clap(short, long, name = "fixed-costs")]
    fixed_costs: f64,

    /// The variable costs per unit
    #[clap(short, long, name = "variable-costs")]
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

impl Depreciation {
    fn straight_line(&self) -> f64 {
        (self.initial_value - self.salvage_value) / self.useful_life
    }

    fn double_declining_balance(&self) -> f64 {
        let straight_line = self.straight_line();
        let remaining_value = self.initial_value - straight_line;

        (2.0 * remaining_value) / self.useful_life
    }

    // Add more functions for different types of depreciation

    fn run(&self) {
        match self.depreciation_method.as_str() {
            "straight-line" => {
                let straight_line_depreciation = self.straight_line();
                let double_declining_balance_depreciation = self.double_declining_balance();

                let mut table = Table::new();
                table.set_titles(Row::new(vec![
                    Cell::new("Depreciation Type"),
                    Cell::new("Amount"),
                ]));

                table.add_row(Row::new(vec![
                    Cell::new("Straight Line"),
                    Cell::new(&format_currency(straight_line_depreciation)),
                ]));

                table.add_row(Row::new(vec![
                    Cell::new("Double Declining Balance"),
                    Cell::new(&format_currency(double_declining_balance_depreciation)),
                ]));

                // Add rows for other types of depreciation

                table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
                table.printstd();
            }
            // Handle other types of depreciation methods here
            _ => println!("Invalid depreciation method."),
        }
    }
}

#[derive(Parser, Debug)]
struct IRR {
    /// The cash flows for the investment/project
    #[clap(name = "cash-flows")]
    cash_flows: Vec<f64>,
}

impl IRR {
    fn execute(&self) {
        match self.calculate_irr() {
            Some(irr) => println!("Internal Rate of Return (IRR): {:.2}%", irr * 100.0),
            None => println!("Unable to calculate the Internal Rate of Return (IRR)"),
        }
    }
    fn calculate_irr(&self) -> Option<f64> {
        const MAX_ITERATIONS: u32 = 100;
        const PRECISION: f64 = 0.0001;

        let mut guess = 0.1; // Initial guess for IRR
        let mut iteration = 0;

        loop {
            let mut npv = 0.0;
            let mut npv_derivative = 0.0;

            for (index, cash_flow) in self.cash_flows.iter().enumerate() {
                let power = (self.cash_flows.len() - index - 1) as f64;
                npv += cash_flow / f64::powf(1.0 + guess, power);
                npv_derivative += -power * cash_flow / f64::powf(1.0 + guess, power + 1.0);
            }

            let new_guess = guess - npv / npv_derivative;

            if (guess - new_guess).abs() < PRECISION {
                return Some(new_guess);
            }

            guess = new_guess;
            iteration += 1;

            if iteration > MAX_ITERATIONS {
                break;
            }
        }

        None // Unable to converge within the maximum number of iterations
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

fn calculate_probability(successes: u32, trials: u32) -> f64 {
    (successes as f64) / (trials as f64)
}

fn calculate_capm(risk_free_rate: f64, beta: f64, market_return: f64) -> f64 {
    risk_free_rate + beta * (market_return - risk_free_rate)
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.command {
        Command::DCF(dcf) => {
            let dcf_value = calculate_dcf(&dcf.cash_flows, dcf.discount_rate);
        
            let mut table = Table::new();
            table.set_titles(Row::new(vec![
                Cell::new("Discount Rate"),
                Cell::new("Cash Flows"),
                Cell::new("DCF Value"),
            ]));
            table.add_row(Row::new(vec![
                Cell::new(&format!("{:.2}%", dcf.discount_rate * 100.0)),
                Cell::new(&format!("{:?}", dcf.cash_flows)),
                Cell::new(&format_currency(dcf_value)),
            ]));
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.printstd();
        }                
        Command::BreakEvenUnits(break_even_units) => {
            let break_even_point = calculate_break_even_units(break_even_units.fixed_costs, break_even_units.variable_costs, break_even_units.price_per_unit);
        
            let mut table = Table::new();
            table.set_titles(Row::new(vec![
                Cell::new("Metric"),
                Cell::new("Value"),
            ]));
            table.add_row(Row::new(vec![
                Cell::new("Break-Even Point (Units)"),
                Cell::new(&break_even_point.to_string()),
            ]));
        
            table.printstd();
        }        
        Command::LoanPayment(loan_payment) => {
            let payment_amount = loan_payment.calculate_loan_payment();
        
            let mut table = Table::new();
            table.set_titles(Row::new(vec![
                Cell::new("Principal"),
                Cell::new("Interest Rate"),
                Cell::new("Loan Term"),
                Cell::new("Monthly Payment"),
            ]));
        
            table.add_row(Row::new(vec![
                Cell::new(&format_currency(loan_payment.principal)),
                Cell::new(&format!("{:.2}%", loan_payment.interest_rate)),
                Cell::new(&format!("{:.0} years", loan_payment.loan_term)),
                Cell::new(&format_currency(payment_amount)),
            ]));
        
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.printstd();
        }
        Command::CAPM(capm) => {
            let expected_return = calculate_capm(capm.risk_free_rate, capm.beta, capm.market_return);

            let mut table = Table::new();
            table.set_titles(Row::new(vec![
                Cell::new("Risk-Free Rate"),
                Cell::new("Beta"),
                Cell::new("Market Return"),
                Cell::new("Expected Return"),
            ]));

            table.add_row(Row::new(vec![
                Cell::new(&format!("{:.2}%", capm.risk_free_rate * 100.0)),
                Cell::new(&capm.beta.to_string()),
                Cell::new(&format!("{:.2}%", capm.market_return * 100.0)),
                Cell::new(&format!("{:.2}%", expected_return * 100.0)),
            ]));

            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.printstd();
        }
        Command::Probability(probability) => {
            let successes = probability.successes;
            let trials = probability.trials;
            let result = calculate_probability(successes, trials);

            let mut table = Table::new();
            table.set_titles(Row::new(vec![
                Cell::new("Successes"),
                Cell::new("Trials"),
                Cell::new("Probability"),
            ]));

            table.add_row(Row::new(vec![
                Cell::new(&successes.to_string()),
                Cell::new(&trials.to_string()),
                Cell::new(&format!("{:.2}%", result * 100.0)),
            ]));

            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.printstd();
        }
        Command::IRR(irr) => {
            irr.execute();
        }
        Command::StandardDeviation(standard_deviation) => {
            let numbers = standard_deviation.numbers;
            let result = calculate_standard_deviation(&numbers);

            match result {
                Some(standard_deviation) => {
                    println!("Standard Deviation: {}", standard_deviation);
                }
                None => {
                    println!("Error: Standard deviation calculation requires a data set with at least 2 numbers.");
                }
            }
        }
        Command::Interest(interest) => {
            let Interest { principal, rate, time } = interest;
            let result = principal * rate * time;

            let mut table = Table::new();
            table.set_titles(Row::new(vec![
                Cell::new("Principal"),
                Cell::new("Rate"),
                Cell::new("Time"),
                Cell::new("Simple Interest"),
            ]));

            let formatted_principal = format_currency(principal);
            let formatted_result = format_currency(result);

            table.add_row(Row::new(vec![
                Cell::new(&formatted_principal),
                Cell::new(&rate.to_string()),
                Cell::new(&time.to_string()),
                Cell::new(&formatted_result),
            ]));
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.printstd();
        }
        Command::CompoundInterest(compound_interest) => {
            let CompoundInterest { principal, rate, n, t } = compound_interest;
            let mut amount = principal;

            let mut table = Table::new();
            table.set_titles(Row::new(vec![
                Cell::new("Year"),
                Cell::new("Amount"),
            ]));

            for year in 1..=t {
                amount *= 1.0 + (rate / n as f64);
                table.add_row(Row::new(vec![
                    Cell::new(&year.to_string()),
                    Cell::new(&format_currency(amount)),
                ]));
            }

            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.printstd();
        }
        Command::PresentValue(args) => {
            let result = calculate_present_value(args.future_value, args.rate, args.time);
        
            let mut table = Table::new();
            table.set_titles(Row::new(vec![
                Cell::new("Future Value"),
                Cell::new("Rate"),
                Cell::new("Time"),
                Cell::new("Present Value"),
            ]));

            table.add_row(Row::new(vec![
                Cell::new(&format_currency(args.future_value)),
                Cell::new(&args.rate.to_string()),
                Cell::new(&args.time.to_string()),
                Cell::new(&format_currency(result)),
            ]));
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.printstd();
        }        
        Command::FutureValue(args) => {
            let result = calculate_future_value(args.present_value, args.rate, args.time);
        
            let mut table = Table::new();
            table.set_titles(Row::new(vec![
                Cell::new("Present Value"),
                Cell::new("Rate"),
                Cell::new("Time"),
                Cell::new("Future Value"),
            ]));
            table.add_row(Row::new(vec![
                Cell::new(&format_currency(args.present_value)),
                Cell::new(&args.rate.to_string()),
                Cell::new(&args.time.to_string()),
                Cell::new(&format_currency(result)),
            ]));
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.printstd();
        }        
        Command::NPV(npv) => {
            let NPV { initial_investment, cash_inflow, discount_rate, lifespan } = npv;
            let mut npv_value = -initial_investment;
        
            let mut table = Table::new();
            table.set_titles(Row::new(vec![
                Cell::new("Year"),
                Cell::new("Cash Inflow"),
                Cell::new("Discounted Cash Flow"),
            ]));
        
            for year in 1..=lifespan {
                let discounted_cash_flow = cash_inflow / (1.0 + discount_rate).powf(year as f64);
                npv_value += discounted_cash_flow;
        
                table.add_row(Row::new(vec![
                    Cell::new(&year.to_string()),
                    Cell::new(&format_currency(cash_inflow)),
                    Cell::new(&format_currency(discounted_cash_flow)),
                ]));
            }
        
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.printstd();
        
            println!("Net Present Value (NPV): {}", format_currency(npv_value));
        }
        Command::Amortization(args) => {
            let loan_amount = args.loan_amount;
            let annual_interest_rate = args.annual_interest_rate;
            let loan_term_years = args.loan_term_years;
        
            let mut table = Table::new();
            table.set_titles(Row::new(vec![
                Cell::new("Month"),
                Cell::new("Principal Payment"),
                Cell::new("Interest Payment"),
                Cell::new("Remaining Balance"),
            ]));
        
            calculate_amortization_schedule(loan_amount, annual_interest_rate, loan_term_years, &mut table);
        
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.printstd();
        }
        Command::ROI(roi) => {
            let ROI { net_profit, cost_of_investment } = roi;
            let roi_value = (net_profit / cost_of_investment) * 100.0;

            let mut table = Table::new();
            table.set_titles(Row::new(vec![
                Cell::new("Net Profit"),
                Cell::new("Cost of Investment"),
                Cell::new("ROI"),
            ]));
            table.add_row(Row::new(vec![
                Cell::new(&format_currency(net_profit)),
                Cell::new(&format_currency(cost_of_investment)),
                Cell::new(&format!("{:.2}%", roi_value)),
            ]));
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.printstd();
        } 
        Command::Average(average) => {
            let Average { numbers } = average;
            let result = calculate_average(&numbers);

            if let Some(average) = result {
                // Printing the result as a pretty table
                let mut table = Table::new();
                table.set_titles(Row::new(vec![
                    Cell::new("Number"),
                ]));

                for number in &numbers {
                    table.add_row(Row::new(vec![
                        Cell::new(&number.to_string()),
                    ]));
                }

                table.add_row(Row::new(vec![
                    Cell::new("Average"),
                    Cell::new(&average.to_string()),
                ]));

                table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
                table.printstd();
            } else {
                println!("Error: Invalid input. Please provide a list of numbers.");
            }
        }   
        Command::Mode(mode) => {
            let result = calculate_mode(&mode.numbers);
        
            let mut table = Table::new();
            table.set_titles(Row::new(vec![Cell::new("Mode")]));
        
            match result {
                Some(mode_value) => {
                    table.add_row(Row::new(vec![Cell::new(&mode_value.to_string())]));
                }
                None => {
                    table.add_row(Row::new(vec![Cell::new("No mode exists.")]));
                }
            }
        
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.printstd();
        }
        Command::Medium(medium) => medium.run(),
        Command::PaybackPeriod(payback_period) => {
            let result = calculate_payback_period(payback_period.cash_flows.clone(), payback_period.initial_cost);
        
            let mut table = Table::new();
            table.set_titles(Row::new(vec![
                Cell::new("Cash Flows"),
                Cell::new("Initial Cost"),
                Cell::new("Payback Period"),
            ]));
            table.add_row(Row::new(vec![
                Cell::new(&format!("{:?}", payback_period.cash_flows)),
                Cell::new(&format_currency(payback_period.initial_cost)),
                Cell::new(&result.to_string()),
            ]));
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.printstd();
        }
        Command::BreakEven(break_even) => {
            calculate_break_even(break_even.fixed_costs, break_even.variable_costs, break_even.price_per_unit);
        }
        Command::Depreciation(depreciation) => {
            depreciation.run();
        }
        Command::Variance(variance) => {
            let numbers: Vec<f64> = variance.numbers
                .iter()
                .map(|n| n.parse::<f64>().unwrap())
                .collect();
        
            let result = calculate_variance(&numbers);
        
            let mut table = Table::new();
            table.set_titles(Row::new(vec![
                Cell::new("Number"),
            ]));
        
            for number in &numbers {
                table.add_row(Row::new(vec![
                    Cell::new(&number.to_string()),
                ]));
            }
        
            table.add_row(Row::new(vec![
                Cell::new("Variance"),
                Cell::new(&result.unwrap_or(0.0).to_string()),
            ]));
        
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.printstd();
        }        
    }
}

fn calculate_present_value(future_value: f64, rate: f64, time: f64) -> f64 {
    future_value / (1.0 + rate).powf(time)
}

fn calculate_future_value(present_value: f64, rate: f64, time: f64) -> f64 {
    present_value * (1.0 + rate).powf(time)
}

fn calculate_amortization_schedule(mut loan_amount: f64, annual_interest_rate: f64, loan_term_years: i32, table: &mut Table) {
    let monthly_interest_rate = annual_interest_rate / 12.0;
    let total_number_of_payments = loan_term_years * 12;

    let monthly_payment = (monthly_interest_rate * loan_amount) / (1.0 - (1.0 + monthly_interest_rate).powf(-total_number_of_payments as f64));

    table.set_titles(row![bFg->"Month", bFg->"Principal", bFg->"Interest", bFg->"Remaining Balance"]);

    for month in 1..=total_number_of_payments {
        let interest_payment = monthly_interest_rate * loan_amount;
        let principal_payment = monthly_payment - interest_payment;
        loan_amount -= principal_payment;

        table.add_row(row![month, format_currency(principal_payment), format_currency(interest_payment), format_currency(loan_amount)]);
    }
}

fn calculate_average(numbers: &[f64]) -> Option<f64> {
    if numbers.is_empty() {
        return None;
    }

    let sum: f64 = numbers.iter().sum();
    let average = sum / numbers.len() as f64;
    Some(average)
}

fn calculate_mode(numbers: &[f64]) -> Option<f64> {
    let mut counts: HashMap<String, usize> = HashMap::new();

    for &number in numbers {
        let key = number.to_string();
        let count = counts.entry(key).or_insert(0);
        *count += 1;
    }

    let max_count = counts.values().max();

    if let Some(&count) = max_count {
        let modes: Vec<f64> = counts
            .iter()
            .filter(|&(_, &c)| c == count)
            .map(|(key, _)| key.parse::<f64>().unwrap())
            .collect();

        if modes.len() == counts.len() {
            return None; // No mode exists
        } else {
            return Some(*modes.first().unwrap()); // Return the first mode
        }
    }

    None // No mode exists
}

fn calculate_medium(numbers: &[f64]) -> Option<f64> {
    let mut sorted_numbers = numbers.to_vec();
    sorted_numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let len = sorted_numbers.len();
    if len == 0 {
        None
    } else if len % 2 == 0 {
        let mid = len / 2;
        let median = (sorted_numbers[mid - 1] + sorted_numbers[mid]) / 2.0;
        Some(median)
    } else {
        let mid = len / 2;
        Some(sorted_numbers[mid])
    }
}

impl Medium {
    fn run(&self) {
        let medium = calculate_medium(&self.numbers);

        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(Row::new(vec![
            Cell::new("Numbers").style_spec("Fg"),
            Cell::new("Medium").style_spec("Fg"),
        ]));

        for number in &self.numbers {
            let row = Row::new(vec![
                Cell::new(&number.to_string()).style_spec("Fg"),
                Cell::new("").style_spec("Fg"),
            ]);
            table.add_row(row);
        }

        if let Some(medium) = medium {
            let result_row = Row::new(vec![
                Cell::new("Result").style_spec("Fg"),
                Cell::new(&medium.to_string()).style_spec("Fg"),
            ]);
            table.add_row(result_row);
        }

        table.printstd();
    }
}

fn calculate_payback_period(cash_flows: Vec<f64>, initial_cost: f64) -> f64 {
    let mut cumulative_cash_flow = 0.0;
    let mut period = 0;

    for cash_flow in cash_flows {
        cumulative_cash_flow += cash_flow;
        period += 1;

        if cumulative_cash_flow >= initial_cost {
            return period as f64;
        }
    }

    -1.0 // Indicates that the payback period was not reached
}

fn calculate_break_even(fixed_costs: f64, variable_costs: f64, price_per_unit: f64) {
    let break_even_point = fixed_costs / (price_per_unit - variable_costs);
    let total_revenue = price_per_unit * break_even_point;

    let mut table = Table::new();
    table.set_titles(Row::new(vec![
        Cell::new("Metric"),
        Cell::new("Value"),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Break-Even Point (units)"),
        Cell::new(&format_currency(break_even_point)),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Total Revenue Required ($)"),
        Cell::new(&format_currency(total_revenue)),
    ]));

    table.printstd();
}

fn format_currency(number: f64) -> String {
    let decimal = Decimal::from_f64(number).unwrap();
    let formatted = decimal.to_string();
    
    let parts: Vec<&str> = formatted.split('.').collect();
    let whole_part = parts[0];
    let decimal_part = parts.get(1).copied().unwrap_or("");
    
    let whole_part_with_commas = insert_commas(whole_part);

    if decimal_part.is_empty() {
        format!("${}", whole_part_with_commas)
    } else {
        format!("${}.{:.2}", whole_part_with_commas, decimal_part)
    }
}

fn insert_commas(number: &str) -> String {
    let chars: Vec<char> = number.chars().collect();
    let mut result = String::new();
    let mut count = 0;

    for i in (0..chars.len()).rev() {
        if count == 3 {
            result.insert(0, ',');
            count = 0;
        }
        result.insert(0, chars[i]);
        count += 1;
    }

    result
}

fn calculate_variance(numbers: &[f64]) -> Option<f64> {
    let count = numbers.len() as f64;

    if count <= 1.0 {
        return None; // Variance is undefined for a single element or an empty data set
    }

    let mean = numbers.iter().sum::<f64>() / count;

    let sum_squared_diff: f64 = numbers
        .iter()
        .map(|&x| (x - mean).powi(2))
        .sum();

    Some(sum_squared_diff / count)
}

fn calculate_standard_deviation(numbers: &[f64]) -> Option<f64> {
    let n = numbers.len() as f64;

    if n < 2.0 {
        return None;
    }

    let mean = calculate_mean(numbers);

    let sum_squared_deviations = numbers
        .iter()
        .map(|&number| (number - mean).powi(2))
        .sum::<f64>();

    let variance = sum_squared_deviations / (n - 1.0);

    Some(variance.sqrt())
}

fn calculate_mean(numbers: &[f64]) -> f64 {
    let sum: f64 = numbers.iter().sum();
    let count = numbers.len() as f64;
    sum / count
}
