use clap::Parser;

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
    #[clap(short, long)]
    initial_investment: f64,

    /// The annual cash inflow
    #[clap(short, long)]
    cash_inflow: f64,

    /// The discount rate
    #[clap(short, long)]
    discount_rate: f64,

    /// The lifespan of the investment in years
    #[clap(short, long)]
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
            println!("The simple interest is: {}", result);
        }
        Command::CompoundInterest(compound_interest) => {
            let CompoundInterest { principal, rate, n, t } = compound_interest;
            let mut amount = principal;
            for year in 1..=t {
                amount *= 1.0 + (rate / n as f64);
                println!("Year {}: {}", year, amount);
            }
        }
        Command::PresentValue(args) => {
            let result = calculate_present_value(args.future_value, args.rate, args.time);
            println!("The present value is: {}", result);
        }
        Command::FutureValue(args) => {
            let result = calculate_future_value(args.present_value, args.rate, args.time);
            println!("The future value is: {}", result);
        }
        Command::NPV(npv) => {
            let NPV {
                initial_investment,
                cash_inflow,
                discount_rate,
                lifespan,
            } = npv;
    
            let npv = calculate_npv(initial_investment, cash_inflow, discount_rate, lifespan);
            println!("The net present value is: {}", npv);
        }
        Command::Amortization(args) => {
            calculate_amortization_schedule(args.loan_amount, args.annual_interest_rate, args.loan_term_years);
        }
    }
}

fn calculate_present_value(future_value: f64, rate: f64, time: f64) -> f64 {
    future_value / (1.0 + rate).powf(time)
}

fn calculate_future_value(present_value: f64, rate: f64, time: f64) -> f64 {
    present_value * (1.0 + rate).powf(time)
}

fn calculate_npv(initial_investment: f64, cash_inflow: f64, discount_rate: f64, lifespan: i32) -> f64 {
    let mut npv = -initial_investment;
    for i in 1..=lifespan {
        npv += cash_inflow / (1.0 + discount_rate).powf(i as f64);
    }
    npv
}

fn calculate_amortization_schedule(mut loan_amount: f64, annual_interest_rate: f64, loan_term_years: i32) {
    let monthly_interest_rate = annual_interest_rate / 12.0;
    let total_number_of_payments = loan_term_years * 12;

    let monthly_payment = (monthly_interest_rate * loan_amount) / (1.0 - (1.0 + monthly_interest_rate).powf(-total_number_of_payments as f64));

    for month in 1..=total_number_of_payments {
        let interest_payment = monthly_interest_rate * loan_amount;
        let principal_payment = monthly_payment - interest_payment;
        loan_amount -= principal_payment;

        println!("Month: {}, Principal: {}, Interest: {}, Remaining Balance: {}", month, principal_payment, interest_payment, loan_amount);
    }
}