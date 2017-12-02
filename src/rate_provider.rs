extern crate curl;
extern crate serde;
extern crate serde_json;

use curl::easy::Easy;
use rate::Rate;

// Print a web page onto stdout
pub fn get() -> Option<Vec<Rate>> {
    let mut handle = Easy::new();
    let mut data = Vec::new();
    let mut output = String::new();
    handle.url("https://api.coinmarketcap.com/v1/ticker/?convert=EUR&limit=10").unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
            output.push_str(&String::from_utf8_lossy(new_data));
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
//    println!("{}", handle.response_code().unwrap());

    let deserialized_result: serde_json::Result<Vec<Rate>> = serde_json::from_str(&output);

    match deserialized_result {
        Ok(deserialized) => Some(deserialized),
        Err(e) => {
            print!("{:?}", e);
            None
        }
    }
}
