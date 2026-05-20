use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use chrono::{NaiveDate, Utc};
use pulldown_cmark::{Options, Parser, html};
use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

const KNOWN_LANGUAGE_SLUGS: &[&str] = &[
    "bash",
    "c",
    "clojure",
    "cpp",
    "crystal",
    "csharp",
    "dart",
    "delphi",
    "elixir",
    "erlang",
    "fortran",
    "fsharp",
    "go",
    "haskell",
    "java",
    "javascript",
    "julia",
    "kotlin",
    "lua",
    "nim",
    "objective-c",
    "ocaml",
    "perl",
    "php",
    "python",
    "r",
    "ruby",
    "rust",
    "scala",
    "solidity",
    "sql",
    "swift",
    "typescript",
    "zig",
];

#[derive(Debug, Error)]
pub enum ContentError {
    #[error("could not read {path}: {source}")]
    Read {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("content validation failed:\n{0}")]
    Validation(String),
}

#[derive(Clone, Debug)]
pub struct SiteContent {
    pub languages: Vec<LanguagePage>,
    pub comparisons: Vec<ComparisonPage>,
    pub guides: Vec<GuidePage>,
    pub concepts: Vec<ConceptPage>,
}

impl SiteContent {
    pub fn load(root: &Path) -> Result<Self, ContentError> {
        let mut errors = Vec::new();
        let mut content = Self {
            languages: load_collection(root, "languages", &mut errors),
            comparisons: load_collection(root, "comparisons", &mut errors),
            guides: load_collection(root, "guides", &mut errors),
            concepts: load_collection(root, "concepts", &mut errors),
        };

        content
            .languages
            .sort_by_key(|page| page.data.title.clone());
        content
            .comparisons
            .sort_by_key(|page| page.data.title.clone());
        content.guides.sort_by_key(|page| page.data.title.clone());
        content.concepts.sort_by_key(|page| page.data.title.clone());

        content.validate(&mut errors);

        if errors.is_empty() {
            Ok(content)
        } else {
            Err(ContentError::Validation(errors.join("\n")))
        }
    }

    pub fn language(&self, slug: &str) -> Option<&LanguagePage> {
        self.languages.iter().find(|page| page.data.slug == slug)
    }

    pub fn comparison(&self, slug: &str) -> Option<&ComparisonPage> {
        self.comparisons.iter().find(|page| page.data.slug == slug)
    }

    pub fn guide(&self, slug: &str) -> Option<&GuidePage> {
        self.guides.iter().find(|page| page.data.slug == slug)
    }

    pub fn concept(&self, slug: &str) -> Option<&ConceptPage> {
        self.concepts.iter().find(|page| page.data.slug == slug)
    }

    pub fn all_routes(&self) -> BTreeSet<String> {
        let mut routes = BTreeSet::from([
            "/".to_string(),
            "/about".to_string(),
            "/about/".to_string(),
            "/contribute".to_string(),
            "/contribute/".to_string(),
            "/languages".to_string(),
            "/languages/".to_string(),
            "/comparisons".to_string(),
            "/comparisons/".to_string(),
            "/guides".to_string(),
            "/guides/".to_string(),
            "/concepts".to_string(),
            "/concepts/".to_string(),
            "/languages.json".to_string(),
            "/search.json".to_string(),
            "/rss.xml".to_string(),
            "/sitemap.xml".to_string(),
            "/robots.txt".to_string(),
        ]);

        for language in &self.languages {
            add_detail_routes(&mut routes, "languages", &language.data.slug);
        }
        for comparison in &self.comparisons {
            add_detail_routes(&mut routes, "comparisons", &comparison.data.slug);
        }
        for guide in &self.guides {
            add_detail_routes(&mut routes, "guides", &guide.data.slug);
        }
        for concept in &self.concepts {
            add_detail_routes(&mut routes, "concepts", &concept.data.slug);
        }

        routes
    }

