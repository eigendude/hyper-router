use std::rc::Rc;

use Route;
use Handler;

pub struct RouteBuilder {
    route: Route
}

impl RouteBuilder {
    pub fn new(route: Route) -> RouteBuilder {
        RouteBuilder {
            route: route
        }
    }

    /// Completes the building process by taking the handler to process the request.
    ///
    /// Returns created route.
    pub fn using(mut self, handler: Rc<Handler>) -> Route {
        self.route.handler = handler;
        self.route
    }
}

