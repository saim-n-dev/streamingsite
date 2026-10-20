use axum::{ extract::{ Query, State }, Json };
use crate::models::{ TmdbResponse, PageQuery };
use crate::state::AppState;

pub async fn root() -> &'static str {
    "Netflix Backend is Online"
}

pub async fn get_trending_movies(
    State(state): State<AppState>,
    Query(params): Query<PageQuery>
) -> Json<TmdbResponse> {
    let page = params.page.unwrap_or(1);
    let url = format!(
        "https://api.themoviedb.org/3/trending/all/week?api_key={}&page={}",
        state.tmdb_api_key,
        page
    );
    let res = state.client.get(&url).send().await.unwrap().json::<TmdbResponse>().await.unwrap();
    Json(res)
}