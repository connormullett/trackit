use super::BugDto;
use super::{bug_repository, NewBug};

pub fn find_all_bugs() -> Vec<BugDto> {
    bug_repository::find_all_bugs()
        .into_iter()
        .map(|bug| {
            return BugDto {
                id: bug.id,
                title: bug.title,
                body: bug.body,
                resolved: bug.resolved,
            };
        })
        .collect()
}

pub fn create_bug(bug: NewBug) -> BugDto {
    let e = bug_repository::create_bug(bug);

    BugDto {
        id: e.id,
        title: e.title,
        body: e.body,
        resolved: e.resolved,
    }
}
