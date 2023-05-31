# Rusty Finance

Rusty Finance is a command-line tool for performing financial calculations. It supports various financial calculations such as simple interest, compound interest, present value, future value, net present value (NPV), amortization schedule, and average calculation.

## Usage

To use Rusty Finance, you can run the executable and provide the desired command and corresponding options as arguments. Here are the available commands:

- `interest`: Calculates simple interest.
- `compound-interest`: Calculates compound interest.
- `present-value`: Calculates present value.
- `future-value`: Calculates future value.
- `npv`: Calculates net present value.
- `amortization`: Calculates an amortization schedule.

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
## License
This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.
