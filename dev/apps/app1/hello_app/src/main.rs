use axum:: {Router, routing::get, response::Html};

#[tokio::main]
async fn main() {

    let app = Router::new().route("/", get(handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    // let message:&str = "cargo";
    // println!("Hello, {}", message);

    // let guess: u32 = "42".parse().expect("Not a number");
    // println!("{}", guess);

    // echo_greeting("Santacross");

    // let number: u64 = 5 + 4 + 3 + 2 + 1;
    // if number > 10 {
    //     println!("number is bigger than 10 : {}", number);
    // } else {
    //     println!("number is smaller than 10 : {}", number);
    // }
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, Rust and Axum!</h1>")
}

// fn echo_greeting(greeting: &str) {
//     println!("Hello,{}", greeting);
// }
