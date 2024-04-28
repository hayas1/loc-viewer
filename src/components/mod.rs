use std::path::PathBuf;

use url::Url;
use yew::prelude::*;

use crate::{
    github::{blob::GitHubBlob, client::GitHubApiClient, repository::GitHubRepository},
    loc::get_statistics,
};

#[function_component(App)]
pub fn app() -> Html {
    let languages = get_statistics("dummy", &Default::default());
    let rust = &languages[&tokei::LanguageType::Rust];

    let repository = GitHubRepository::from_url(&Url::parse("https://github.com/hayas1/loc-viewer").unwrap()).unwrap();

    let contents = use_state(|| None);
    {
        let contents = contents.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                // contents.set(Some(GitHubApiClient::new(repository).trees("master").await.unwrap()));
                let blobs =
                    GitHubApiClient::new(repository).blobs("79710170d2cca3eafb449f9cb6432f0b3b3e67ed").await.unwrap();
                let content = GitHubBlob::from_model(PathBuf::from("Cargo.toml"), blobs).unwrap().content;
                contents.set(Some(content));
            })
        });
    }

    html! {
        <>
            <h1>{ "Hello World" }</h1>
            <p>{ format!("Lines of code: {}", rust.code) }</p>
            <p>{ format!("rust: {:?}", rust) }</p>
            <p>{ match (*contents).clone() {
                Some(contents) => format!("contents: {:?}", contents),
                None => format!("loading..."),
             } }</p>
        </>
    }
}
