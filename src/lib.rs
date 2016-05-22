

extern crate sapper;

use sapper::header::{Cookie, CookiePair};
use sapper::header::SetCookie;

use sapper::{Request, Response, Result, Key};

pub struct SessionCookie;
impl Key for SessionCookie { type Value = String; }

pub fn process(req: &mut Request, ckey: &str) -> Result<()> {
    
    let mut cookie_value: Option<String> = None;
    match req.headers().get::<Cookie>() {
        Some(ref value) => {
            let Cookie(ref ckvec) = **value;
            let cookie_vec = ckvec.iter()
                                .filter(|item: &&CookiePair| item.name == ckey.to_owned())
                                .take(1)
                                .collect::<Vec<&CookiePair>>();
            
            if cookie_vec.len() == 1 {
                let cookie_obj = cookie_vec[0];
                cookie_value = Some(cookie_obj.value.clone());
            }
            
            
        },
        None => {
            println!("no cookie in headers");
        }
    }
    
    cookie_value.and_then(|val| {
        req.ext_mut().insert::<SessionCookie>(val);
        Some(())
    });
    
    // if cookie_value.is_some() {
    //     req.ext_mut().insert::<SAppCookieValue>(cookie_value.unwrap());
    // }
    
    Ok(())
}


// pub fn make_uuid() -> String {
    
    
// }

// library function
pub fn set_cookie(res: &mut Response, ckey: &str, val: &str, path: Option<String>, max_age: Option<u64>, ) -> Result<()> {
    
    let mut ck = CookiePair::new(ckey.to_owned(), val.to_owned());
    ck.path = path;
    ck.max_age = max_age;
    
    println!("{:?}", ck);
    
    res.headers_mut().set(SetCookie(vec![ck]));
    
    Ok(())
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
