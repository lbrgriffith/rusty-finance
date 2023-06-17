# Rusty Finance CLI

![Rust](https://img.shields.io/badge/Rust-Programming%20Language-orange?logo=rust)

Rusty Finance is a command-line tool for performing financial calculations. It supports various financial calculations such as simple interest, compound interest, present value, future value, net present value (NPV), amortization schedule, IRR, mode, and more.

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
- `irr`: Calculates the internal rate of return.
- `variance`: Measures how much the numbers in a data set deviate from the mean.
- `probability`: Calculates the probability of an event.
- `capm`: Calculates the expected return using the Capital Asset Pricing Model.
- `loan-payment`: Calculate loan payments, including the monthly payment amount, total interest paid, and the loan payoff date.
- `break-even-units`: Calculate the number of units a business needs to sell to break even.

## Examples

### Calculate Simple Interest

To calculate simple interest, use the `interest` command and provide the required options:

```shell
$ rusty-finance interest --principal 1000 --rate 0.05 --time 2
```

### Result
This will output the simple interest amount:

```
+-----------+------+------+-----------------+
| Principal | Rate | Time | Simple Interest |
+-----------+------+------+-----------------+
| $1,000    | 0.05 | 2    | $100            |
+-----------+------+------+-----------------+
```

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

### Result
This will output the present value amount.

```
+------+-----------+
| Year | Amount    |
+------+-----------+
| 1    | $1,004.16 |
| 2    | $1,008.35 |
| 3    | $1,012.55 |
+------+-----------+
```

### Calculate Future Value
To calculate _future value_, use the `future-value` command and provide the required options:
```shell
$ rusty-finance future-value --present-value 1000 --rate 0.05 --time 3
```

### Result
This will output the future value amount:
```
+---------------+------+------+--------------+
| Present Value | Rate | Time | Future Value |
+---------------+------+------+--------------+
| $1,000        | 0.05 | 3    | $1,157.62    |
+---------------+------+------+--------------+
```

### Calculate Net Present Value (NPV)
To calculate _net present value_, use the `npv` command and provide the required options:
```shell
$ rusty-finance npv --initial-investment 10000 --cash-inflow 2000 --discount-rate 0.1 --lifespan 5
```

### Result
This will output a table showing the discounted cash flow for each year and the net present value:
```
+------+-------------+----------------------+
| Year | Cash Inflow | Discounted Cash Flow |
+------+-------------+----------------------+
| 1    | $2,000      | $1,818.18            |
| 2    | $2,000      | $1,652.89            |
| 3    | $2,000      | $1,502.62            |
| 4    | $2,000      | $1,366.02            |
| 5    | $2,000      | $1,241.84            |
+------+-------------+----------------------+
Net Present Value (NPV): $-2,418.42
```

### Calculate Amortization Schedule
To calculate an _amortization schedule_, use the `amortization` command and provide the required options:
```shell
$ rusty-finance amortization -a 100000 -i 0.05 -t 30
```

Result:
```
+-------+-----------+----------+-------------------+
| Month | Principal | Interest | Remaining Balance |
+-------+-----------+----------+-------------------+
| 1     | $643.98   | $416.66  | $99,356.01        |
| 2     | $646.67   | $413.98  | $98,709.33        |
| 3     | $649.36   | $411.28  | $98,059.97        |
| 4     | $652.07   | $408.58  | $97,407.90        |
| 5     | $654.78   | $405.86  | $96,753.11        |
| 6     | $657.51   | $403.13  | $96,095.59        |
| 7     | $660.25   | $400.39  | $95,435.33        |
| 8     | $663.00   | $397.64  | $94,772.33        |
| 9     | $665.77   | $394.88  | $94,106.56        |
| 10    | $668.54   | $392.11  | $93,438.01        |
| 11    | $671.33   | $389.32  | $92,766.68        |
| 12    | $674.12   | $386.52  | $92,092.55        |
| 13    | $676.93   | $383.71  | $91,415.62        |
| 14    | $679.75   | $380.89  | $90,735.86        |
| 15    | $682.58   | $378.06  | $90,053.27        |
...
| 109   | $1,009.03 | $51.62   | $11,380.71        |
| 110   | $1,013.23 | $47.41   | $10,367.48        |
| 111   | $1,017.45 | $43.19   | $9,350.02         |
| 112   | $1,021.69 | $38.95   | $8,328.32         |
| 113   | $1,025.95 | $34.70   | $7,302.37         |
| 114   | $1,030.22 | $30.42   | $6,272.14         |
| 115   | $1,034.52 | $26.13   | $5,237.62         |
| 116   | $1,038.83 | $21.82   | $4,198.79         |
| 117   | $1,043.16 | $17.49   | $3,155.63         |
| 118   | $1,047.50 | $13.14   | $2,108.12         |
| 119   | $1,051.87 | $8.78    | $1,056.25         |
| 120   | $1,056.25 | $4.40    | $-0.00            |
+-------+-----------+----------+-------------------+
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
### Results:

```
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
### Result
```
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
### Result
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

### Result
```
+-----------------------+--------------+----------------+
| Cash Flows            | Initial Cost | Payback Period |
+-----------------------+--------------+----------------+
| [100.0, 200.0, 300.0] | $500         | 3              |
+-----------------------+--------------+----------------+
```

Feel free to explore other financial calculations provided by `rusty-finance` using the available commands documented in the tool's usage section.

```bash
cargo run -- payback-period --initial-cost <initial-cost> --cash-flows <cash-flow-1> --cash-flows <cash-flow-2> ...
```
Replace `<fixed-costs>` with the total fixed costs incurred by the business, `<variable-costs>` with the variable costs per unit, and `<price-per-unit>` with the price per unit of the product or service.

For example, to perform a Break-Even Analysis with fixed costs of $5000, variable costs of $10 per unit, and a price per unit of $20, you would run the following command:
```shell
cargo run -- break-even --fixed-costs 5000 --variable-costs 10 --price-per-unit 20
```
### Result
The tool will display the Break-Even Analysis results, including the Break-Even point in terms of the number of units that need to be sold and the total revenue required to cover all costs.
```
+----------------------------+---------+
| Metric                     | Value   |
+============================+=========+
| Break-Even Point (units)   | $500    |
+----------------------------+---------+
| Total Revenue Required ($) | $10,000 |
+----------------------------+---------+
```


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

### Result
```
+--------------------------+---------+
| Depreciation Type        | Amount  |
+--------------------------+---------+
| Straight Line            | $8,000  |
| Double Declining Balance | $16,800 |
+--------------------------+---------+
```

## Calculate IRR
**IRR (Internal Rate of Return)**: This function calculates the internal rate of return for a series of cash flows. It helps evaluate the profitability of an investment by determining the discount rate at which the net present value (NPV) of the cash flows becomes zero.

## Example
To calculate the internal rate of return (IRR) for a series of cash flows, use the irr command followed by the cash flow values. For example:

```shell
rusty-finance irr -- 1000 -500 300 200 -100
```

This command will calculate the IRR based on the provided cash flows: `1000, -500, 300, 200, -100`.

### Result
```
Internal Rate of Return (IRR): 186.37%
```
## Variance
Calculating the variance and standard deviation are common statistical measures used to understand the spread or dispersion of a series of numbers. They provide insights into the variability or consistency of the data set. Here's an overview of these calculations:
```shell
rusty-finance variance 10 20 30 40 50
```

### Result
```
+----------+-----+
| Number   |     |
+----------+-----+
| 10       |     |
| 20       |     |
| 30       |     |
| 40       |     |
| 50       |     |
| Variance | 200 |
+----------+-----+
```

## Example: Calculating Probability
To calculate the probability of getting heads when flipping a fair coin, you can use the following command:
```shell
rusty-finance probability --successes 3 --trials 5
```
### Result
```
+-----------+--------+-------------+
| Successes | Trials | Probability |
+-----------+--------+-------------+
| 3         | 5      | 60.00%      |
+-----------+--------+-------------+
```

## Calculate Expected Return (CAPM)
To calculate the expected return using the Capital Asset Pricing Model (CAPM), use the capm command followed by the required options. The CAPM formula requires the risk-free rate, beta, and market return.

```shell
rusty-finance capm --risk-free-rate 0.05 --beta 1.2 --market-return 0.1
```
### Result
This command will calculate the expected return based on the provided risk-free rate, beta, and market return.
```
+----------------+------+---------------+-----------------+
| Risk-Free Rate | Beta | Market Return | Expected Return |
+----------------+------+---------------+-----------------+
| 5.00%          | 1.2  | 10.00%        | 11.00%          |
+----------------+------+---------------+-----------------+
```

## Loan Calculator
Calculate loan payments, including the monthly payment amount, total interest paid, and the loan payoff date.

```shell
rusty-finance loan-payment --principal 100000 --interest-rate 5 --loan-term 10
```

### Result
+-----------+---------------+-----------+-----------------+
| Principal | Interest Rate | Loan Term | Monthly Payment |
+-----------+---------------+-----------+-----------------+
| $100,000  | 5.00%         | 10 years  | $1,060.65       |
+-----------+---------------+-----------+-----------------+

## Break-Even Point in Units
Calculates the number of units a business needs to sell to break even, taking into account fixed costs, variable costs per unit, and selling price per unit.

```shell
rusty-finance  break-even-units --fixed-costs 1000 --variable-costs 10 --price-per-unit 20
```

### Reesult
```
+--------------------------+-------+
| Metric                   | Value |
+==========================+=======+
| Break-Even Point (Units) | 100   |
+--------------------------+-------+
```

> **Thanks!**
> Enjoy using Rusty Finance for your financial calculations! If you have any further questions, feel free to ask.

## License
This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.
