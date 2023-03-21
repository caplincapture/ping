mod error;
use error::Error;
use rawsock::open_best_library;

use maplit::*;
use serde::Deserialize;
use wmi::{query::FilterValue, COMLibrary, WMIConnection};

fn main() -> Result<(), Error> {
    let com_con = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_con.into())?;

    #[derive(Deserialize, Debug)]
    #[allow(non_camel_case_types, non_snake_case)]
    struct Win32_IP4RouteTable {
        InterfaceIndex: i64,
    }

    let route: Win32_IP4RouteTable = wmi_con
        .filtered_query(&hashmap! {
            "Destination".into() => FilterValue::Str("0.0.0.0"),
        })?
        .drain(..)
        .next()
        .expect("should have a default network interface");
    println!("{:#?}", route);

    #[derive(Deserialize, Debug)]
    #[allow(non_camel_case_types, non_snake_case)]
    struct Win32_NetworkAdapter {
        GUID: String,
    }
    let adapter: Win32_NetworkAdapter = wmi_con
        .filtered_query(&hashmap! {
            "InterfaceIndex".into() => FilterValue::Number(route.InterfaceIndex),
        })?
        .drain(..)
        .next()
        .expect("default network interface should exist");
    println!("{:#?}", adapter);

    let lib = open_best_library()?;
    let interface_name = format!(r#"\Device\NPF_{}"#, adapter.GUID);
    lib.open_interface(&interface_name)?;

    println!("Interface opened!");
    Ok(())
}