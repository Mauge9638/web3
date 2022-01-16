use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    number();
    let name= req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

fn number(){
    let mut number: i32= 0;
    while &number < &100000000 {
        if (number % 1000000)==0
        {
            println!("{}",&number);
        }
        number+=1;
    }
    println!("Done: {}",&number);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(greet))
        .route("/{name}", web::get().to(greet))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}