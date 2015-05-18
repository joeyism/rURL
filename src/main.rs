extern crate hyper;

use std::io::Read;

use hyper::Client;
use hyper::header::Connection;

fn main(){
    let mut client = Client::new();

    let mut res = client.get("http://www.joeyism.com")
        .header(Connection::close())
        .send().unwrap();
    assert_eq!(res.status, hyper::Ok);


    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Response: {}",body);

}
