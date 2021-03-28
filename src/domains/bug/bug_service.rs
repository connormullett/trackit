use super::bug_repository;
use super::BugDto;

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
