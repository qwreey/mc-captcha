use qwreey_rocket::{export_list, RouteExportList};

mod index;
mod commit_user;

pub fn export_all() -> RouteExportList {
    export_list![
        commit_user::UserCommit,
        index::Index,
    ]
}
