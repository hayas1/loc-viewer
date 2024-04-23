use std::{
    collections::{btree_map::Entry, BTreeMap},
    path::PathBuf,
};

use tokei::{Config, Language, LanguageType, Languages, Report};

pub fn get_statistics(repo: &str, config: &Config) -> Languages {
    let mut languages = walk(repo, config);
    languages
        .iter_mut()
        .for_each(|(_, language)| language.total());
    languages
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

pub fn walk(repo: &str, config: &Config) -> Languages {
    let mut languages: BTreeMap<LanguageType, Language> = BTreeMap::new();
    let repo_contents: BTreeMap<_, _> = [
        ("a.rs", r#"println!("Hello World")"#),
        ("b.rs", r#"println!("Hello World");"#),
        (
            "main.rs",
            r#"
                use loc_viwer::components::App;

                fn main() {
                    gloo::console::log!(format!(
                        "start {} (version {})",
                        env!("CARGO_PKG_NAME"),
                        env!("CARGO_PKG_VERSION")
                    ));

                    yew::Renderer::<App>::new().render();
                }
            "#,
        ),
    ]
    .into_iter()
    .collect();

    let files = repo_contents.iter().filter_map(|(path, contents)| {
        LanguageType::from_path(path, &config).map(|l| (l, path, contents))
    });

    for (lt, path, contents) in files {
        let language = languages.entry(lt).or_insert_with(Language::new);
        let root = PathBuf::from(repo);
        let mut report = Report::new(root.join(path));
        report += lt.parse_from_str(contents, &config);
        language.add_report(report)
    }

    as_languages(languages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_statistics() {
        let languages = get_statistics("dummy", &Default::default());
        let rust = &languages[&LanguageType::Rust];
        println!("Lines of code: {}", rust.code);
        println!("rust: {:?}", rust);
    }
}
