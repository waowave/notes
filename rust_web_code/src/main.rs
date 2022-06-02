use actix_web::{web, App, HttpResponse,HttpRequest, HttpServer,Responder};
use std::collections::HashMap;
use std::sync::Mutex;
use tera::Tera;
use tera::Context;



struct MyData {
   tera: tera::Tera,
   counter: u64,
}

// BTreeMaps

async fn my_handler(
    req: HttpRequest,
//    data: web::Data<Mutex<MyData>>,
) -> impl Responder {
   // return "HELLOWORLD";
   
   let body = reqwest::get("https://jsonplaceholder.typicode.com/todos/")
    .await
    .unwrap()
    .text()
    .await
    .unwrap();

    let data:&web::Data<Mutex<MyData>> = req.app_data().unwrap();

    let mut m_data = data.lock().unwrap();
    m_data.counter +=1;

    for (k,v) in req.match_info().iter()  {
        println!("pathinfo={} == {}", k,v  );
    }

    


    let mut context = Context::new();
//    context.insert("product", &product);

// &Context::from_serialize(&product)?

//    let json_data1: HashMap<String, serde_json::Value > = serde_json::from_str(&body).unwrap();
    let json_data2: serde_json::Value = serde_json::from_str(&body).unwrap();

//    context.insert("data1",  req.match_info() );
    context.insert("data2", &json_data2);


    let txt= m_data.tera.render("index.html", &context).unwrap();

/*
    println!("path={} count={} ({})",req.path(),m_data.counter,body );
*/
     return  HttpResponse::Ok().body(txt);


/*

let body = reqwest::get("https://www.rust-lang.org")
    .await?
    .text()
    .await?;


    let client = reqwest::Client::new();
let res = client.post("http://httpbin.org/post")
    .body("the exact body that is sent")
    .send()
    .await?;



*/

//    tokio::time::delay_for(Duration::from_secs(5)).await; // <-- Ok. Worker thread will handle other requests here
  //  "response\n"
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {


    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let data = web::Data::new(Mutex::new(MyData{ tera: tera,counter:0 }));


    HttpServer::new(move || App::new()
    .app_data(data.clone() )
    .service(
        web::resource("{a}/{b}")
        .route(web::get().to(my_handler)  ))
        )
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}



/*
jwt:

use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").unwrap();
let token_str = "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJzb21lb25lIn0.5wwE1sBrs-vftww_BGIuTVDeHtc1Jsjo-fiHhDwR8m0";

let claims: BTreeMap<String, String> = token_str.verify_with_key(&key).unwrap();

assert_eq!(claims["sub"], "someone");


//

use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, Token, VerifyWithKey};
use sha2::Sha384;
use std::collections::BTreeMap;

let key: Hmac<Sha384> = Hmac::new_from_slice(b"some-secret").unwrap();
let token_str = "eyJhbGciOiJIUzM4NCJ9.eyJzdWIiOiJzb21lb25lIn0.WM_WnPUkHK6zm6Wz7zk1kmIxz990Te7nlDjQ3vzcye29szZ-Sj47rLNSTJNzpQd_";

let token: Token<Header, BTreeMap<String, String>, _> = VerifyWithKey::verify_with_key(token_str, &key).unwrap();
let header = token.header();
let claims = token.claims();

assert_eq!(header.algorithm, AlgorithmType::Hs384);
assert_eq!(claims["sub"], "someone");

////
/// 
/// https://crates.io/crates/jwt


*/