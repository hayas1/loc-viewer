use futures::{pin_mut, StreamExt};
use url::Url;
use yew::prelude::*;

use crate::{github::repository::GitHubRepository, loc::get_statistics};

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
                let stream = repository.walk().await;
                pin_mut!(stream); // needed for iteration

                let mut buff = Vec::new();
                while let Some(value) = stream.next().await {
                    match value {
                        Ok(value) => buff.push(value),
                        Err(_) => break,
                    }
                    contents.set(Some(buff.clone()))
                }
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
