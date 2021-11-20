# TODO

## Reading CSV
- [x] Input parameter
  - [x] handle parameter missing
- [x] Read file synchronously using buffer
  - [x] Handle file not found
- [x] Read file using CSV lib 
  - [x] Skip first line
  - [x] Handle wrong format
  - [ ] Harden parsing (headers, double quotes)... 
- [x] Parse a line
  - [x] Not enough columns : BLOCKING : return Parsing error (can affect other records)
  - [x] type : Transaction Type not supported: NON BLOCKING : say None & continue with the file
  - [x] client : missing: NON BLOCKING : say None & continue with the file
  - [x] tx : missing: NON BLOCKING : say None & continue with the file
  - [x] amount : missing: NON BLOCKING : say None & continue with the file (check later semantically if deposit or withdrawal)

- [ ] Improve CSV Perf : See [here](https://docs.rs/csv/1.1.6/csv/tutorial/index.html#amortizing-allocations)
  - [ ] Handle non-text format ? Looks complicated ....https://github.com/keirlawson/binaryornot/blob/master/src/lib.rs
- [ ] Avoid creating new Vec on each line (aka: Reuse same buffer) see https://stackoverflow.com/questions/45882329/read-large-files-line-by-line-in-rust
- [ ] Read file async streaming like (but treat sequentially): https://tokio.rs/tokio/tutorial/io


## Models / Structs
- [x] TransactionKind : an Enum
- [x] RawTransaction : to hold raw data just parsed by CSV operator
- [x] RawTransactionsList : abstract vec to hold all raw transactions parsed
- [x] Structs for every transaction :
  - [x] Deposit
    - [x] Implement TryFrom<RawTransaction>
    - [x] Edge case : Missing required value
    - [x] Edge case : Account does not exist / create it
    - [x] Edge case : Amount is not a number
  - [x] Withdrawal
    - [x] Edge case : Missing required value
    - [x] Edge case : Account does not exist / create it
    - [x] Edge case : Amount is not a number
  - [ ] Dispute
  - [ ] Resolve
  - [ ] Chargeback
- [x] AccountBalance : lock, available, held 
- [x] BalanceBook : (aka the state) a HashSet of accounts (client, amount, status)
- [x] Handle amount convertion to Decimal (using dedicated crate))

## Process / Applying trasactions
- [x] Deposit
    - [x] Increase available amount
    - [x] Edge case : Above max amount (Skip)
- [ ] Withdrawal
    - [ ] Decrease available amount
    - [ ] Edge case : not enough funds (Skip)
- [ ] Dispute
- [ ] Resolve
- [ ] Chargeback

## Generate output
- [ ] Compute Total amount dynamically
- [ ] Write file

## Misc:
- [ ] Logs
- [ ] Help (-h on command line)
- [ ] README


## Polish:
- [ ] Document code
- [ ] Apply Clippy
- [ ] CI ?

## Problems / Edges cases
- [ ] Precision problems? 

Bonus:
- Support transfer ?
- Fraud status ?

Explain: transaction id or timestamp missing.  