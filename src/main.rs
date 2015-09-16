extern crate hyper;
use hyper::client::Client;
use std::io::Read;


fn main() {
    let client = Client::new();

    let res = client.get("https://www.heise.de").send();


    let mut response = match res {
            Ok(x)  => x,
            Err(err) =>  panic!("{:?}", err)
    };

    let mut body = String::new();
    response.read_to_string(&mut body);
    println!("{}", body);

}