    fn validate(&self, errors: &mut Vec<String>) {
        let known_languages: BTreeSet<&str> = KNOWN_LANGUAGE_SLUGS.iter().copied().collect();
        let published_languages: BTreeSet<&str> = self
            .languages
            .iter()
            .map(|page| page.data.slug.as_str())
            .collect();

        validate_duplicate_slugs(
            "languages",
            self.languages.iter().map(|page| page.data.slug.as_str()),
            errors,
        );
        validate_duplicate_slugs(
            "comparisons",
            self.comparisons.iter().map(|page| page.data.slug.as_str()),
            errors,
        );
        validate_duplicate_slugs(
            "guides",
            self.guides.iter().map(|page| page.data.slug.as_str()),
            errors,
        );
        validate_duplicate_slugs(
            "concepts",
            self.concepts.iter().map(|page| page.data.slug.as_str()),
            errors,
        );

        for page in &self.languages {
            validate_file_slug("languages", &page.file_stem, &page.data.slug, errors);
            validate_known_slug("languages", &page.data.slug, &known_languages, errors);
            validate_date(
                "languages",
                &page.data.slug,
                page.data.last_verified,
                errors,
            );
            validate_sources("languages", &page.data.slug, &page.data.sources, errors);
            validate_urls(
                "languages",
                &page.data.slug,
                [
                    ("officialSite", Some(page.data.official_site.as_str())),
                    ("repository", page.data.repository.as_deref()),
                ],
                errors,
            );
            for slug in &page.data.compared_with {
                validate_known_slug("languages.comparedWith", slug, &known_languages, errors);
            }
        }

        for page in &self.comparisons {
            validate_file_slug("comparisons", &page.file_stem, &page.data.slug, errors);
            validate_date(
                "comparisons",
                &page.data.slug,
                page.data.last_verified,
                errors,
            );
            validate_sources("comparisons", &page.data.slug, &page.data.sources, errors);
            for slug in &page.data.languages {
                validate_known_slug("comparisons.languages", slug, &known_languages, errors);
            }
            if page.data.languages.len() < 2 {
                errors.push(format!(
                    "comparisons/{} must reference at least two languages.",
                    page.data.slug
                ));
            }
        }

        for page in &self.guides {
            validate_file_slug("guides", &page.file_stem, &page.data.slug, errors);
            validate_date("guides", &page.data.slug, page.data.last_verified, errors);
            validate_sources("guides", &page.data.slug, &page.data.sources, errors);
            for slug in &page.data.candidates {
                validate_known_slug("guides.candidates", slug, &known_languages, errors);
            }
        }

        for page in &self.concepts {
            validate_file_slug("concepts", &page.file_stem, &page.data.slug, errors);
            validate_date("concepts", &page.data.slug, page.data.last_verified, errors);
            validate_sources("concepts", &page.data.slug, &page.data.sources, errors);
            for slug in &page.data.related_languages {
                validate_known_slug("concepts.relatedLanguages", slug, &known_languages, errors);
            }
        }

        self.validate_internal_links(errors);
        for slug in published_languages {
            if !self.all_routes().contains(&format!("/languages/{slug}")) {
                errors.push(format!("languages/{slug} has no route."));
            }
        }
    }

    fn validate_internal_links(&self, errors: &mut Vec<String>) {
        let routes = self.all_routes();
        let link_re = Regex::new(r#"\]\((/[^)\s#?]+)"#).expect("link regex");

        for page in self.all_pages() {
            for capture in link_re.captures_iter(page.body_markdown()) {
                let link = capture.get(1).expect("link capture").as_str();
                let normalized = link.trim_end_matches('/');
                if !routes.contains(link) && !routes.contains(normalized) {
                    errors.push(format!(
                        "{} contains a broken internal link to {link}.",
                        page.source_label()
                    ));
                }
            }
        }
    }

    fn all_pages(&self) -> Vec<PageRef<'_>> {
        let mut pages = Vec::new();
        pages.extend(self.languages.iter().map(PageRef::Language));
        pages.extend(self.comparisons.iter().map(PageRef::Comparison));
        pages.extend(self.guides.iter().map(PageRef::Guide));
        pages.extend(self.concepts.iter().map(PageRef::Concept));
        pages
    }
}

