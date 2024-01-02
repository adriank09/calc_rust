# What
A program implemented in Rust that calculates two values based on the specified operation.
# Why
- To serve as a toy app while messing around with Rust.
# How
To use this app, invoke the following in your favorite terminal application.
```
calc <op> [<first_value> <second_value>]
```
1. Required arguments
- `calc`: Name of the app.
- `<op>`: Type of calculation to be performed. Accepted values are: `add` (addition), `sub` (subtraction), `mul` (multiplication), `div` (division) or `hist` (history viewing)
2. Optional arguments
- `<first_value>`: The first value. Must be specified if `<op>` is not `hist`.
- `<second_value>`: The second value. Must be specified if `<op>` is not `hist`.
## Examples
- To calculate the addition of 2 and 3
```
calc add 2 3
```
- To view past calculations
```
calc hist
```