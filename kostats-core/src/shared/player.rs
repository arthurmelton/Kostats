#[derive(Debug)]
pub struct Player {
    pub name: String,

    pub distance_glide: i64,
    pub distance_sprint: i64,
    pub distance_walk: i64,
    pub distance_ballform: i64,

    pub playtime: i64,
}
