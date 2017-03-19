use hyper::server::{Request, Response};
use hyper::status::StatusCode;

use Handler;

pub struct default_404_handler;

impl Handler for default_404_handler {
	fn handle(&self, _: Request, mut response: Response) {
	    {*response.status_mut() = StatusCode::NotFound}
	    response.send(b"page not found").ok();
	}
}

pub struct method_not_supported_handler;

impl Handler for method_not_supported_handler {
	fn handle(&self, _: Request, mut response: Response) {
	    {*response.status_mut() = StatusCode::MethodNotAllowed}
	    response.send(b"method not supported").ok();
	}
}

pub struct internal_server_error_handler;

impl Handler for internal_server_error_handler {
	fn handle(&self, _: Request, mut response: Response) {
	    {*response.status_mut() = StatusCode::InternalServerError}
	    response.send(b"internal server error").ok();
	}
}

pub struct not_implemented_handler;

impl Handler for not_implemented_handler {
	fn handle(&self, _: Request, mut response: Response) {
	    {*response.status_mut() = StatusCode::NotImplemented}
	    response.send(b"not implemented").ok();
	}
}
