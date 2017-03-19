use super::Route;
use super::Router;

/// Builder for a router
/// 
/// Example usage:
///
#[derive(Debug)]
pub struct RouterBuilder<'r> {
    routes: Vec<Route<'r>>,
}

impl<'r> RouterBuilder<'r> {
    pub fn new() -> RouterBuilder<'r> {
        RouterBuilder { routes: vec![] }
    }

    /// Adds new `Route` for `Router` that is being built.
    ///
    /// Example:
    ///
    /// ```ignore
    /// use hyper::server::{Request, Response};
    /// use hyper_router::{Route, RouterBuilder};
    ///
    /// fn some_handler(_: Request, _: Response) {
    ///   // do something
    /// }
    ///
    /// RouterBuilder::new().add(Route::get(r"/person/\d+").using(some_handler));
    /// ```
    pub fn add(mut self, route: Route<'r>) -> RouterBuilder<'r> {
        self.routes.push(route);
        self
    }

    pub fn build(self) -> Router<'r> {
        Router { routes: self.routes }
    }
}
