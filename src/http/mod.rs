mod hoc;
mod routes;

pub(crate) use self::{
    hoc::{calculate_hoc, delete_repo_and_cache, json_hoc, overview},
    routes::{favicon32, generate, health_check, index, p404, static_file},
};
