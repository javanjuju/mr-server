use std::collections::HashMap;

use crate::{http::requests::RequestMethod, HttpRequest, HttpResponse};

type Handler = Box<dyn Fn(&HttpRequest) -> HttpResponse + Sync + Send + 'static>;

pub struct Router {
    routes: HashMap<String, HashMap<RequestMethod, Handler>>,
}

impl Router {
    ///Creates a new router
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    fn add(
        &mut self,
        method: RequestMethod,
        path: impl Into<String>,
        handler: impl Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static,
    ) {
        self.routes
            .entry(path.into())
            .or_default()
            .insert(method, Box::new(handler));
    }

    ///Register a handler for GET requests for path
    pub fn get(
        &mut self,
        path: impl Into<String>,
        handler: impl Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static,
    ) {
        self.add(RequestMethod::Get, path.into(), handler);
    }

    ///Register a handler for PUT requests for path
    pub fn put(
        &mut self,
        path: impl Into<String>,
        handler: impl Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static,
    ) {
        self.add(RequestMethod::Put, path.into(), handler);
    }

    ///Register a handler for POST requests for path
    pub fn post(
        &mut self,
        path: impl Into<String>,
        handler: impl Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static,
    ) {
        self.add(RequestMethod::Post, path.into(), handler);
    }

    ///Register a handler for PATCH requests for path
    pub fn patch(
        &mut self,
        path: impl Into<String>,
        handler: impl Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static,
    ) {
        self.add(RequestMethod::Patch, path.into(), handler);
    }

    ///Register a handler for QUERY requests for path
    pub fn query(
        &mut self,
        path: impl Into<String>,
        handler: impl Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static,
    ) {
        self.add(RequestMethod::Query, path.into(), handler);
    }

    ///Register a handler for CONNECT requests for path
    pub fn connect(
        &mut self,
        path: impl Into<String>,
        handler: impl Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static,
    ) {
        self.add(RequestMethod::Connect, path.into(), handler);
    }

    ///Register a handler for HEAD requests for path
    pub fn head(
        &mut self,
        path: impl Into<String>,
        handler: impl Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static,
    ) {
        self.add(RequestMethod::Head, path.into(), handler);
    }

    ///Register a handler for TRACE requests for path
    pub fn trace(
        &mut self,
        path: impl Into<String>,
        handler: impl Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static,
    ) {
        self.add(RequestMethod::Trace, path.into(), handler);
    }

    ///Register a handler for DELETE requests for path
    pub fn delete(
        &mut self,
        path: impl Into<String>,
        handler: impl Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static,
    ) {
        self.add(RequestMethod::Delete, path.into(), handler);
    }

    /// Register a handler for OPTIONS requests for path
    pub fn options(
        &mut self,
        path: impl Into<String>,
        handler: impl Fn(&HttpRequest) -> HttpResponse + Send + Sync + 'static,
    ) {
        self.add(RequestMethod::Options, path.into(), handler);
    }

    ///Handles the actual routing
    pub fn handle(&self, request: &HttpRequest) -> HttpResponse {
        let path = request.path().split('?').next().unwrap_or("");
        match self.routes.get(path) {
            None => HttpResponse::not_found(),
            Some(methods) => match methods.get(request.method()) {
                Some(handler) => handler(request),
                None => {
                    let mut allowed_methods: Vec<&str> =
                        methods.keys().map(|method| method.as_str()).collect();
                    allowed_methods.sort();
                    HttpResponse::method_not_allowed(allowed_methods)
                }
            },
        }
    }
}
