#![allow(proc_macro_derive_resolution_fallback)]

use diesel::prelude::*;
use diesel::{QueryResult, RunQueryDsl};

use super::{BugModel, NewBug};
use crate::schema::bug;

pub fn get_all_bugs(connection: &PgConnection) -> QueryResult<Vec<BugModel>> {
    bug::table.load::<BugModel>(&*connection)
}

pub fn get_bug_by_id(id: i32, connection: &PgConnection) -> QueryResult<BugModel> {
    bug::table.find(id).get_result(connection)
}

pub fn create_bug(new_bug: NewBug, connection: &PgConnection) -> QueryResult<BugModel> {
    diesel::insert_into(bug::table)
        .values(&new_bug)
        .get_result(connection)
}
