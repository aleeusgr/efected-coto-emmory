use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

<<<<<<< HEAD
#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind("127.0.0.1:5000")?
        .run()
        .await
=======
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
>>>>>>> efected-cotoemory
}
