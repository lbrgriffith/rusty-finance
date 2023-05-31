use clap::Parser;
use prettytable::{Table, Row, Cell, format};
use prettytable::row;

/// Financial calculation tool
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Opts {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    /// Calculates simple interest
    Interest(Interest),
    /// Calculates compound interest
    CompoundInterest(CompoundInterest),
    PresentValue(PresentValue),
    FutureValue(FutureValue),
    /// Calculates net present value
    NPV(NPV),
    /// Calculates an amortization schedule
    Amortization(Amortization),
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
    #[clap(short, long)]
    future_value: f64,
    #[clap(short, long)]
    rate: f64,
    #[clap(short, long)]
    time: f64,
}

#[derive(Parser, Debug)]
struct FutureValue {
    #[clap(short, long)]
    present_value: f64,
    #[clap(short, long)]
    rate: f64,
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



fn main() {
    let opts: Opts = Opts::parse();

    match opts.command {
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
            table.add_row(Row::new(vec![
                Cell::new(&principal.to_string()),
                Cell::new(&rate.to_string()),
                Cell::new(&time.to_string()),
                Cell::new(&result.to_string()),
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
                    Cell::new(&amount.to_string()),
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
                Cell::new(&args.future_value.to_string()),
                Cell::new(&args.rate.to_string()),
                Cell::new(&args.time.to_string()),
                Cell::new(&result.to_string()),
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
                Cell::new(&args.present_value.to_string()),
                Cell::new(&args.rate.to_string()),
                Cell::new(&args.time.to_string()),
                Cell::new(&result.to_string()),
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
                    Cell::new(&cash_inflow.to_string()),
                    Cell::new(&discounted_cash_flow.to_string()),
                ]));
            }
        
            table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
            table.printstd();
        
            println!("Net Present Value (NPV): {}", npv_value);
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

        table.add_row(row![month, principal_payment, interest_payment, loan_amount]);
    }
}
