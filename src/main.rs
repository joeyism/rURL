#![feature(collections)]
#![feature(core)]
#![warn(non_snake_case)]
#![warn(unused_mut)]
extern crate hyper;

use std::io::Read;
use std::env;
use std::convert::AsRef;

use hyper::Client;
use hyper::client::response::Response;
use hyper::client::IntoUrl;
use hyper::header::Connection;

fn print_type_of<T>(_: &T) -> () {
    let type_name =
        unsafe {
            std::intrinsics::type_name::<T>()
        };
    println!("{}", type_name);
}

fn responseFcn(mut res: Response){
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    println!("{}",body);
}

fn main(){
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let mut client = Client::new();

        match args[1].to_uppercase().as_ref(){
            "GET" => {
                let ref url = args[2];
                let mut res = client.get(url)
                    .header(Connection::close())
                    .send().unwrap();
                responseFcn(res);
            },
            "POST" =>{
                let ref url = args[2];
                let ref body = args[3];
                let mut res = client.post(url)
                    .body(body)
                    .send().unwrap();
                responseFcn(res);
            },
            _ => println!("I don't know what this means"),
        }
    } else {
        println!("Please provide valid arguments");
    }
}