enum PageRef<'a> {
    Language(&'a LanguagePage),
    Comparison(&'a ComparisonPage),
    Guide(&'a GuidePage),
    Concept(&'a ConceptPage),
}

impl PageRef<'_> {
    fn source_label(&self) -> String {
        match self {
            Self::Language(page) => format!("languages/{}", page.data.slug),
            Self::Comparison(page) => format!("comparisons/{}", page.data.slug),
            Self::Guide(page) => format!("guides/{}", page.data.slug),
            Self::Concept(page) => format!("concepts/{}", page.data.slug),
        }
    }

    fn body_markdown(&self) -> &str {
        match self {
            Self::Language(page) => &page.body_markdown,
            Self::Comparison(page) => &page.body_markdown,
            Self::Guide(page) => &page.body_markdown,
            Self::Concept(page) => &page.body_markdown,
        }
    }
}

#[derive(Clone, Debug)]
pub struct LanguagePage {
    pub file_stem: String,
    pub data: LanguageFrontmatter,
    pub body_markdown: String,
    pub body_html: String,
}

#[derive(Clone, Debug)]
pub struct ComparisonPage {
    pub file_stem: String,
    pub data: ComparisonFrontmatter,
    pub body_markdown: String,
    pub body_html: String,
}

#[derive(Clone, Debug)]
pub struct GuidePage {
    pub file_stem: String,
    pub data: GuideFrontmatter,
    pub body_markdown: String,
    pub body_html: String,
}

#[derive(Clone, Debug)]
pub struct ConceptPage {
    pub file_stem: String,
    pub data: ConceptFrontmatter,
    pub body_markdown: String,
    pub body_html: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Source {
    pub title: String,
    pub url: String,
    #[serde(default)]
    pub publisher: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TypingInfo {
    pub discipline: String,
    #[serde(default)]
    pub strength: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ModelInfo {
    pub model: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LanguageFrontmatter {
    pub title: String,
    pub slug: String,
    pub status: String,
    pub summary: String,
    #[serde(default, rename = "firstReleased")]
    pub first_released: Option<u16>,
    #[serde(default)]
    pub creators: Vec<String>,
    #[serde(default)]
    pub paradigms: Vec<String>,
    pub typing: TypingInfo,
    pub memory: ModelInfo,
    pub runtime: ModelInfo,
    #[serde(rename = "officialSite")]
    pub official_site: String,
    #[serde(default)]
    pub repository: Option<String>,
    #[serde(default, rename = "packageManagers")]
    pub package_managers: Vec<String>,
    #[serde(default, rename = "comparedWith")]
    pub compared_with: Vec<String>,
    #[serde(default, rename = "bestFor")]
    pub best_for: Vec<String>,
    #[serde(default, rename = "poorFit")]
    pub poor_fit: Vec<String>,
    pub sources: Vec<Source>,
    #[serde(rename = "lastVerified")]
    pub last_verified: NaiveDate,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ComparisonFrontmatter {
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub languages: Vec<String>,
    #[serde(default, rename = "useCases")]
    pub use_cases: Vec<String>,
    pub sources: Vec<Source>,
    #[serde(rename = "lastVerified")]
    pub last_verified: NaiveDate,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuideFrontmatter {
    pub title: String,
    pub slug: String,
    pub summary: String,
    #[serde(default)]
    pub audience: Option<String>,
    #[serde(default)]
    pub candidates: Vec<String>,
    pub sources: Vec<Source>,
    #[serde(rename = "lastVerified")]
    pub last_verified: NaiveDate,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConceptFrontmatter {
    pub title: String,
    pub slug: String,
    pub summary: String,
    #[serde(default, rename = "relatedLanguages")]
    pub related_languages: Vec<String>,
    pub sources: Vec<Source>,
    #[serde(rename = "lastVerified")]
    pub last_verified: NaiveDate,
}

trait FromContentFile: Sized {
    fn from_parts(
        file_stem: String,
        frontmatter: &str,
        body_markdown: String,
    ) -> Result<Self, String>;
}

impl FromContentFile for LanguagePage {
    fn from_parts(
        file_stem: String,
        frontmatter: &str,
        body_markdown: String,
    ) -> Result<Self, String> {
        Ok(Self {
            file_stem,
            data: serde_yaml::from_str(frontmatter).map_err(|err| err.to_string())?,
            body_html: markdown_to_html(&body_markdown),
            body_markdown,
        })
    }
}

impl FromContentFile for ComparisonPage {
    fn from_parts(
        file_stem: String,
        frontmatter: &str,
        body_markdown: String,
    ) -> Result<Self, String> {
        Ok(Self {
            file_stem,
            data: serde_yaml::from_str(frontmatter).map_err(|err| err.to_string())?,
            body_html: markdown_to_html(&body_markdown),
            body_markdown,
        })
    }
}

impl FromContentFile for GuidePage {
    fn from_parts(
        file_stem: String,
        frontmatter: &str,
        body_markdown: String,
    ) -> Result<Self, String> {
        Ok(Self {
            file_stem,
            data: serde_yaml::from_str(frontmatter).map_err(|err| err.to_string())?,
            body_html: markdown_to_html(&body_markdown),
            body_markdown,
        })
    }
}

impl FromContentFile for ConceptPage {
    fn from_parts(
        file_stem: String,
        frontmatter: &str,
        body_markdown: String,
    ) -> Result<Self, String> {
        Ok(Self {
            file_stem,
            data: serde_yaml::from_str(frontmatter).map_err(|err| err.to_string())?,
            body_html: markdown_to_html(&body_markdown),
            body_markdown,
        })
    }
}

fn load_collection<T: FromContentFile>(
    root: &Path,
    collection: &str,
    errors: &mut Vec<String>,
) -> Vec<T> {
    let path = root.join(collection);
    let entries = match fs::read_dir(&path) {
        Ok(entries) => entries,
        Err(source) => {
            errors.push(format!("could not read {}: {source}", path.display()));
            return Vec::new();
        }
    };

    let mut pages = Vec::new();
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(source) => {
                errors.push(format!(
                    "could not read entry in {}: {source}",
                    path.display()
                ));
                continue;
            }
        };
        let file_path = entry.path();
        if !matches!(
            file_path.extension().and_then(|ext| ext.to_str()),
            Some("md" | "mdx")
        ) {
            continue;
        }

        let text = match fs::read_to_string(&file_path) {
            Ok(text) => text,
            Err(source) => {
                errors.push(format!("could not read {}: {source}", file_path.display()));
                continue;
            }
        };
        let Some(file_stem) = file_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .map(str::to_owned)
        else {
            errors.push(format!("{} has no UTF-8 file stem.", file_path.display()));
            continue;
        };
        let (frontmatter, body_markdown) = match split_frontmatter(&text) {
            Ok(parts) => parts,
            Err(err) => {
                errors.push(format!("{} {err}", file_path.display()));
                continue;
            }
        };

        match T::from_parts(file_stem, frontmatter, body_markdown.to_owned()) {
            Ok(page) => pages.push(page),
            Err(err) => errors.push(format!(
                "{} has invalid frontmatter: {err}",
                file_path.display()
            )),
        }
    }

    pages
}

fn split_frontmatter(text: &str) -> Result<(&str, &str), &'static str> {
    let body = text
        .strip_prefix("---\n")
        .or_else(|| text.strip_prefix("---\r\n"))
        .ok_or("is missing YAML frontmatter.")?;
    let delimiter = body
        .find("\n---")
        .ok_or("is missing a closing frontmatter delimiter.")?;
    let frontmatter = &body[..delimiter];
    let rest = &body[delimiter + "\n---".len()..];
    let rest = rest
        .strip_prefix("\r\n")
        .or_else(|| rest.strip_prefix('\n'))
        .unwrap_or(rest);
    Ok((frontmatter, rest))
}

fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    let parser = Parser::new_ext(markdown, options);
    let mut output = String::new();
    html::push_html(&mut output, parser);
    let output = output.replace("<pre>", r#"<pre tabindex="0">"#);
    ammonia::Builder::default()
        .add_tags(["table", "thead", "tbody", "tr", "th", "td"])
        .add_generic_attributes(["class", "tabindex"])
        .clean(&output)
        .to_string()
}

fn add_detail_routes(routes: &mut BTreeSet<String>, collection: &str, slug: &str) {
    routes.insert(format!("/{collection}/{slug}"));
    routes.insert(format!("/{collection}/{slug}/"));
}

fn validate_duplicate_slugs<'a>(
    collection: &str,
    slugs: impl Iterator<Item = &'a str>,
    errors: &mut Vec<String>,
) {
    let mut counts = BTreeMap::new();
    for slug in slugs {
        *counts.entry(slug).or_insert(0usize) += 1;
    }
    for (slug, count) in counts {
        if count > 1 {
            errors.push(format!("{collection}/{slug} is duplicated."));
        }
    }
}

fn validate_file_slug(collection: &str, file_stem: &str, slug: &str, errors: &mut Vec<String>) {
    if file_stem != slug {
        errors.push(format!(
            "{collection}/{file_stem}.mdx has slug {slug}, expected {file_stem}."
        ));
    }
}

fn validate_known_slug(
    field: &str,
    slug: &str,
    known_languages: &BTreeSet<&str>,
    errors: &mut Vec<String>,
) {
    if !known_languages.contains(slug) {
        errors.push(format!("{field} references unknown language slug {slug}."));
    }
}

fn validate_date(collection: &str, slug: &str, date: NaiveDate, errors: &mut Vec<String>) {
    if date > Utc::now().date_naive() {
        errors.push(format!(
            "{collection}/{slug} has future lastVerified {date}."
        ));
    }
}

fn validate_sources(collection: &str, slug: &str, sources: &[Source], errors: &mut Vec<String>) {
    if sources.is_empty() {
        errors.push(format!("{collection}/{slug} needs at least one source."));
    }
    for (index, source) in sources.iter().enumerate() {
        if source.title.trim().is_empty() {
            errors.push(format!(
                "{collection}/{slug} sources[{index}].title is empty."
            ));
        }
        validate_web_url(
            collection,
            slug,
            &format!("sources[{index}].url"),
            &source.url,
            errors,
        );
    }
}

fn validate_urls<'a>(
    collection: &str,
    slug: &str,
    urls: impl IntoIterator<Item = (&'a str, Option<&'a str>)>,
    errors: &mut Vec<String>,
) {
    for (field, value) in urls {
        if let Some(value) = value {
            validate_web_url(collection, slug, field, value, errors);
        }
    }
}

fn validate_web_url(
    collection: &str,
    slug: &str,
    field: &str,
    value: &str,
    errors: &mut Vec<String>,
) {
    match Url::parse(value) {
        Ok(url) if matches!(url.scheme(), "http" | "https") => {}
        Ok(url) => errors.push(format!(
            "{collection}/{slug} has non-web {field}: {}.",
            url.as_str()
        )),
        Err(err) => errors.push(format!(
            "{collection}/{slug} has invalid {field} {value}: {err}."
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::default_content_root;

    #[test]
    fn all_current_content_loads_and_validates() {
        let content = SiteContent::load(&default_content_root()).expect("content validates");
        assert_eq!(content.languages.len(), 22);
        assert!(content.language("fortran").is_some());
        assert!(content.language("objective-c").is_some());
        assert!(content.language("perl").is_some());
        assert!(content.language("scala").is_some());
        assert!(content.language("lua").is_some());
        assert!(content.language("dart").is_some());
        assert!(content.language("rust").is_some());
        assert!(content.comparison("perl-vs-python").is_some());
        assert!(content.comparison("fortran-vs-cpp").is_some());
        assert!(
            content
                .comparison("fortran-vs-python-for-numerics")
                .is_some()
        );
        assert!(content.comparison("swift-vs-objective-c").is_some());
        assert!(content.comparison("perl-vs-shell").is_some());
        assert!(content.comparison("scala-vs-java").is_some());
        assert!(content.comparison("scala-vs-kotlin").is_some());
        assert!(content.comparison("lua-vs-javascript").is_some());
        assert!(content.comparison("lua-vs-python").is_some());
        assert!(content.comparison("dart-vs-typescript").is_some());
        assert!(content.comparison("rust-vs-go").is_some());
        assert!(
            content
                .guide("choosing-an-embedded-scripting-language")
                .is_some()
        );
        assert!(
            content
                .guide("choosing-a-legacy-maintenance-language")
                .is_some()
        );
        assert!(content.guide("choosing-a-systems-language").is_some());
        assert!(content.concept("ownership").is_some());
    }

    #[test]
    fn route_manifest_covers_public_content() {
        let content = SiteContent::load(&default_content_root()).expect("content validates");
        let routes = content.all_routes();
        assert!(routes.contains("/languages/lua"));
        assert!(routes.contains("/languages/dart"));
        assert!(routes.contains("/languages/rust"));
        assert!(routes.contains("/comparisons/lua-vs-javascript"));
        assert!(routes.contains("/comparisons/lua-vs-python"));
        assert!(routes.contains("/comparisons/dart-vs-typescript"));
        assert!(routes.contains("/comparisons/rust-vs-go/"));
        assert!(routes.contains("/guides/choosing-an-embedded-scripting-language"));
        assert!(routes.contains("/guides/choosing-a-systems-language"));
        assert!(routes.contains("/concepts/ownership"));
        assert!(routes.contains("/languages.json"));
        assert!(routes.contains("/search.json"));
    }
}
