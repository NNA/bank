use crate::models::ledger::AccountIndex;

use csv::WriterBuilder;

use std::error::Error;
use std::io;

use serde::Serialize;

#[derive(Serialize)]
struct DisplayedBalanceHeader<'a> {
    id: &'a str,
    available: &'a str,
    held: &'a str,
    total: &'a str,
    locked: &'a str,
}

pub fn write_balances(account_balances: AccountIndex) -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new()
        .has_headers(false)
        .from_writer(io::stdout());

    // Manual header because non scalar types not supported as header by CSV library
    wtr.serialize(DisplayedBalanceHeader {
        id: "client",
        available: "available",
        held: "held",
        total: "total",
        locked: "locked",
    })?;

    for (id, balance) in account_balances.iter() {
        // Writing fields one by one so that non structs fields (ex: functions) can be called
        wtr.write_field(id.to_string())?;
        wtr.write_field(balance.available.to_string())?;
        wtr.write_field(balance.held.to_string())?;
        wtr.write_field(balance.total().to_string())?;
        wtr.write_field(balance.locked.to_string())?;
        wtr.write_record(None::<&[u8]>)?;
    }

    wtr.flush()?;

    Ok(())
}
