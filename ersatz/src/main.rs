mod error;
use error::Error;
use rawsock::open_best_library;

use maplit::*;
use serde::Deserialize;
use wmi::{query::FilterValue, COMLibrary, WMIConnection};

mod vls;
mod ipv4;


mod loadlibrary;
mod netinfo;

fn main() -> Result<(), Error> {
    let interface_name = format!(r#"\Device\NPF_{}"#, netinfo::default_nic_guid()?);
    let lib = open_best_library()?;
    lib.open_interface(&interface_name)?;

    println!("Interface opened!");
    Ok(())
}
