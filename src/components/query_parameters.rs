use std::collections::HashMap;

use crate::error::{convert::Unreachable, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct StatisticsParamsModel {
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

    use crate::error::convert::ConvertError;

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
            _ => Err(D::Error::custom(ConvertError::OptionLengthShouldBeLowerThanOne)),
        }
    }
}

impl StatisticsParamsModel {
    // TODO return Result<Vec<(String, String)>, Vec<(String, String)>>
    pub fn into_query(&self) -> Result<Vec<(String, String)>> {
        // TODO do not Self -> serde_json::Value -> Vec[(String, String)], but Self -> Vec[(String, String)]
        let value =
            serde_json::to_value(self).map_err(|_| anyhow::anyhow!(Unreachable::StructShouldBeConvertToValue))?;
        let map: HashMap<String, Vec<String>> =
            serde_json::from_value(value).map_err(|_| anyhow::anyhow!(Unreachable::ParamsShouldBeConvertToQuery))?;
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
        let value =
            serde_json::to_value(map).map_err(|_| anyhow::anyhow!(Unreachable::StructShouldBeConvertToValue))?;
        let params =
            serde_json::from_value(value).map_err(|_| anyhow::anyhow!(Unreachable::QueryShouldBeConvertToParams))?;
        Ok(params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_into_query() {
        let params = StatisticsParamsModel {
            sha: Some("master".to_string()),
            paths: vec!["/src".to_string(), "/test".to_string()],
            excluded: vec![],
        };
        let query = params.into_query().unwrap();
        assert_eq!(
            HashMap::<_, _>::from_iter(query),
            HashMap::from_iter(vec![
                ("sha".to_string(), "master".to_string()),
                ("paths".to_string(), "/src".to_string()),
                ("paths".to_string(), "/test".to_string())
            ])
        );
    }

    #[test]
    fn test_from_query() {
        let query = vec![
            ("sha".to_string(), "main".to_string()),
            ("paths".to_string(), "/src".to_string()),
            ("paths".to_string(), "/test".to_string()),
        ];
        let params = StatisticsParamsModel::from_query(&query).unwrap();
        assert_eq!(
            params,
            StatisticsParamsModel {
                sha: Some("main".to_string()),
                paths: vec!["/src".to_string(), "/test".to_string()],
                excluded: vec![]
            }
        );
    }
}
