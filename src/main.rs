extern crate hyper;
extern crate html5ever;
extern crate tendril;


use hyper::client::Client;
use std::io::Read;

use tendril::*;
use tendril::fmt::UTF8;

use html5ever::tokenizer::{TokenSink, Token, TokenizerOpts, ParseError};
use html5ever::tokenizer::{TagToken, StartTag, Tag};
use html5ever::driver::{tokenize_to, one_input};

use std::thread;

use std::sync::{Mutex, Arc};

struct LinkFinder {
    links: Vec<String>
}

impl TokenSink for LinkFinder {

    fn process_token(&mut self, token: Token) {
        match token {
            TagToken(tag @Tag{kind: StartTag, ..}) => {  
                
                if (tag.name.as_slice() == "a" ) {
                    for attr in tag.attrs {
                        if attr.name.local.as_slice() == "href" {
                            //println!("{:?}", attr.value);
                            self.links.push(String::from(attr.value));
                        }
                        
                    }
                    
                }

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
    //println!("{}", body);


    let mut sink = LinkFinder{links: vec![]};
    sink = tokenize_to(sink, one_input(Tendril::from(body)), Default::default());
    let l = sink.links;

    let data = Arc::new(Mutex::new(l));
    let mut handlers = vec![];


    for i in 0..5 {
            let data = data.clone();
            handlers.push(thread::spawn(move || { 
                loop {
                    let link = {
                        data.lock().unwrap().pop()
                    };
                    let mut data = data.lock().unwrap();
                    match data.pop() {
                        Some(d) => println!("Thread: {}, link {}", i,d),
                        None => break
                    }
                    println!("Hello")
                }
            }));
    }
    for handler in handlers {
        handler.join();
    }

}
