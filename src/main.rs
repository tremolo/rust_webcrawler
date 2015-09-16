extern crate hyper;
extern crate html5ever;
extern crate tendril;

use hyper::client::Client;
use std::io::Read;

use tendril::*;

use html5ever::tokenizer::{TokenSink, Token, TokenizerOpts, ParseError};
use html5ever::tokenizer::{TagToken};
use html5ever::driver::{tokenize_to, one_input};

struct TokenLogger;

impl TokenSink for TokenLogger {

    fn process_token(&mut self, token: Token) {
        match token {
            TagToken(tag) => {  
                println!("{:?}", tag) 

            },
            _ =>  {}
        }
        
    }
}

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


    let mut sink = TokenLogger;
    tokenize_to(sink, one_input(Tendril::from(body)), Default::default());


}
