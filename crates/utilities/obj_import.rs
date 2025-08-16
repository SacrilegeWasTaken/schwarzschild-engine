use tobj::{self, LoadResult};

fn load_obj(path: &str) -> LoadResult {
    tobj::load_obj(
        path,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
    )
}
