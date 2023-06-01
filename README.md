# Rusty Finance

![Rust](https://img.shields.io/badge/Rust-Programming%20Language-orange?logo=rust)

Rusty Finance is a command-line tool for performing financial calculations. It supports various financial calculations such as simple interest, compound interest, present value, future value, net present value (NPV), amortization schedule, and mode.

## Usage

To use Rusty Finance, you can run the executable and provide the desired command and corresponding options as arguments. Here are the available commands:

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

## Examples

### Calculate Simple Interest

To calculate _simple interest_, use the `interest` command and provide the required options:

```shell
$ rusty-finance interest --principal 1000 --rate 0.05 --time 2
```
This will output the simple interest amount.

### Calculate Compound Interest
To calculate _compound interest_, use the `compound-interest` command and provide the required options:
```shell
$ rusty-finance compound-interest --principal 1000 --rate 0.05 --n 12 --t 3
```
This will output a table showing the amount at the end of each year.

### Calculate Present Value
To calculate _present value_, use the `present-value` command and provide the required options:
```shell
$ rusty-finance present-value --future-value 1000 --rate 0.05 --time 3
```
This will output the present value amount.

### Calculate Future Value
To calculate _future value_, use the `future-value` command and provide the required options:
```shell
$ rusty-finance future-value --present-value 1000 --rate 0.05 --time 3
```
This will output the future value amount.

### Calculate Net Present Value (NPV)
To calculate _net present value_, use the `npv` command and provide the required options:
```shell
$ rusty-finance npv --initial-investment 10000 --cash-inflow 2000 --discount-rate 0.1 --lifespan 5
```
This will output a table showing the discounted cash flow for each year and the net present value.

### Calculate Amortization Schedule
To calculate an _amortization schedule_, use the `amortization` command and provide the required options:
```shell
$ rusty-finance amortization -a 100000 -i 0.05 -t 30
```
### Calculate Return on Investment (ROI)
```shell
$ rusty-finance roi --net-profit 5000 --cost-of-investment 20000
```
### Calculate Average
To calculate the average of a series of numbers, use the average command and provide the numbers:
```shell
$ rusty-finance average 5 10 15 20 25
```
Result:
```shell
+---------+----+
| Number  |    |
+---------+----+
| 5       |    |
| 10      |    |
| 15      |    |
| 20      |    |
| 25      |    |
| Average | 15 |
+---------+----+
```

### Calculate Mode
To calculate the mode of a series of numbers, use the mode command and provide the numbers as arguments:
```shell
$ rusty-finance mode 5 10 15 10 20

```
The result is as follows:
```shell
+------+
| Mode |
+------+
| 10   |
+------+
```

### Calculate Medium
To calculate the median of a series of numbers:
```shell
cargo run medium --numbers 5 10 15 20 25
```
Results:
```
 Numbers | Medium 
---------+--------
 5       |  
 10      |  
 15      |  
 20      |  
 25      |  
 Result  | 15
```
## Payback Period Calculation

The `rusty-finance` tool provides a command to calculate the payback period of an investment. The payback period is the length of time required to recoup the initial investment through cash inflows. It is a simple metric used to assess the risk and profitability of an investment.

To calculate the payback period, use the following command:

Replace `<initial-cost>` with the initial investment cost and `<cash-flow-i>` with the cash inflow values for each period. You can provide multiple `--cash-flows` arguments to account for cash inflows occurring over multiple periods.

For example, to calculate the payback period for an investment with an initial cost of $500 and cash inflows of $100, $200, and $300, you would run the following command:

```shell
cargo run -- payback-period --initial-cost 500 --cash-flows 100 --cash-flows 200 --cash-flows 300
```
The tool will display the payback period in terms of the number of periods it takes to recover the initial investment.

Please note that the `rusty-finance` tool assumes the cash inflows are evenly spaced over the investment period.

Feel free to explore other financial calculations provided by `rusty-finance` using the available commands documented in the tool's usage section.

```bash
cargo run -- payback-period --initial-cost <initial-cost> --cash-flows <cash-flow-1> --cash-flows <cash-flow-2> ...
```
Replace `<fixed-costs>` with the total fixed costs incurred by the business, `<variable-costs>` with the variable costs per unit, and `<price-per-unit>` with the price per unit of the product or service.

For example, to perform a Break-Even Analysis with fixed costs of $5000, variable costs of $10 per unit, and a price per unit of $20, you would run the following command:
```shell
cargo run -- break-even --fixed-costs 5000 --variable-costs 10 --price-per-unit 20
```
The tool will display the Break-Even Analysis results, including the Break-Even point in terms of the number of units that need to be sold and the total revenue required to cover all costs.

Please note that the rusty-finance tool assumes a linear relationship between the number of units sold and the costs and revenue.

Feel free to explore other financial calculations provided by rusty-finance using the available commands documented in the tool's usage section.


## Break-Even Analysis

The `rusty-finance` tool provides a command to perform a Break-Even Analysis. Break-Even Analysis is a financial calculation used to determine the point at which total revenue equals total costs, resulting in neither profit nor loss. It helps businesses understand the minimum level of sales required to cover all expenses.

To perform a Break-Even Analysis, use the following command:

```bash
cargo run -- break-even --fixed-costs <fixed-costs> --variable-costs <variable-costs> --price-per-unit <price-per-unit>
```

## Depreciation
The depreciation command calculates the decrease in value of an asset over time. It supports different depreciation methods, such as straight-line and double-declining balance.

To calculate the depreciation, use the following command:
```shell
rusty-finance depreciation --initial-value <INITIAL_VALUE> --salvage-value <SALVAGE_VALUE> --useful-life <USEFUL_LIFE> --depreciation-method <DEPRECIATION_METHOD>
```

- `--initial-value`: The initial value of the asset.
- `--salvage-value`: The salvage value of the asset.
- `--useful-life`: The useful life of the asset.
- `--depreciation-method`: The method of depreciation (e.g., straight-line, double-declining-balance).

Example:
```shell
rusty-finance depreciation --initial-value 50000 --salvage-value 10000 --useful-life 5 --depreciation-method straight-line
```
This will calculate the depreciation using the straight-line method and display the result in a table.

Note: Additional depreciation methods can be added to the Depreciation struct in the code.

Feel free to adjust the description as needed. Let me know if there's anything else I can help you with!

---
## License
This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.
