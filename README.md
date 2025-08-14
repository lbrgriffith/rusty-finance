# Rusty Finance CLI

![Rust](https://img.shields.io/badge/Rust-Programming%20Language-orange?logo=rust)
![Edition](https://img.shields.io/badge/Rust%20Edition-2021-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Build Status](https://img.shields.io/badge/build-passing-green)

Rusty Finance is a modern, high-performance command-line tool for comprehensive financial calculations. Built with Rust's safety and performance in mind, it features a modular architecture with robust error handling, beautiful table formatting, and extensive validation.

![image](https://github.com/user-attachments/assets/a7955b90-442f-48a3-9058-6bffe27d396d)

## Features
- 🚀 **Modern Rust Architecture**: Built with Rust 2021 edition featuring modular design
- 🎨 **Rich CLI Experience**: Color-coded output with modern table formatting using `comfy-table`
- 🔒 **Type Safety**: Uses `rust_decimal` for precise financial calculations
- 🛡️ **Robust Error Handling**: Comprehensive validation with `thiserror` and `anyhow`
- 📊 **Beautiful Output**: Professional table formatting with proper alignment
- 🔧 **Extensible Design**: Modular architecture with separate calculation modules
- 📋 **Comprehensive Commands**: 20+ financial calculation commands
- 📝 **Advanced Logging**: Configurable verbosity levels with structured logging
- ⚡ **High Performance**: Optimized for fast calculations with minimal memory usage


## Installation

### Prerequisites
- Rust 1.70+ (2021 edition)
- Cargo package manager

### From Source
```bash
# Clone the repository
git clone https://github.com/yourusername/rusty-finance.git
cd rusty-finance

# Build optimized release version
cargo build --release

# Install globally (optional)
cargo install --path .

# The binary will be available at target/release/rusty-finance
```

### Quick Test
```bash
# Verify installation
./target/release/rusty-finance --help

# Run a quick calculation
./target/release/rusty-finance interest --principal 1000 --rate 0.05 --time 2
```

## Usage

Rusty Finance provides a comprehensive set of financial calculation commands with a modern CLI interface.

### Basic Usage
```bash
# General syntax
rusty-finance <COMMAND> [OPTIONS]

# Enable verbose logging
rusty-finance -v <COMMAND> [OPTIONS]

# Get help for any command
rusty-finance help <COMMAND>

### Shell Completion

Rusty Finance supports shell completions for Bash, Zsh, Fish, and PowerShell to help you quickly navigate commands and options.

#### Installation

To install shell completions, generate the completion script for your shell:

```bash
# For Bash
rusty-finance completion bash > ~/.local/share/bash-completion/completions/rusty-finance

# For Zsh (make sure ~/.zfunc is in your $fpath)
mkdir -p ~/.zfunc
rusty-finance completion zsh > ~/.zfunc/_rusty-finance

# For Fish
rusty-finance completion fish > ~/.config/fish/completions/rusty-finance.fish

# For PowerShell (add to your PowerShell profile)
rusty-finance completion powershell >> $PROFILE
```

After installing, restart your shell or source your shell configuration file.

#### Usage

With completions installed, you can:
- Press `Tab` to auto-complete commands and options
- Press `Tab` twice to see available options
- Navigate through long command names easily

Example:
```bash
rusty-finance <TAB><TAB>      # Shows all available commands
rusty-finance int<TAB>        # Completes to "interest"
rusty-finance interest --<TAB><TAB>  # Shows available options: --principal, --rate, --time
```

Here are the available commands:

- `interest`: Calculates simple interest.
- `compound-interest`: Calculates compound interest.
- `present-value`: Calculates present value.
- `future-value`: Calculates future value.
- `npv`: Calculates net present value.
- `amortization`: Calculates an amortization schedule.
- `average`: Calculates the average of a series of numbers.
- `mode`: Calculates the mode of a series of numbers.
- `medium`: Calculates the median of a series of numbers.
- `payback-period`: Calculates the payback period.
- `break-even`: Performs break-even analysis.
- `depreciation`: Calculates the depreciation of an asset.
- `irr`: Calculates the internal rate of return.
- `variance`: Measures how much the numbers in a data set deviate from the mean.
- `probability`: Calculates the probability of an event.
- `capm`: Calculates the expected return using the Capital Asset Pricing Model.
- `loan-payment`: Calculate loan payments, including the monthly payment amount, total interest paid, and the loan payoff date.
- `break-even-units`: Calculate the number of units a business needs to sell to break even.
- `dcf`: Calculates the present value of future cash flows, considering the time value of money.
- `mortgage`: Calculate mortgage payments.
- `weighted-average`: Calculate the weighted average of a series of numbers.
- `wacc`: Calculates the weighted average cost of capital
- `dividend-yield`: Calculates the dividend yield of a stock by dividing the annual dividend per share by the stock's current price.
- `return-on-equity`: Calculates the return on equity as a percentage,
- `completion`: Generate shell completions for Bash, Zsh, Fish, or PowerShell.

# List all available commands
rusty-finance --help
```

### Available Commands

#### 💰 Interest & Time Value
- **`interest`** - Simple interest calculations
- **`compound-interest`** - Compound interest with compounding periods
- **`present-value`** - Present value of future cash flows
- **`future-value`** - Future value calculations

#### 📈 Investment Analysis
- **`npv`** - Net Present Value analysis
- **`irr`** - Internal Rate of Return
- **`dcf`** - Discounted Cash Flow valuation
- **`payback-period`** - Investment payback analysis
- **`capm`** - Capital Asset Pricing Model

#### 🏠 Loans & Mortgages
- **`loan-payment`** - Loan payment calculations
- **`mortgage`** - Mortgage payment analysis
- **`amortization`** - Detailed amortization schedules

#### 📊 Business Analysis
- **`break-even`** - Break-even point analysis
- **`break-even-units`** - Unit-based break-even calculations
- **`depreciation`** - Asset depreciation calculations
- **`wacc`** - Weighted Average Cost of Capital

#### 📈 Financial Ratios
- **`dividend-yield`** - Stock dividend yield analysis
- **`return-on-equity`** - ROE calculations

#### 📉 Statistics
- **`average`** - Arithmetic mean calculations
- **`median`** - Median value calculations
- **`mode`** - Mode (most frequent value)
- **`variance`** - Data variance and standard deviation
- **`weighted-average`** - Weighted average calculations
- **`probability`** - Basic probability calculations

## Architecture & Design

### Modern Rust Practices
- **Modular Design**: Organized into separate modules (`calculations`, `cli`, `display`)
- **Type Safety**: Uses `rust_decimal` for precise financial calculations
- **Error Handling**: Comprehensive error types with `thiserror` and `anyhow`
- **Input Validation**: Robust validation for all financial inputs
- **Testing**: Comprehensive unit tests for all calculation modules

### Key Dependencies
- `clap` 4.5+ - Modern CLI argument parsing with derive macros
- `comfy-table` - Beautiful table formatting
- `rust_decimal` - Precise decimal arithmetic for financial calculations
- `chrono` - Date and time handling for financial schedules
- `owo-colors` - Rich terminal colors and styling

### Performance Features
- Zero-cost abstractions using Rust's type system
- Minimal runtime overhead with compile-time optimizations
- Memory-safe calculations without garbage collection
- Fast startup time and low resource usage

## Examples

### 💰 Simple Interest Calculation

Calculate interest on a principal amount:

```bash
rusty-finance interest --principal 1000 --rate 0.05 --time 2
```

**Output:**
```
┌───────────┬──────┬──────┬─────────────────┐
│ Principal │ Rate │ Time │ Simple Interest │
├───────────┼──────┼──────┼─────────────────┤
│ $1,000    │ 5%   │ 2    │ $100.00         │
└───────────┴──────┴──────┴─────────────────┘
```

### 📈 Compound Interest Analysis

Calculate compound interest with monthly compounding:

```bash
rusty-finance compound-interest --principal 1000 --rate 0.05 --n 12 --t 3
```

**Output:**
```
┌──────┬─────────────┐
│ Year │ Amount      │
├──────┼─────────────┤
│ 1    │ $1,051.16   │
│ 2    │ $1,104.89   │
│ 3    │ $1,161.47   │
└──────┴─────────────┘
```

### 💵 Present Value Calculation

Find the current value of future money:

```bash
rusty-finance present-value --future-value 1000 --rate 0.05 --time 3
```

**Output:**
```
┌─────────────────┬──────┬──────┬───────────────┐
│ Future Value    │ Rate │ Time │ Present Value │
├─────────────────┼──────┼──────┼───────────────┤
│ $1,000          │ 5%   │ 3    │ $863.84       │
└─────────────────┴──────┴──────┴───────────────┘
```

### 🚀 Investment Analysis

**Net Present Value (NPV)**
```bash
rusty-finance npv --initial-investment 10000 --cash-inflow 2000 --discount-rate 0.1 --lifespan 5
```

**Internal Rate of Return (IRR)**
```bash
rusty-finance irr -1000 500 300 200 100
```

**Capital Asset Pricing Model (CAPM)**
```bash
rusty-finance capm --risk-free-rate 0.03 --beta 1.2 --market-return 0.08
```

### 🏠 Loan & Mortgage Calculations

**Loan Payment Analysis**
```bash
rusty-finance loan-payment --principal 100000 --interest-rate 5 --loan-term 10
```

**Mortgage Calculator**
```bash
rusty-finance mortgage --loan-amount 300000 --interest-rate 4.5 --term 30
```

**Amortization Schedule**
```bash
rusty-finance amortization --amount 100000 --interest 0.05 --term 30
```

### 📊 Business & Financial Ratios

**Break-Even Analysis**
```bash
rusty-finance break-even --fixed-costs 50000 --variable-costs 10 --price-per-unit 25
```

**Weighted Average Cost of Capital (WACC)**
```bash
rusty-finance wacc --cost-of-equity 0.12 --cost-of-debt 0.06 --tax-rate 0.25 --market-value-equity 1000000 --market-value-debt 400000
```

**Return on Equity (ROE)**
```bash
rusty-finance return-on-equity --net-income 500000 --equity 2500000
```

### 📈 Statistical Analysis

**Data Analysis**
```bash
# Calculate various statistics
rusty-finance average 10 20 30 40 50
rusty-finance median 5 10 15 20 25
rusty-finance variance 100 200 300 400 500

# Weighted calculations
rusty-finance weighted-average --numbers "85 90 78 92" --weights "0.3 0.3 0.2 0.2"
```

## Development

### Building from Source
```bash
# Clone and build
git clone https://github.com/yourusername/rusty-finance.git
cd rusty-finance

# Run tests
cargo test

# Build optimized release
cargo build --release

# Run with cargo
cargo run -- interest --principal 1000 --rate 0.05 --time 2
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test calculations::interest
```

### Project Structure
```
src/
├── main.rs              # Application entry point
├── lib.rs               # Library root with error types
├── cli.rs               # Command-line interface definitions
├── display.rs           # Table formatting and output
└── calculations/        # Financial calculation modules
    ├── mod.rs           # Module definitions
    ├── interest.rs      # Interest calculations
    ├── investment.rs    # Investment analysis
    ├── loan.rs          # Loan and mortgage calculations
    ├── ratios.rs        # Financial ratios
    └── statistics.rs    # Statistical functions
```

## Contributing

We welcome contributions! Here's how to get started:

### Setting up Development Environment
```bash
# Fork and clone the repository
git clone https://github.com/yourusername/rusty-finance.git
cd rusty-finance

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install dependencies and build
cargo build

# Run the test suite
cargo test
```

### Making Changes
1. Create a new branch for your feature
2. Add comprehensive tests for new functionality
3. Ensure all tests pass: `cargo test`
4. Follow Rust conventions and use `cargo fmt`
5. Submit a pull request with a clear description

### Code Style
- Use `cargo fmt` for consistent formatting
- Follow Rust naming conventions
- Add comprehensive documentation for public APIs
- Write unit tests for all new functions

## Changelog

### v0.1.0 (Latest)
- **🚀 Modern Architecture**: Complete refactor with modular design
- **🔧 Updated Dependencies**: Latest versions of clap, comfy-table, and rust_decimal
- **🛡️ Enhanced Error Handling**: Comprehensive error types with thiserror
- **📊 Improved Output**: Beautiful table formatting with proper alignment
- **🧪 Comprehensive Testing**: Unit tests for all calculation modules
- **📝 Better Documentation**: Improved CLI help and code documentation

## License
This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.
