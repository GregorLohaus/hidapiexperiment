extern crate hidapi;
use std::fmt::{Display};
use std::io::Read;
use std::sync::{Arc, Mutex};
use hidapi::{HidApi};
use std::thread;

fn main() {
    let mut api:HidApi;
    match HidApi::new() {
        Ok(r) => {api = r;}
        Err(e) => {eprintln!("Api Initialization error: {}",e); return; }
    }
    let result_vec = Arc::new(Mutex::new(vec![]));
    let mut handles = vec![];
    for device in api.device_list() {
        let mut path: &str = "";
        let mut product_string: &str = "";
        match device.product_string() {
            Some(s) => product_string = s,
            None => ()
        }
        match device.path().to_str() {
            Ok(p) => path = p,
            Err(e) => eprintln!("Path String Error: {}",e)
        }

        println!("Product: {}; Path: {}",product_string,path);

        let mut device_correct:bool = false;
        match product_string {
            "CORSAIR K55 RGB PRO Gaming Keyboard" => device_correct = true,
            _ => ()
        }
        if device_correct {
            if let Ok(t) = api.open_path(device.path()) {
                let mut buf: [u8;256] = [0;256];
                let thread_result = result_vec.clone();
                let handle = thread::spawn(move || {
                    loop {
                        match t.read(&mut buf) {
                           Ok(b) => {
                               let mut data_string = String::new();
                               for u in &buf[..b] {
                                   data_string.push_str(&(u.to_string() + "\t"));
                               }
                               let mut r = thread_result.lock().unwrap();
                               (*r).push(data_string);
                           },
                           Err(e) => {
                               dbg!(e);
                           }
                        }
                    }
                });
                handles.push(handle);
            }
        }
    }

    loop {
        let mut r = result_vec.lock().unwrap();
        if let Some(x) = (*r).pop() {
            dbg!(x);
        }
    }

}


