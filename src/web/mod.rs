use qwreey_rocket::{export_list, RouteExportList};

mod commit_user;
mod index;

pub fn export_all() -> RouteExportList {
    export_list![index::Index, commit_user::UserCommit]
}
