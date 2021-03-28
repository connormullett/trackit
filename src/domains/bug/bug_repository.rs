#![allow(proc_macro_derive_resolution_fallback)]

use diesel::RunQueryDsl;

use super::{BugModel, NewBug};
use crate::domains::estabilish_connection;

pub fn get_all_bugs() -> Vec<BugModel> {
    use crate::schema::bug::dsl::*;

    let connection = estabilish_connection();
    let bugs = bug
        .load::<BugModel>(&connection)
        .expect("error loading bugs");

    bugs
}

pub fn create_bug(bug: NewBug) -> BugModel {
    use crate::schema::bug;

    let connection = estabilish_connection();

    diesel::insert_into(bug::table)
        .values(&bug)
        .get_result(&connection)
        .expect("error saving new bug")
}
