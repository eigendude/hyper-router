use hyper::server::{Request, Response};
use hyper::status::StatusCode;

use Handler;

pub struct default_404;

impl Handler for default_404 {
	fn handle(&self, _: Request, mut response: Response) {
	    {*response.status_mut() = StatusCode::NotFound}
	    response.send(b"page not found").ok();
	}
}

pub struct method_not_supported;

impl Handler for method_not_supported {
	fn handle(&self, _: Request, mut response: Response) {
	    {*response.status_mut() = StatusCode::MethodNotAllowed}
	    response.send(b"method not supported").ok();
	}
}

pub struct internal_server_error;

impl Handler for internal_server_error {
	fn handle(&self, _: Request, mut response: Response) {
	    {*response.status_mut() = StatusCode::InternalServerError}
	    response.send(b"internal server error").ok();
	}
}

pub struct not_implemented;

impl Handler for not_implemented {
	fn handle(&self, _: Request, mut response: Response) {
	    {*response.status_mut() = StatusCode::NotImplemented}
	    response.send(b"not implemented").ok();
	}
}

pub static default_404_handler: Box<Handler> = Box::new(default_404{});
pub static method_not_supported_handler: Box<Handler> = Box::new(method_not_supported{});
pub static internal_server_error_handler: Box<Handler> = Box::new(internal_server_error{});
pub static not_implemented_handler: Box<Handler> = Box::new(not_implemented{});
