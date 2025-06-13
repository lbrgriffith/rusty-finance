use anyhow::{Context, Result};
use chrono::{Local, Months};
use clap::Parser;
use env_logger::Env;
use log::{debug, info};
use owo_colors::OwoColorize;

// Import from our modular structure
use rusty_finance::{
    calculations::*,
    cli::*,
    display::*,
};

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
            
            let result = calculate_simple_interest(interest.principal, interest.rate, interest.time)
                .context("Failed to calculate simple interest")?;
            
            info!("Calculated simple interest: {:.4}", result);
            
            // Create and format table
            let mut table = create_table(vec!["Principal", "Rate", "Time", "Simple Interest"]);
            
            table.add_row(vec![
                format_currency(interest.principal),
                format_rate_as_percentage(interest.rate),
                format_years(interest.time),
                format_currency(result),
            ]);
            
            println!("{table}");
            info!("Simple interest calculation completed");
            Ok(())
        }
        Command::CompoundInterest(ci) => {
            debug!("Calculating compound interest with: {:?}", ci);
            
            // Create and format table
            let mut table = create_table(vec!["Year", "Amount"]);
            
            // Calculate compound interest for each year
            for year in 1..=ci.t {
                let amount = calculate_compound_interest(ci.principal, ci.rate, ci.n, year)
                    .context("Failed to calculate compound interest")?;
                
                table.add_row(vec![
                    format!("{}", year),
                    format_currency(amount),
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
            
            table.add_row(vec![
                format_currency(pv.future_value),
                format_rate_as_percentage(pv.rate),
                format_years(pv.time),
                format_currency(result),
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
            
            table.add_row(vec![
                format_currency(fv.present_value),
                format_rate_as_percentage(fv.rate),
                format_years(fv.time),
                format_currency(result),
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
                
                table.add_row(vec![
                    format!("{}", year),
                    format_currency(npv.cash_inflow),
                    format_currency(discounted_cash_flow),
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
            
            table.add_row(vec![
                format_currency(roi.net_profit),
                format_currency(roi.cost_of_investment),
                format_percentage(roi_value / 100.0, 2),
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
                table.add_row(vec![format!("{:.2}", number)]);
            }
            
            table.add_row(vec![format!("Average: {:.2}", avg)]);
            
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
                table.add_row(vec![format!("{:.2}", number), "".to_string()]);
            }
            
            match mode_value {
                Some(m) => table.add_row(vec!["Mode:".to_string(), format!("{:.2}", m)]),
                None => table.add_row(vec!["Mode:".to_string(), "No mode".to_string()]),
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
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            
            for number in &sorted {
                table.add_row(vec![format!("{:.2}", number)]);
            }
            
            table.add_row(vec![format!("Median: {:.2}", median)]);
            
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
            
            table.add_row(vec![
                format!("{:?}", payback.cash_flows),
                format_currency(payback.initial_cost),
                payback_str,
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
                    table.add_row(vec![
                        format!("{}", payment.month),
                        format_currency(payment.principal_payment),
                        format_currency(payment.interest_payment),
                        format_currency(payment.remaining_balance),
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
            
            table.add_row(vec![
                format_currency(roe.net_income),
                format_currency(roe.equity),
                format_percentage(roe_value / 100.0, 2),
            ]);
            
            println!("{table}");
            info!("ROE calculation completed successfully");
            Ok(())
        }
        Command::DividendYield(dividend_yield) => {
            debug!("Calculating dividend yield with: {:?}", dividend_yield);
            
            let result = calculate_dividend_yield(dividend_yield.dividend, dividend_yield.price)
                .context("Failed to calculate dividend yield")?;
            
            info!("Calculated dividend yield: {:.4}", result);
            
            let mut table = create_table(vec!["Dividend", "Price", "Dividend Yield"]);
            
            table.add_row(vec![
                format!("{:.2}", dividend_yield.dividend),
                format!("{:.2}", dividend_yield.price),
                format_percentage(result / 100.0, 2),
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