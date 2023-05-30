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
    }
}

fn calculate_present_value(future_value: f64, rate: f64, time: f64) -> f64 {
    future_value / (1.0 + rate).powf(time)
}

fn calculate_future_value(present_value: f64, rate: f64, time: f64) -> f64 {
    present_value * (1.0 + rate).powf(time)
}