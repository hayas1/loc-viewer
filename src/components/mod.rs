use yew::prelude::*;

use crate::loc::get_statistics;

#[function_component(App)]
pub fn app() -> Html {
    let languages = get_statistics("dummy", &Default::default());
    let rust = &languages[&tokei::LanguageType::Rust];

    let contents = use_state(|| serde_json::Value::Null);
    {
        let contents = contents.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                contents.set(
                    crate::loc::repository::content("https://github.com/XAMPPRocky/octocrab").await,
                );
            })
        });
    }

    html! {
        <>
            <h1>{ "Hello World" }</h1>
            <p>{ format!("Lines of code: {}", rust.code) }</p>
            <p>{ format!("rust: {:?}", rust) }</p>
            <p>{ format!("contents: {:?}", contents) }</p>
        </>
    }
}
