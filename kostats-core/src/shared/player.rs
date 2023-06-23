use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Player {
    pub name: String,

    pub distance_glide: Option<i64>,
    pub distance_sprint: Option<i64>,
    pub distance_walk: Option<i64>,
    pub distance_ballform: Option<i64>,

    pub hit_given: Option<i64>,
    pub hit_received: Option<i64>,

    pub ko_given: Option<i64>,
    pub ko_given_doubles: Option<i64>,
    pub ko_given_frenzes: Option<i64>,
    pub ko_given_triples: Option<i64>,
    pub ko_received: Option<i64>,

    pub successful_tackles: Option<i64>,

    pub playtime: Option<i64>,

    pub mvp: Option<i64>,
    pub match_wins: Option<i64>,
    pub rounds_win: Option<i64>,
    pub rounds_lost: Option<i64>,
}
