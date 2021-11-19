# TODO

## Reading CSV
- [ ] Input parameter
  - [ ] handle parameter missing
- [ ] Read file in streaming but treat sequentially, buffer like
  - [ ] Handle file not found
  - [ ] Handle non-text format ?
- [ ] Skip first line
- [ ] Parse a line
  - [ ] Handle parsing error (type not supported, not enough columns) : skip the line


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