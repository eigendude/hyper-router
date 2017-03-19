use Route;
use Handler;

pub struct RouteBuilder {
    route: Route
}

impl<'r> RouteBuilder<'r> {
    pub fn new(route: Route) -> RouteBuilder {
        RouteBuilder {
            route: route
        }
    }

    /// Completes the building process by taking the handler to process the request.
    ///
    /// Returns created route.
    pub fn using(mut self, handler: Box<Handler>) -> Route<'r> {
        self.route.handler = handler;
        self.route
    }
}

