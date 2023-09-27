# Testing

Every application should be well tested and understandable. Rocket provides the
tools to perform unit and integration tests. It also provides a means to inspect
code generated by Rocket.

## Local Dispatching

Rocket applications are tested by dispatching requests to a local instance of
`Rocket`. The [`local`] module contains all of the structures necessary to do
so. In particular, it contains a [`Client`] structure that is used to create
[`LocalRequest`] structures that can be dispatched against a given [`Rocket`]
instance. Usage is straightforward:

  1. Construct a `Rocket` instance that represents the application.

     ```rust
     let rocket = rocket::ignite();
     # let _ = rocket;
     ```

  2. Construct a `Client` using the `Rocket` instance.

     ```rust
     # use rocket::local::Client;
     # let rocket = rocket::ignite();
     let client = Client::new(rocket).expect("valid rocket instance");
     # let _ = client;
     ```

  3. Construct requests using the `Client` instance.

     ```rust
     # use rocket::local::Client;
     # let rocket = rocket::ignite();
     # let client = Client::new(rocket).unwrap();
     let req = client.get("/");
     # let _ = req;
     ```

  4. Dispatch the request to retrieve the response.

     ```rust
     # use rocket::local::Client;
     # let rocket = rocket::ignite();
     # let client = Client::new(rocket).unwrap();
     # let req = client.get("/");
     let response = req.dispatch();
     # let _ = response;
     ```

[`local`]: @api/rocket/local/
[`Client`]: @api/rocket/local/struct.Client.html
[`LocalRequest`]: @api/rocket/local/struct.LocalRequest.html
[`Rocket`]: @api/rocket/struct.Rocket.html

## Validating Responses

A `dispatch` of a `LocalRequest` returns a [`LocalResponse`] which can be used
transparently as a [`Response`] value. During testing, the response is usually
validated against expected properties. These includes things like the response
HTTP status, the inclusion of headers, and expected body data.

The [`Response`] type provides methods to ease this sort of validation. We list
a few below:

  * [`status`]: returns the HTTP status in the response.
  * [`content_type`]: returns the Content-Type header in the response.
  * [`headers`]: returns a map of all of the headers in the response.
  * [`body_string`]: returns the body data as a `String`.
  * [`body_bytes`]: returns the body data as a `Vec<u8>`.

[`LocalResponse`]: @api/rocket/local/struct.LocalResponse.html
[`Response`]: @api/rocket/struct.Response.html
[`status`]: @api/rocket/struct.Response.html#method.status
[`content_type`]: @api/rocket/struct.Response.html#method.content_type
[`headers`]: @api/rocket/struct.Response.html#method.headers
[`body_string`]: @api/rocket/struct.Response.html#method.body_string
[`body_bytes`]: @api/rocket/struct.Response.html#method.body_bytes

These methods are typically used in combination with the `assert_eq!` or
`assert!` macros as follows:

```rust
# #![feature(proc_macro_hygiene, decl_macro)]
# #[macro_use] extern crate rocket;

# use std::io::Cursor;
# use rocket::Response;
# use rocket::http::Header;

# #[get("/")]
# fn hello() -> Response<'static> {
#     Response::build()
#         .header(ContentType::Plain)
#         .header(Header::new("X-Special", ""))
#         .sized_body(Cursor::new("Expected Body"))
#         .finalize()
# }

use rocket::local::Client;
use rocket::http::{ContentType, Status};

let rocket = rocket::ignite().mount("/", routes![hello]);
let client = Client::new(rocket).expect("valid rocket instance");
let mut response = client.get("/").dispatch();

assert_eq!(response.status(), Status::Ok);
assert_eq!(response.content_type(), Some(ContentType::Plain));
assert!(response.headers().get_one("X-Special").is_some());
assert_eq!(response.body_string(), Some("Expected Body".into()));
```

## Testing "Hello, world!"

To solidify an intuition for how Rocket applications are tested, we walk through
how to test the "Hello, world!" application below:

```rust
# #![feature(proc_macro_hygiene, decl_macro)]
# #[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![hello])
}

fn main() {
    # if false {
    rocket().launch();
    # }
}
```

Notice that we've separated the _creation_ of the `Rocket` instance from the
_launch_ of the instance. As you'll soon see, this makes testing our application
easier, less verbose, and less error-prone.

### Setting Up

First, we'll create a `test` module with the proper imports:

```rust
#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn hello_world() {
        /* .. */
    }
}
```

You can also move the body of the `test` module into its own file, say
`tests.rs`, and then import the module into the main file using:

```rust
#[cfg(test)] mod tests;
```

### Testing

To test our "Hello, world!" application, we first create a `Client` for our
`Rocket` instance. It's okay to use methods like `expect` and `unwrap` during
testing: we _want_ our tests to panic when something goes wrong.

```rust
# fn rocket() -> rocket::Rocket { rocket::ignite() }
# use rocket::local::Client;

let client = Client::new(rocket()).expect("valid rocket instance");
```

Then, we create a new `GET /` request and dispatch it, getting back our
application's response:

```rust
# fn rocket() -> rocket::Rocket { rocket::ignite() }
# use rocket::local::Client;
# let client = Client::new(rocket()).expect("valid rocket instance");
let mut response = client.get("/").dispatch();
```

Finally, we ensure that the response contains the information we expect it to.
Here, we want to ensure two things:

  1. The status is `200 OK`.
  2. The body is the string "Hello, world!".

We do this by checking the `Response` object directly:

```rust
# #![feature(proc_macro_hygiene, decl_macro)]
# #[macro_use] extern crate rocket;

# #[get("/")]
# fn hello() -> &'static str { "Hello, world!" }

# use rocket::local::Client;
use rocket::http::{ContentType, Status};
#
# let rocket = rocket::ignite().mount("/", routes![hello]);
# let client = Client::new(rocket).expect("valid rocket instance");
# let mut response = client.get("/").dispatch();

assert_eq!(response.status(), Status::Ok);
assert_eq!(response.body_string(), Some("Hello, world!".into()));
```

That's it! Altogether, this looks like:

```rust
# #![feature(proc_macro_hygiene, decl_macro)]
# #[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![hello])
}

# /*
#[cfg(test)]
# */
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    # /*
    #[test]
    # */ pub
    fn hello_world() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }
}

# fn main() { test::hello_world(); }
```

The tests can be run with `cargo test`. You can find the full source code to
[this example on GitHub](@example/testing).

## Codegen Debug

It can be useful to inspect the code that Rocket's code generation is emitting,
especially when you get a strange type error. To have Rocket log the code that
it is emitting to the console, set the `ROCKET_CODEGEN_DEBUG` environment
variable when compiling:

```sh
ROCKET_CODEGEN_DEBUG=1 cargo build
```

During compilation, you should see output like:

```rust,ignore
note: emitting Rocket code generation debug output
 --> examples/hello_world/src/main.rs:7:1
  |
7 | #[get("/")]
  | ^^^^^^^^^^^
  |
  = note:
    fn rocket_route_fn_hello<'_b>(
        __req: &'_b ::rocket::Request,
        __data: ::rocket::Data
    ) -> ::rocket::handler::Outcome<'_b> {
        let responder = hello();
        ::rocket::handler::Outcome::from(__req, responder)
    }
```

This corresponds to the facade request handler Rocket has generated for the
`hello` route.