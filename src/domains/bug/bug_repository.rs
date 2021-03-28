#![allow(proc_macro_derive_resolution_fallback)]

use diesel::RunQueryDsl;

use super::BugModel;
use crate::domains::estabilish_connection;

pub fn find_all_bugs() -> Vec<BugModel> {
    use crate::schema::bug::dsl::*;

    let connection = estabilish_connection();
    let bugs = bug
        .load::<BugModel>(&connection)
        .expect("error loading bugs");

    bugs
}
