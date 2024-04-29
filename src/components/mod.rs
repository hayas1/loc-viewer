use url::Url;
use yew::prelude::*;

use crate::github::repository::GitHubRepository;

#[function_component(App)]
pub fn app() -> Html {
    let repository = GitHubRepository::from_url(&Url::parse("https://github.com/hayas1/loc-viewer").unwrap()).unwrap();
    // let repository = GitHubRepository::from_url(&Url::parse("https://github.com/rust-lang/rust").unwrap()).unwrap();

    let contents = use_state(|| None);
    {
        let contents = contents.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let languages = repository.get_statistics().await.unwrap();
                contents.set(Some(languages));
            })
        });
    }

    html! {
        <>
            <h1>{ "Hello World" }</h1>
            <p>{ match &(*contents) {
                Some(contents) => format!("contents: {:?}", contents),
                None => format!("loading..."),
             } }</p>
        </>
    }
}
