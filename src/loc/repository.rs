pub async fn content() -> octocrab::models::Repository {
    let repository: octocrab::models::Repository =
        gloo::net::http::Request::get("https://api.github.com/repos/XAMPPRocky/octocrab")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

    repository
}
