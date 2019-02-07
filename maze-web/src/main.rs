#![feature(proc_macro_hygiene, decl_macro)]

extern crate rand;
#[macro_use]
extern crate rocket;
extern crate svg;

extern crate labyru;

fn main() {
    rocket::ignite().mount("/", routes![]).launch();
}
