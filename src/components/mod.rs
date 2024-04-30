use std::sync::Arc;

use url::Url;
use yew::prelude::*;
use yew_autoprops::autoprops;

use crate::github::repository::GitHubRepository;

#[function_component(App)]
pub fn app() -> HtmlResult {
    let repository = GitHubRepository::from_url(&Url::parse("https://github.com/hayas1/loc-viewer").unwrap()).unwrap();
    // let repository = GitHubRepository::from_url(&Url::parse("https://github.com/rust-lang/rust").unwrap()).unwrap();
    // let repository = GitHubRepository::from_url(&Url::parse("https://github.com/XAMPPRocky/tokei").unwrap()).unwrap();

    Ok(html! {
        <Table repository={Arc::new(repository)} />
    })
}

#[autoprops]
#[function_component(Table)]
pub fn table(repository: &Arc<GitHubRepository>) -> HtmlResult {
    let repository = repository.clone();
    let result = use_state(|| None);
    {
        let result = result.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let statistics = repository.get_statistics().await;
                result.set(Some(statistics));
            })
        });
    }

    Ok(html! {
        match &(*result) {
            Some(Ok(statistics)) => html! {
                <table class="table-auto">
                    <thead>
                        <tr>
                            <th>{ "Language" }</th>
                            <th>{ "Files" }</th>
                            <th>{ "Lines" }</th>
                            <th>{ "Code" }</th>
                            <th>{ "Comments" }</th>
                            <th>{ "Blanks" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            for statistics.languages.iter().map(|(language_type, language)| {
                                html! {
                                    <tr>
                                        <td>{ language_type.to_string() }</td>
                                        <td>{ language.reports.len() }</td>
                                        <td>{ language.lines() }</td>
                                        <td>{ language.code }</td>
                                        <td>{ language.comments }</td>
                                        <td>{ language.blanks }</td>
                                        // <td>{ language.total() }</td>
                                   </tr>
                                }
                            })
                        }
                    </tbody>
                </table>
            },
            Some(Err(err)) => html! { format!("error occurred: {err:?}") },
            None => html! { format!("loading...") },
        }
    })
}
