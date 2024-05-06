use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_autoprops::autoprops;
use yew_router::hooks::use_location;

use super::routes::RouterUnavailable;
use crate::{error::Result, github::repository::GitHubRepository};

#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ParamsModel {
    #[serde(skip_serializing_if = "Option::is_none", with = "option_as_vec")]
    pub sha: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub excluded: Vec<String>,
}

// TODO can we use serde_with crate?
pub mod option_as_vec {
    use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
        S: Serializer,
    {
        value.into_iter().collect::<Vec<_>>().serialize(serializer)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        let mut value = <Vec<T>>::deserialize(deserializer)?;
        match value.len() {
            0 => Ok(None),
            1 => Ok(Some(value.remove(0))),
            _ => Err(D::Error::custom("//TODO too many values")),
        }
    }
}

impl ParamsModel {
    // TODO return Result<Vec<(String, String)>, Vec<(String, String)>>
    pub fn into_query(&self) -> Result<Vec<(String, String)>> {
        // TODO do not Self -> serde_json::Value -> Vec[(String, String)], but Self -> Vec[(String, String)]
        let value = serde_json::to_value(self).map_err(anyhow::Error::from)?; // TODO unreachable
        let map: HashMap<String, Vec<String>> = serde_json::from_value(value).map_err(anyhow::Error::from)?; // TODO unreachable
        let query = map.into_iter().flat_map(|(key, vs)| vs.into_iter().map(move |s| (key.clone(), s))).collect();
        Ok(query)
    }

    // TODO return Result<Self, Self>
    pub fn from_query(query: &[(String, String)]) -> Result<Self> {
        // TODO do not Vec[(String, String)] -> serde_json::Value -> Self, but Vec[(String, String)] -> Self
        let mut map = HashMap::new();
        for (key, value) in query {
            map.entry(key).or_insert(Vec::new()).push(value);
        }
        let value = serde_json::to_value(map).map_err(anyhow::Error::from)?; // TODO unreachable
        let params = serde_json::from_value(value).map_err(anyhow::Error::from)?; // TODO unreachable
        Ok(params)
    }
}

#[autoprops]
#[function_component(Statistics)]
pub fn statistics(host: &String, owner: &String, repo: &String) -> HtmlResult {
    let Some(location) = use_location() else {
        return Ok(html! { <RouterUnavailable/> });
    };
    let query = ParamsModel::from_query(&location.query::<Vec<(String, String)>>().unwrap());
    let repository = Arc::new(GitHubRepository::new(owner, repo));
    Ok(html! {
        <div>
            <p>{ format!("query: {:?}", query) }</p>
            <Table repository={repository}/>
        </div>
    })
}

#[autoprops]
#[function_component(Table)]
pub fn table(repository: &Arc<GitHubRepository>) -> HtmlResult {
    let repository = repository.clone();
    let result = use_state(|| None);
    {
        let (result, repository) = (result.clone(), repository.clone());
        use_effect_with(repository.clone(), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let statistics = repository.get_statistics(&Default::default()).await;
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
