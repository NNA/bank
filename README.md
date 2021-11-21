# Bank: A toy payment system

## Description

This is a demo payment application.

It reads a CSV file with transactions (Deposit, Withdrawal, Dispute, Resolve & Chargeback) and outputs a status for every account.

## Usage:

```shell
cargo run -- ./tests/fixtures/regular_tx.csv > /tmp/output.txt
```

There is also unit tests & e2e tests. 
```shell
cargo test
```

Some debug info is also available. adapt our log level with `RUST_LOG` env var.


## Choices / Acknowledgment:

- The program does 3 things in order :
  1 - Read file and parse content to transaction lines (parser.rs)
  2 - Compute balance & status of accounts (computer.rs)
  3 - Display account balances in csv format.  

- There are 2 kinds of errors blocking & non-blocking : 
  - Blocking errors deal with incorrect data that might impact several record (missing input, wrong csv format)
  - Non blocking errors impact only some client and thus may be skipped / gracefully ignored as the program continues. This is the case for instance when there if not enough funds available on the account.

## Status:

Not finished yet. All transactions are not suported (missing Dispute, Resolve & Chargeback).


## Areas of improvements: 

- Overral design of Transactions. To much code is duplicated between deposit & withdrawals. Should use Traits, traits Objects to share common behavior. Too many struct and functions are public, and many 'objects' change other object properties, instead of sending messages ("tell don't ask").

- Performance :
  - Stream the file to start parsing transactions & applying transactions before end of file
  - Partition transactions by client so they can be treated in parralel (seems possible as there is no interactions between accounts : transfers)


