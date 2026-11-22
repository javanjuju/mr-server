use std::collections::HashMap;

use crate::{http::requests::RequestMethod, HttpRequest, HttpResponse};

type Handler = Box<dyn Fn(&HttpRequest) -> HttpResponse + Sync + Send + 'static>;

pub struct Router {
    route: HashMap<(RequestMethod, String), Handler>,
}

impl Router {
    ///Creates a new router
    pub fn new() -> Self {
        Self {
            route: HashMap::new(),
        }
    }

    ///Register a handler for GET requests for path
    pub fn get(&mut self, path: impl Into<String>, handler: impl Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static) {
        self.route
            .insert((RequestMethod::Get, path.into()), Box::new(handler));
    }

    ///Register a handler for PUT requests for path
    pub fn put(&mut self, path: impl Into<String>, handler: impl Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static) {
        self.route
            .insert((RequestMethod::Put, path.into()), Box::new(handler));
    }

    ///Register a handler for POST requests for path
    pub fn post(&mut self, path: impl Into<String>, handler: impl Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static) {
        self.route
            .insert((RequestMethod::Post, path.into()), Box::new(handler));
    }

    ///Register a handler for PATCH requests for path
    pub fn patch(&mut self, path: impl Into<String>, handler: impl Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static) {
        self.route
            .insert((RequestMethod::Patch, path.into()), Box::new(handler));
    }

    ///Register a handler for QUERY requests for path
    pub fn query(&mut self, path: impl Into<String>, handler: impl Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static) {
        self.route
            .insert((RequestMethod::Query, path.into()), Box::new(handler));
    }

    ///Register a handler for CONNECT requests for path
    pub fn connect(&mut self, path: impl Into<String>, handler: impl Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static) {
        self.route
            .insert((RequestMethod::Connect, path.into()), Box::new(handler));
    }

    ///Register a handler for HEAD requests for path
    pub fn head(&mut self, path: impl Into<String>, handler: impl Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static) {
        self.route
            .insert((RequestMethod::Head, path.into()), Box::new(handler));
    }

    ///Register a handler for TRACE requests for path
    pub fn trace(&mut self, path: impl Into<String>, handler: impl Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static) {
        self.route
            .insert((RequestMethod::Trace, path.into()), Box::new(handler));
    }

    ///Register a handler for DELETE requests for path
    pub fn delete(&mut self, path: impl Into<String>, handler: impl Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static) {
        self.route
            .insert((RequestMethod::Delete, path.into()), Box::new(handler));
    }

    /// Register a handler for OPTIONS requests for path
    pub fn options(&mut self, path: impl Into<String>, handler: impl Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static) {
        self.route
            .insert((RequestMethod::Options, path.into()), Box::new(handler));
    }

    ///Handles the actual routing
    pub fn handle(&self, request: &HttpRequest) -> HttpResponse {
        let request_path = request.path();
        let request_method = request.method();
        for ((method, path), handler) in &self.route {
            //If the path and method match the request then call the handler to return a response
            if request_path.starts_with(path) && *request_method == *method {
               return  handler(request)
            }

            //if the path matches but the method does not then return a MethodNotAllowed response i.e method with code 405
            if request_path.starts_with(path) {
                return HttpResponse::method_not_allowed()
            }
        }

        HttpResponse::not_found()
    }
}
