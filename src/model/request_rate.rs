#[derive(Debug, Clone)]
/// The model of limiting the frequency of requests to the server.
/// It's set by the `Request-Rate` directive.
/// # Example
/// For the directive `Request-Rate: 1/5` is equivalent to the model `RequestRate {requests: 1, seconds: 5}`
pub struct RequestRate {
    pub requests: usize,
    pub seconds: usize,
}
