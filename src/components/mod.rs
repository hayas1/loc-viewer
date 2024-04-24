use url::Url;
use yew::prelude::*;

use crate::{
    github::{client::GitHubApiClient, repository::GitHubRepository},
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
                contents.set(Some(GitHubApiClient::new(repository).get_repository().await.unwrap()));
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
