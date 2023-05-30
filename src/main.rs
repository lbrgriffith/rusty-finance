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
    }
}
