
extern crate sapper;
extern crate sapper_session;
extern crate env_logger;
#[macro_use]
extern crate log;

use sapper::{SapperApp, SapperAppShell, Request, Response, Result};
use sapper_session::session_val;


mod biz;
use biz::Biz;



#[derive(Clone)]
struct MyApp;
// must impl it
// total entry and exitice
impl SapperAppShell for MyApp {
    fn before(&self, req: &mut Request) -> Result<()> {
        println!("{}", "in SapperAppShell before.");
        session_val(req, "TestKey");
        
        Ok(())
    }
    
    fn after(&self, req: &Request, res: &mut Response) -> Result<()> {
        println!("{}", "in SapperAppShell after.");
        
        Ok(())
    }
}



pub fn main() {
    env_logger::init().unwrap();
    
    let mut sapp = SapperApp::new();
    sapp.address("127.0.0.1")
        .port(1337)
        .with_shell(Box::new(MyApp))
        .add_module(Box::new(Biz));
    
    println!("Listening on http://127.0.0.1:1337");
    sapp.run_http();
    
}
