# CLI Financial Tool

![Rust](https://img.shields.io/badge/Rust-Programming%20Language-orange?logo=rust)

CLI Financial Tool is an open-source command-line interface (CLI) tool for performing financial calculations. The application is written in Rust and designed to be easy to use, efficient, and powerful. It leverages the power of the `clap` library to parse command-line arguments.

## Features

Our CLI tool currently supports the following financial calculation:

- **Simple Interest:** Calculates the simple interest on a principal amount, given the rate of interest and the time period.

## Getting Started

To start using the CLI Financial Tool, you will need Rust installed on your machine. You can download Rust from [the official website](https://www.rust-lang.org/tools/install).

Then, clone this repository to your local machine:

```bash
git clone https://github.com/lbrgriffith/rusty-finance.git
```
Next, navigate into the directory and build the project:

```bash
cd CLI-Financial-Tool
cargo build
```
## Usage
Once you have successfully built the project, you can start using the tool. Here is how you can use the simple interest calculation feature:

```bash
cargo run -- interest --principal <principal> --rate <rate> --time <time>
```
Where:
`<principal>` is the initial amount of money that you have or plan to invest.
`<rate>` is the rate of interest per time period. This is a decimal, so a rate of 5% would be input as 0.05.
`<time>` is the time that the money is invested for.

## Example
For instance, to calculate the simple interest on a principal of 1000 with an interest rate of 5% per period for 1 period, you would run:

```bash
cargo run -- interest --principal 1000 --rate 0.05 --time 1
```
The output will be:

```bash
The simple interest is: 50
```
## Contributing
We welcome contributions to this project! If you find a bug or think of a new feature, please open an issue. If you're able to implement it yourself, we would appreciate a pull request.


