use std::collections::HashMap;

use libloading::{Library, Symbol};



pub struct Wrapper<'a, T> {
    wintab:libloading::Library,
    functions: HashMap<String, Symbol<'a, T>>
}