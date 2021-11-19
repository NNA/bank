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
  - [x] transaction Type not supported: NON BLOCKING : say None & continue with the file
  - [ ] Not enough columns : BLOCKING : return Parsing error

- [ ] Improve CSV Perf : See [here](https://docs.rs/csv/1.1.6/csv/tutorial/index.html#amortizing-allocations)
  - [ ] Handle non-text format ? Looks complicated ....https://github.com/keirlawson/binaryornot/blob/master/src/lib.rs
- [ ] Avoid creating new Vec on each line (aka: Reuse same buffer) see https://stackoverflow.com/questions/45882329/read-large-files-line-by-line-in-rust
- [ ] Read file async streaming like (but treat sequentially): https://tokio.rs/tokio/tutorial/io


## Models
- [ ] Clients = Accounts
- [ ] TransactionKinds : an Enum
- [ ] TransactionStatus : Prepared, Applied
- [ ] Transaction : client, kind, amount, status 
- [ ] Ledger : (aka the state) a HashSet of accounts (client, amount, status)

## Applying trasactions
- [ ] Deposit
    - [ ] Client does not exist
    - [ ] Amount is not a number
- [ ] Withdrawal
    - [ ] Not enough funds (Skip)
- [ ] Dispute
- [ ] Resolve
- [ ] Chargeback

## Generate output
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