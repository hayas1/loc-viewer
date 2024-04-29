use std::collections::{btree_map::Entry, BTreeMap};

use anyhow::Result;
use futures::{pin_mut, StreamExt};
use tokei::{Config, Language, LanguageType, Languages, Report};

use super::repository::GitHubRepository;

#[derive(Debug)]
pub struct Statistics {
    pub repository: GitHubRepository,
    pub languages: Languages,
}
impl Statistics {
    pub async fn get(repository: GitHubRepository) -> Result<Self> {
        let languages = Self::get_statistics(&repository, &Config::default()).await?;
        Ok(Self { repository, languages })
    }

    pub async fn get_statistics(repository: &GitHubRepository, config: &Config) -> Result<Languages> {
        let mut languages = Self::walk(repository, config).await?;
        languages.iter_mut().for_each(|(_, language)| language.total());
        Ok(languages)
    }

    /// `from_previous` is private method
    /// https://github.com/XAMPPRocky/tokei/blob/v12.1.2/src/language/languages.rs#L46-L61
    pub fn as_languages(map: BTreeMap<LanguageType, Language>) -> Languages {
        let mut languages = Languages::new();
        for (name, input_language) in map {
            match languages.entry(name) {
                Entry::Occupied(mut entry) => {
                    *entry.get_mut() += input_language;
                }
                Entry::Vacant(entry) => {
                    entry.insert(input_language);
                }
            }
        }
        languages
    }

    pub async fn walk(repository: &GitHubRepository, config: &Config) -> Result<Languages> {
        let mut languages: BTreeMap<LanguageType, Language> = BTreeMap::new();

        let stream = repository.walk().await;
        pin_mut!(stream); // needed for iteration
        while let Some(value) = stream.next().await {
            let blob = value?;
            let Some(language_type) = LanguageType::from_path(&blob.path, &config) else {
                continue;
            };
            let language = languages.entry(language_type).or_insert_with(Language::new);
            let mut report = Report::new(blob.path);
            report += language_type.parse_from_str(&blob.content, &config);
            language.add_report(report);
        }

        Ok(Self::as_languages(languages))
    }
}
