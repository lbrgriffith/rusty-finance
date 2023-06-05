# Rusty Finance CLI

![Rust](https://img.shields.io/badge/Rust-Programming%20Language-orange?logo=rust)

Rusty Finance is a command-line tool for performing financial calculations. It supports various financial calculations such as simple interest, compound interest, present value, future value, net present value (NPV), amortization schedule, IRR, and mode.

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

## Examples

### Calculate Simple Interest

To calculate _simple interest_, use the `interest` command and provide the required options:

```shell
$ rusty-finance interest --principal 1000 --rate 0.05 --time 2
```
This will output the simple interest amount:
```shell
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
This will output the present value amount.

```bash
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
| 16    | $685.43   | $375.22  | $89,367.84        |
| 17    | $688.28   | $372.36  | $88,679.55        |
| 18    | $691.15   | $369.49  | $87,988.39        |
| 19    | $694.03   | $366.61  | $87,294.36        |
| 20    | $696.92   | $363.72  | $86,597.43        |
| 21    | $699.83   | $360.82  | $85,897.59        |
| 22    | $702.74   | $357.90  | $85,194.85        |
| 23    | $705.67   | $354.97  | $84,489.17        |
| 24    | $708.61   | $352.03  | $83,780.55        |
| 25    | $711.56   | $349.08  | $83,068.98        |
| 26    | $714.53   | $346.12  | $82,354.45        |
| 27    | $717.51   | $343.14  | $81,636.94        |
| 28    | $720.50   | $340.15  | $80,916.44        |
| 29    | $723.50   | $337.15  | $80,192.93        |
| 30    | $726.51   | $334.13  | $79,466.41        |
| 31    | $729.54   | $331.11  | $78,736.87        |
| 32    | $732.58   | $328.07  | $78,004.28        |
| 33    | $735.63   | $325.01  | $77,268.65        |
| 34    | $738.70   | $321.95  | $76,529.94        |
| 35    | $741.78   | $318.87  | $75,788.16        |
| 36    | $744.87   | $315.78  | $75,043.29        |
| 37    | $747.97   | $312.68  | $74,295.32        |
| 38    | $751.09   | $309.56  | $73,544.23        |
| 39    | $754.22   | $306.43  | $72,790.01        |
| 40    | $757.36   | $303.29  | $72,032.64        |
| 41    | $760.51   | $300.13  | $71,272.12        |
| 42    | $763.68   | $296.96  | $70,508.44        |
| 43    | $766.86   | $293.78  | $69,741.57        |
| 44    | $770.06   | $290.58  | $68,971.50        |
| 45    | $773.27   | $287.38  | $68,198.23        |
| 46    | $776.49   | $284.15  | $67,421.73        |
| 47    | $779.73   | $280.92  | $66,642.00        |
| 48    | $782.98   | $277.67  | $65,859.02        |
| 49    | $786.24   | $274.41  | $65,072.78        |
| 50    | $789.51   | $271.13  | $64,283.26        |
| 51    | $792.80   | $267.84  | $63,490.45        |
| 52    | $796.11   | $264.54  | $62,694.34        |
| 53    | $799.42   | $261.22  | $61,894.91        |
| 54    | $802.75   | $257.89  | $61,092.15        |
| 55    | $806.10   | $254.55  | $60,286.05        |
| 56    | $809.46   | $251.19  | $59,476.58        |
| 57    | $812.83   | $247.81  | $58,663.75        |
| 58    | $816.22   | $244.43  | $57,847.52        |
| 59    | $819.62   | $241.03  | $57,027.90        |
| 60    | $823.03   | $237.61  | $56,204.86        |
| 61    | $826.46   | $234.18  | $55,378.39        |
| 62    | $829.91   | $230.74  | $54,548.48        |
| 63    | $833.36   | $227.28  | $53,715.11        |
| 64    | $836.84   | $223.81  | $52,878.27        |
| 65    | $840.32   | $220.32  | $52,037.94        |
| 66    | $843.83   | $216.82  | $51,194.11        |
| 67    | $847.34   | $213.30  | $50,346.76        |
| 68    | $850.87   | $209.77  | $49,495.89        |
| 69    | $854.42   | $206.23  | $48,641.46        |
| 70    | $857.98   | $202.67  | $47,783.48        |
| 71    | $861.55   | $199.09  | $46,921.92        |
| 72    | $865.14   | $195.50  | $46,056.78        |
| 73    | $868.75   | $191.90  | $45,188.03        |
| 74    | $872.37   | $188.28  | $44,315.65        |
| 75    | $876.00   | $184.64  | $43,439.65        |
| 76    | $879.65   | $180.99  | $42,559.99        |
| 77    | $883.32   | $177.33  | $41,676.67        |
| 78    | $887.00   | $173.65  | $40,789.67        |
| 79    | $890.69   | $169.95  | $39,898.97        |
| 80    | $894.40   | $166.24  | $39,004.56        |
| 81    | $898.13   | $162.51  | $38,106.42        |
| 82    | $901.87   | $158.77  | $37,204.54        |
| 83    | $905.63   | $155.01  | $36,298.91        |
| 84    | $909.40   | $151.24  | $35,389.50        |
| 85    | $913.19   | $147.45  | $34,476.30        |
| 86    | $917.00   | $143.65  | $33,559.30        |
| 87    | $920.82   | $139.83  | $32,638.47        |
| 88    | $924.66   | $135.99  | $31,713.81        |
| 89    | $928.51   | $132.14  | $30,785.29        |
| 90    | $932.38   | $128.27  | $29,852.91        |
| 91    | $936.26   | $124.38  | $28,916.64        |
| 92    | $940.16   | $120.48  | $27,976.47        |
| 93    | $944.08   | $116.56  | $27,032.39        |
| 94    | $948.02   | $112.63  | $26,084.37        |
| 95    | $951.97   | $108.68  | $25,132.40        |
| 96    | $955.93   | $104.71  | $24,176.46        |
| 97    | $959.91   | $100.73  | $23,216.54        |
| 98    | $963.91   | $96.73   | $22,252.62        |
| 99    | $967.93   | $92.71   | $21,284.69        |
| 100   | $971.96   | $88.68   | $20,312.72        |
| 101   | $976.01   | $84.63   | $19,336.70        |
| 102   | $980.08   | $80.56   | $18,356.61        |
| 103   | $984.16   | $76.48   | $17,372.44        |
| 104   | $988.26   | $72.38   | $16,384.17        |
| 105   | $992.38   | $68.26   | $15,391.79        |
| 106   | $996.52   | $64.13   | $14,395.26        |
| 107   | $1,000.67 | $59.98   | $13,394.59        |
| 108   | $1,004.84 | $55.81   | $12,389.74        |
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

## Calculate IRR
**IRR (Internal Rate of Return)**: This function calculates the internal rate of return for a series of cash flows. It helps evaluate the profitability of an investment by determining the discount rate at which the net present value (NPV) of the cash flows becomes zero.

## Example
To calculate the internal rate of return (IRR) for a series of cash flows, use the irr command followed by the cash flow values. For example:

```shell
rusty-finance irr -- 1000 -500 300 200 -100
```

This command will calculate the IRR based on the provided cash flows: `1000, -500, 300, 200, -100`.

```
Internal Rate of Return (IRR): 186.37%
```
## Variance
Calculating the variance and standard deviation are common statistical measures used to understand the spread or dispersion of a series of numbers. They provide insights into the variability or consistency of the data set. Here's an overview of these calculations:
```shell
rusty-finance variance 10 20 30 40 50
```

Results:
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
rusty-finance probability --successes 1 --trials 1
```
### Result
```
```
Successes: 1
Trials:    1
Probability: 0.5 (50.0%)
---
## License
This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.
