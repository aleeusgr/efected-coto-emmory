
// this is a handler, a function that is used in a Router
// A handler is an async function that accepts zero or more “extractors” as arguments and returns something that can be converted into a response.
pub async fn say_hello() -> String {
    // integrate new functionality here:
   return "Hello!!!".to_string();
}
