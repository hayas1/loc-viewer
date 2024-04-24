pub async fn content(url: &str) -> serde_json::Value {
    // let repo = octocrab::instance()
    //     .repos("XAMPPRocky", "octocrab")
    //     .get_readme()
    //     .send()
    //     .await
    //     .unwrap();

    let readme: serde_json::Value =
        gloo::net::http::Request::get("https://api.github.com/repos/XAMPPRocky/octocrab/readme")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

    readme
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    async fn test_content() {
        let content = content("https://github.com/XAMPPRocky/octocrab").await;
        assert_eq!("content", content);
    }
}
