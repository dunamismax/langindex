use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use chrono::NaiveDate;
use leptos::IntoView;
use leptos::prelude::*;
use leptos::tachys::view::RenderHtml;
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::content::{ComparisonPage, ConceptPage, GuidePage, LanguagePage, SiteContent, Source};

const SITE_URL: &str = "https://langindex.dev";

const FEATURED_LANGUAGE_SLUGS: &[&str] = &["rust", "python", "typescript", "go", "java", "c"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NavSection {
    Home,
    Languages,
    Comparisons,
    Guides,
    Concepts,
    About,
    Contribute,
    None,
}

struct PageMeta {
    title: String,
    description: String,
    section: NavSection,
}

pub async fn home(State(state): State<AppState>) -> Response {
    let content = &state.content;
    let mut body = String::new();
    body.push_str(
        r#"<section class="hero"><div class="container hero-grid"><div class="hero-copy">"#,
    );
    body.push_str(r#"<p class="eyebrow">Open source · Self-hosted · Source-backed</p>"#);
    body.push_str(
        r#"<h1>Understand programming languages by fit, tradeoff, and maintenance reality.</h1>"#,
    );
    body.push_str(r#"<p class="lede">LangIndex is a practical reference for developers choosing, learning, or maintaining languages. It keeps factual claims source-backed and turns comparisons into constraints instead of winner lists.</p>"#);
    body.push_str(r#"<p class="actions"><a class="button primary" href="/languages/">Browse languages</a><a class="button" href="/guides/">Choose by problem</a><a class="button quiet" href="/contribute">Improve a page</a></p>"#);
    body.push_str(r#"</div>"#);
    body.push_str(&home_search_panel());
    body.push_str(r#"</div></section>"#);
    body.push_str(&stats(content));

    body.push_str(r#"<section class="section section-tight"><div class="container section-head"><div><p class="eyebrow">Start with the question</p><h2>Browse by practical need</h2><p>Move from a production constraint to languages, guides, and comparisons that explain the tradeoffs.</p></div></div><div class="container need-grid">"#);
    body.push_str(&need_card(
        "Systems and embedded",
        "Memory control, native deployment, interop, and long-term maintenance.",
        "/languages/?use_case=systems%20software",
        ["Rust", "C", "C++", "Go"],
    ));
    body.push_str(&need_card(
        "Web and application backends",
        "Runtime fit, package ecosystems, deployment surfaces, and team familiarity.",
        "/languages/?use_case=web%20applications",
        ["JavaScript", "TypeScript", "Python", "PHP"],
    ));
    body.push_str(&need_card(
        "Data and automation",
        "Scripting speed, query semantics, statistical tooling, and operational glue.",
        "/languages/?use_case=data",
        ["Python", "SQL", "R", "Bash"],
    ));
    body.push_str("</div></section>");

    body.push_str(r#"<section class="section"><div class="container section-head"><div><p class="eyebrow">Language shelf</p><h2>Featured languages</h2><p>A short cross-section of the reference. Each profile surfaces the high-value facts first: fit, runtime, typing, memory model, tooling, sources, and last verification date.</p></div><a href="/languages/">Browse all languages</a></div><div class="container card-grid">"#);
    for slug in FEATURED_LANGUAGE_SLUGS {
        if let Some(language) = content.language(slug) {
            body.push_str(&language_card(language));
        }
    }
    body.push_str(r#"</div></section>"#);
    body.push_str(r#"<section class="section"><div class="container section-head"><div><p class="eyebrow">Reference paths</p><h2>Compare, decide, and define terms</h2><p>Use the supporting collections when the language choice depends on adjacent ecosystems or shared concepts.</p></div></div><div class="container browse-grid">"#);
    body.push_str(&hub_card(
        "Comparisons",
        "/comparisons/",
        "Dimensional, tradeoff-first comparisons between related languages.",
        content
            .comparisons
            .iter()
            .take(4)
            .map(|page| {
                (
                    &page.data.title,
                    format!("/comparisons/{}/", page.data.slug),
                )
            })
            .collect(),
    ));
    body.push_str(&hub_card(
        "Guides",
        "/guides/",
        "Decision guides that frame languages by the problem you are solving.",
        content
            .guides
            .iter()
            .take(4)
            .map(|page| (&page.data.title, format!("/guides/{}/", page.data.slug)))
            .collect(),
    ));
    body.push_str(&hub_card(
        "Concepts",
        "/concepts/",
        "Cross-language ideas: type systems, runtimes, memory, and tooling.",
        content
            .concepts
            .iter()
            .take(4)
            .map(|page| (&page.data.title, format!("/concepts/{}/", page.data.slug)))
            .collect(),
    ));
    body.push_str(r#"</div></section>"#);
    render_page(
        PageMeta {
            title: "LangIndex".to_string(),
            description: "Open source, self-hosted reference for programming languages."
                .to_string(),
            section: NavSection::Home,
        },
        body,
        StatusCode::OK,
    )
}

pub async fn about() -> Response {
    render_markdown_page(
        PageMeta {
            title: "About".to_string(),
            description: "What LangIndex is and why it exists.".to_string(),
            section: NavSection::About,
        },
        "About",
        include_str!("../../../README.md"),
    )
}

pub async fn contribute() -> Response {
    render_markdown_page(
        PageMeta {
            title: "Contribute".to_string(),
            description: "How to improve LangIndex content.".to_string(),
            section: NavSection::Contribute,
        },
        "Contribute",
        include_str!("../../../CONTRIBUTING.md"),
    )
}

pub async fn languages_index(
    State(state): State<AppState>,
    Query(filters): Query<LanguageFilters>,
) -> Response {
    let languages = filter_languages(&state.content.languages, &filters);
    let total = state.content.languages.len();
    let showing = languages.len();
    let query = filters.q.as_deref().unwrap_or_default();
    let paradigms =
        unique_language_values(&state.content.languages, |page| page.data.paradigms.clone());
    let typings = unique_language_values(&state.content.languages, |page| {
        let mut values = vec![page.data.typing.discipline.clone()];
        values.extend(page.data.typing.strength.clone());
        values
    });
    let runtimes = unique_language_values(&state.content.languages, |page| {
        vec![page.data.runtime.model.clone()]
    });
    let ecosystems = unique_language_values(&state.content.languages, |page| {
        page.data.package_managers.clone()
    });
    let use_cases = unique_language_values(&state.content.languages, |page| {
        use_case_tags(&page.data.best_for)
    });

    let mut body = index_intro(
        "Languages",
        "Browse language profiles by practical fit, runtime, memory model, typing, tooling, and verified sources.",
    );
    body.push_str(r#"<section class="section"><div class="container"><form class="filter-form" action="/languages/" role="search">"#);
    body.push_str(r#"<div class="filter-grid">"#);
    body.push_str(&format!(
        r#"<label>Search<input name="q" type="search" value="{}" placeholder="Rust, garbage collected, web..." /></label>"#,
        escape_attr(query)
    ));
    body.push_str(&select_filter(
        "Paradigm",
        "paradigm",
        &filters.paradigm,
        &paradigms,
    ));
    body.push_str(&select_filter(
        "Typing",
        "typing",
        &filters.typing,
        &typings,
    ));
    body.push_str(&select_filter(
        "Runtime target",
        "runtime",
        &filters.runtime,
        &runtimes,
    ));
    body.push_str(&select_filter(
        "Ecosystem",
        "ecosystem",
        &filters.ecosystem,
        &ecosystems,
    ));
    body.push_str(&select_filter(
        "Use case",
        "use_case",
        &filters.use_case,
        &use_cases,
    ));
    body.push_str("</div>");
    body.push_str(&format!(
        r#"<div class="filter-actions"><p aria-live="polite">Showing {} {} of {}.</p><button type="submit">Apply filters</button><a class="button" href="/languages/">Reset filters</a></div>"#,
        showing,
        if showing == 1 { "language" } else { "languages" },
        total
    ));
    body.push_str(r#"</form><div class="card-grid">"#);
    for language in languages {
        body.push_str(&language_card(language));
    }
    if showing == 0 {
        body.push_str(r#"<p class="empty-state">No languages match those filters yet. Try resetting or relaxing a constraint.</p>"#);
    }
    body.push_str("</div></div></section>");
    render_page(
        PageMeta {
            title: "Languages".to_string(),
            description: "Browse programming language profiles on LangIndex.".to_string(),
            section: NavSection::Languages,
        },
        body,
        StatusCode::OK,
    )
}

pub async fn language_detail(State(state): State<AppState>, Path(slug): Path<String>) -> Response {
    let Some(page) = state.content.language(&slug) else {
        return not_found().await;
    };
    let mut body = breadcrumb("Languages", "/languages/", &page.data.title);
    body.push_str(r#"<article class="container detail">"#);
    body.push_str(&format!(
        r#"<header class="detail-hero"><div><p class="eyebrow">Language profile</p><h1>{}</h1><p class="lede">{}</p></div><aside class="trust-card" aria-label="Verification summary"><p class="trust-label">Last verified</p><p class="trust-date"><time datetime="{}">{}</time></p><p>{} source{} checked for this profile.</p><p class="trust-actions"><a class="button compact" href="{}">Official site</a>{}</p></aside></header>"#,
        escape(&page.data.title),
        escape(&page.data.summary),
        page.data.last_verified,
        page.data.last_verified,
        page.data.sources.len(),
        if page.data.sources.len() == 1 { "" } else { "s" },
        escape_attr(&page.data.official_site),
        page.data
            .repository
            .as_ref()
            .map(|url| format!(
                r#"<a class="button compact" href="{}">Repository</a>"#,
                escape_attr(url)
            ))
            .unwrap_or_default()
    ));
    body.push_str(r#"<dl class="fact-grid">"#);
    push_fact(&mut body, "Status", &page.data.status);
    if !page.data.creators.is_empty() {
        push_fact(&mut body, "Creator", &page.data.creators.join(", "));
    }
    if !page.data.paradigms.is_empty() {
        push_fact(&mut body, "Paradigms", &page.data.paradigms.join(", "));
    }
    push_fact(
        &mut body,
        "Typing",
        &format!(
            "{}{}",
            page.data.typing.discipline,
            page.data
                .typing
                .strength
                .as_ref()
                .map(|value| format!(", {value}"))
                .unwrap_or_default()
        ),
    );
    push_fact(&mut body, "Runtime", &page.data.runtime.model);
    push_fact(&mut body, "Memory", &page.data.memory.model);
    if let Some(first_released) = page.data.first_released {
        push_fact(&mut body, "First released", &first_released.to_string());
    }
    if !page.data.package_managers.is_empty() {
        push_fact(
            &mut body,
            "Package managers",
            &page.data.package_managers.join(", "),
        );
    }
    body.push_str("</dl>");
    body.push_str(r#"<section class="fit-grid">"#);
    body.push_str(&list_panel("Best fit", &page.data.best_for));
    body.push_str(&list_panel("Poor fit", &page.data.poor_fit));
    body.push_str("</section>");
    body.push_str(r#"<div class="content-body">"#);
    body.push_str(&page.body_html);
    body.push_str("</div>");
    let related = state
        .content
        .comparisons
        .iter()
        .filter(|comparison| {
            comparison
                .data
                .languages
                .iter()
                .any(|item| item == &page.data.slug)
        })
        .map(|comparison| {
            (
                comparison.data.title.as_str(),
                format!("/comparisons/{}/", comparison.data.slug),
            )
        })
        .collect();
    body.push_str(&link_list("Related comparisons", related));
    body.push_str(&source_list(&page.data.sources, page.data.last_verified));
    body.push_str("</article>");
    render_page(
        PageMeta {
            title: page.data.title.clone(),
            description: page.data.summary.clone(),
            section: NavSection::Languages,
        },
        body,
        StatusCode::OK,
    )
}

pub async fn comparisons_index(
    State(state): State<AppState>,
    Query(filters): Query<CollectionFilters>,
) -> Response {
    let total = state.content.comparisons.len();
    let query = filters.q.as_deref().unwrap_or_default();
    let language = filters.language.as_deref().unwrap_or_default();
    let content_ref: &SiteContent = &state.content;
    let language_options = language_filter_options(
        content_ref,
        state
            .content
            .comparisons
            .iter()
            .map(|page| page.data.languages.as_slice()),
    );

    let filtered: Vec<&ComparisonPage> = state
        .content
        .comparisons
        .iter()
        .filter(|page| {
            matches_collection_text(
                &page.data.title,
                &page.data.summary,
                &page.body_markdown,
                query,
            )
        })
        .filter(|page| matches_language(&page.data.languages, language))
        .collect();

    let items: Vec<(&String, &String, String)> = filtered
        .iter()
        .map(|page| {
            (
                &page.data.title,
                &page.data.summary,
                format!("/comparisons/{}/", page.data.slug),
            )
        })
        .collect();

    render_filtered_collection_index(FilteredCollectionParams {
        title: "Comparisons",
        summary: "Dimensional, tradeoff-first comparisons between related languages.",
        section: NavSection::Comparisons,
        action_path: "/comparisons/",
        query,
        language,
        language_options: &language_options,
        item_label: "comparison",
        total,
        items,
    })
}

pub async fn comparison_detail(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Response {
    let Some(page) = state.content.comparison(&slug) else {
        return not_found().await;
    };
    render_standard_detail(StandardDetail {
        collection_label: "Comparisons",
        collection_path: "/comparisons/",
        title: &page.data.title,
        summary: &page.data.summary,
        body_html: &page.body_html,
        sources: &page.data.sources,
        last_verified: page.data.last_verified,
        section: NavSection::Comparisons,
        links: language_links(&state.content, &page.data.languages),
    })
}

pub async fn guides_index(
    State(state): State<AppState>,
    Query(filters): Query<CollectionFilters>,
) -> Response {
    let total = state.content.guides.len();
    let query = filters.q.as_deref().unwrap_or_default();
    let language = filters.language.as_deref().unwrap_or_default();
    let content_ref: &SiteContent = &state.content;
    let language_options = language_filter_options(
        content_ref,
        state
            .content
            .guides
            .iter()
            .map(|page| page.data.candidates.as_slice()),
    );

    let filtered: Vec<&GuidePage> = state
        .content
        .guides
        .iter()
        .filter(|page| {
            matches_collection_text(
                &page.data.title,
                &page.data.summary,
                &page.body_markdown,
                query,
            )
        })
        .filter(|page| matches_language(&page.data.candidates, language))
        .collect();

    let items: Vec<(&String, &String, String)> = filtered
        .iter()
        .map(|page| {
            (
                &page.data.title,
                &page.data.summary,
                format!("/guides/{}/", page.data.slug),
            )
        })
        .collect();

    render_filtered_collection_index(FilteredCollectionParams {
        title: "Guides",
        summary: "Decision guides framed by the problem a developer is solving.",
        section: NavSection::Guides,
        action_path: "/guides/",
        query,
        language,
        language_options: &language_options,
        item_label: "guide",
        total,
        items,
    })
}

pub async fn guide_detail(State(state): State<AppState>, Path(slug): Path<String>) -> Response {
    let Some(page) = state.content.guide(&slug) else {
        return not_found().await;
    };
    render_standard_detail(StandardDetail {
        collection_label: "Guides",
        collection_path: "/guides/",
        title: &page.data.title,
        summary: &page.data.summary,
        body_html: &page.body_html,
        sources: &page.data.sources,
        last_verified: page.data.last_verified,
        section: NavSection::Guides,
        links: language_links(&state.content, &page.data.candidates),
    })
}

pub async fn concepts_index(State(state): State<AppState>) -> Response {
    let mut body = index_intro(
        "Concepts",
        "Cross-language ideas grouped by type systems, memory, runtime, concurrency, paradigms, and tooling.",
    );
    body.push_str(r#"<section class="section"><div class="container concept-groups">"#);
    for group in concept_group_definitions() {
        let pages: Vec<_> = group
            .slugs
            .iter()
            .filter_map(|slug| state.content.concept(slug))
            .collect();
        if pages.is_empty() {
            continue;
        }
        body.push_str(&format!(
            r#"<section class="concept-group" aria-labelledby="concept-group-{}"><div class="section-head"><div><p class="eyebrow">{} concepts</p><h2 id="concept-group-{}">{}</h2><p>{}</p></div><p class="concept-count">{} page{}</p></div><div class="list-grid">"#,
            escape_attr(group.id),
            escape(group.title),
            escape_attr(group.id),
            escape(group.title),
            escape(group.summary),
            pages.len(),
            if pages.len() == 1 { "" } else { "s" }
        ));
        for page in pages {
            body.push_str(&concept_card(page));
        }
        body.push_str("</div></section>");
    }
    body.push_str("</div></section>");
    render_page(
        PageMeta {
            title: "Concepts".to_string(),
            description:
                "Cross-language ideas grouped by type systems, memory, runtime, concurrency, paradigms, and tooling."
                    .to_string(),
            section: NavSection::Concepts,
        },
        body,
        StatusCode::OK,
    )
}

pub async fn concept_detail(State(state): State<AppState>, Path(slug): Path<String>) -> Response {
    let Some(page) = state.content.concept(&slug) else {
        return not_found().await;
    };
    render_standard_detail(StandardDetail {
        collection_label: "Concepts",
        collection_path: "/concepts/",
        title: &page.data.title,
        summary: &page.data.summary,
        body_html: &page.body_html,
        sources: &page.data.sources,
        last_verified: page.data.last_verified,
        section: NavSection::Concepts,
        links: language_links(&state.content, &page.data.related_languages),
    })
}

pub async fn languages_json(State(state): State<AppState>) -> impl IntoResponse {
    #[derive(Serialize)]
    struct LanguageJson {
        title: String,
        slug: String,
        status: String,
        summary: String,
        paradigms: Vec<String>,
        typing: String,
        runtime: String,
        memory: String,
        package_managers: Vec<String>,
        last_verified: String,
        url: String,
    }

    let payload: Vec<_> = state
        .content
        .languages
        .iter()
        .map(|page| LanguageJson {
            title: page.data.title.clone(),
            slug: page.data.slug.clone(),
            status: page.data.status.clone(),
            summary: page.data.summary.clone(),
            paradigms: page.data.paradigms.clone(),
            typing: page.data.typing.discipline.clone(),
            runtime: page.data.runtime.model.clone(),
            memory: page.data.memory.model.clone(),
            package_managers: page.data.package_managers.clone(),
            last_verified: page.data.last_verified.to_string(),
            url: format!("/languages/{}/", page.data.slug),
        })
        .collect();
    Json(payload)
}

pub async fn search_json(State(state): State<AppState>) -> impl IntoResponse {
    #[derive(Serialize)]
    struct SearchItem {
        kind: &'static str,
        title: String,
        summary: String,
        url: String,
        text: String,
    }

    let mut payload = Vec::new();
    payload.extend(state.content.languages.iter().map(|page| SearchItem {
        kind: "language",
        title: page.data.title.clone(),
        summary: page.data.summary.clone(),
        url: format!("/languages/{}/", page.data.slug),
        text: language_search_text(page),
    }));
    payload.extend(state.content.comparisons.iter().map(|page| SearchItem {
        kind: "comparison",
        title: page.data.title.clone(),
        summary: page.data.summary.clone(),
        url: format!("/comparisons/{}/", page.data.slug),
        text: format!(
            "{} {} {}",
            page.data.title, page.data.summary, page.body_markdown
        ),
    }));
    payload.extend(state.content.guides.iter().map(|page| SearchItem {
        kind: "guide",
        title: page.data.title.clone(),
        summary: page.data.summary.clone(),
        url: format!("/guides/{}/", page.data.slug),
        text: format!(
            "{} {} {}",
            page.data.title, page.data.summary, page.body_markdown
        ),
    }));
    payload.extend(state.content.concepts.iter().map(|page| SearchItem {
        kind: "concept",
        title: page.data.title.clone(),
        summary: page.data.summary.clone(),
        url: format!("/concepts/{}/", page.data.slug),
        text: format!(
            "{} {} {}",
            page.data.title, page.data.summary, page.body_markdown
        ),
    }));

    Json(payload)
}

pub async fn rss(State(state): State<AppState>) -> Response {
    let mut body = String::from(r#"<?xml version="1.0" encoding="utf-8"?>"#);
    body.push_str(r#"<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom"><channel>"#);
    body.push_str("<title>LangIndex</title><link>https://langindex.dev/</link><description>Source-backed programming language reference updates.</description>");
    body.push_str(r#"<atom:link href="https://langindex.dev/rss.xml" rel="self" type="application/rss+xml" />"#);
    for item in feed_items(&state.content) {
        body.push_str("<item>");
        body.push_str(&format!(
            "<title>{}</title><link>{}{}</link><guid>{}{}</guid><description>{}</description><pubDate>{}</pubDate>",
            escape(&item.title),
            SITE_URL,
            item.path,
            SITE_URL,
            item.path,
            escape(&item.summary),
            item.last_verified.format("%a, %d %b %Y 00:00:00 GMT")
        ));
        body.push_str("</item>");
    }
    body.push_str("</channel></rss>");
    xml_response(body, "application/rss+xml; charset=utf-8")
}

pub async fn sitemap(State(state): State<AppState>) -> Response {
    let mut body = String::from(r#"<?xml version="1.0" encoding="utf-8"?>"#);
    body.push_str(r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);
    for route in state.content.all_routes() {
        if matches!(
            route.as_str(),
            "/robots.txt" | "/languages.json" | "/search.json" | "/rss.xml"
        ) {
            continue;
        }
        body.push_str("<url><loc>");
        body.push_str(SITE_URL);
        body.push_str(&escape(&route));
        body.push_str("</loc></url>");
    }
    body.push_str("</urlset>");
    xml_response(body, "application/xml; charset=utf-8")
}

pub async fn robots() -> Response {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        "User-agent: *\nAllow: /\n\nSitemap: https://langindex.dev/sitemap.xml\n",
    )
        .into_response()
}

pub async fn healthz() -> &'static str {
    "ok"
}

pub async fn not_found() -> Response {
    render_page(
        PageMeta {
            title: "Page not found".to_string(),
            description: "The requested LangIndex page was not found.".to_string(),
            section: NavSection::None,
        },
        r#"<section class="section"><div class="container"><h1>Page not found</h1><p>The page you requested does not exist in the LangIndex route set.</p><p><a href="/languages/">Browse languages</a></p></div></section>"#.to_string(),
        StatusCode::NOT_FOUND,
    )
}

struct StandardDetail<'a> {
    collection_label: &'a str,
    collection_path: &'a str,
    title: &'a str,
    summary: &'a str,
    body_html: &'a str,
    sources: &'a [Source],
    last_verified: NaiveDate,
    section: NavSection,
    links: Vec<(&'a str, String)>,
}

fn render_standard_detail(detail: StandardDetail<'_>) -> Response {
    let mut body = breadcrumb(
        detail.collection_label,
        detail.collection_path,
        detail.title,
    );
    body.push_str(r#"<article class="container detail">"#);
    body.push_str(&format!(
        r#"<header class="detail-hero"><div><p class="eyebrow">{}</p><h1>{}</h1><p class="lede">{}</p></div><aside class="trust-card" aria-label="Verification summary"><p class="trust-label">Last verified</p><p class="trust-date"><time datetime="{}">{}</time></p><p>{} source{} checked for this page.</p></aside></header>"#,
        escape(detail.collection_label.trim_end_matches('s')),
        escape(detail.title),
        escape(detail.summary),
        detail.last_verified,
        detail.last_verified,
        detail.sources.len(),
        if detail.sources.len() == 1 { "" } else { "s" }
    ));
    body.push_str(&link_list("Related languages", detail.links));
    body.push_str(r#"<div class="content-body">"#);
    body.push_str(detail.body_html);
    body.push_str("</div>");
    body.push_str(&source_list(detail.sources, detail.last_verified));
    body.push_str("</article>");
    render_page(
        PageMeta {
            title: detail.title.to_string(),
            description: detail.summary.to_string(),
            section: detail.section,
        },
        body,
        StatusCode::OK,
    )
}

#[derive(Debug, Default, Deserialize)]
pub struct CollectionFilters {
    q: Option<String>,
    language: Option<String>,
}

struct FilteredCollectionParams<'a> {
    title: &'a str,
    summary: &'a str,
    section: NavSection,
    action_path: &'a str,
    query: &'a str,
    language: &'a str,
    language_options: &'a [(String, String)],
    item_label: &'a str,
    total: usize,
    items: Vec<(&'a String, &'a String, String)>,
}

fn render_filtered_collection_index(params: FilteredCollectionParams<'_>) -> Response {
    let mut body = index_intro(params.title, params.summary);
    let showing = params.items.len();
    let plural_label = if showing == 1 {
        params.item_label.to_string()
    } else {
        format!("{}s", params.item_label)
    };

    body.push_str(r#"<section class="section"><div class="container">"#);
    body.push_str(&format!(
        r#"<form class="filter-form" action="{}" role="search"><div class="filter-grid filter-grid-compact">"#,
        escape_attr(params.action_path)
    ));
    body.push_str(&format!(
        r#"<label>Search<input name="q" type="search" value="{}" placeholder="Filter by title or summary..." /></label>"#,
        escape_attr(params.query)
    ));
    let selected_lang = normalize_filter(params.language);
    body.push_str(
        r#"<label>Language<select name="language"><option value="">Any language</option>"#,
    );
    for (slug, label) in params.language_options {
        let is_selected = if slug == &selected_lang {
            " selected"
        } else {
            ""
        };
        body.push_str(&format!(
            r#"<option value="{}"{}>{}</option>"#,
            escape_attr(slug),
            is_selected,
            escape(label)
        ));
    }
    body.push_str("</select></label>");
    body.push_str("</div>");
    body.push_str(&format!(
        r#"<div class="filter-actions"><p aria-live="polite">Showing {} {} of {}.</p><button type="submit">Apply filters</button><a class="button" href="{}">Reset filters</a></div>"#,
        showing,
        plural_label,
        params.total,
        escape_attr(params.action_path)
    ));
    body.push_str("</form>");
    body.push_str(r#"<div class="list-grid">"#);
    for (item_title, item_summary, href) in &params.items {
        body.push_str(&format!(
            r#"<article class="card collection-card"><p class="eyebrow">Reference page</p><h2><a href="{}">{}</a></h2><p>{}</p><p class="card-footer"><a class="card-action" href="{}">Open page</a></p></article>"#,
            escape_attr(href),
            escape(item_title),
            escape(item_summary),
            escape_attr(href)
        ));
    }
    if showing == 0 {
        body.push_str(r#"<p class="empty-state">No matches yet. Try resetting filters or broadening the search.</p>"#);
    }
    body.push_str("</div></div></section>");
    render_page(
        PageMeta {
            title: params.title.to_string(),
            description: params.summary.to_string(),
            section: params.section,
        },
        body,
        StatusCode::OK,
    )
}

fn matches_collection_text(title: &str, summary: &str, body: &str, query: &str) -> bool {
    let normalized = normalize_filter(query);
    if normalized.is_empty() {
        return true;
    }
    let haystack = format!(
        "{} {} {}",
        normalize_filter(title),
        normalize_filter(summary),
        normalize_filter(body)
    );
    normalized
        .split_whitespace()
        .all(|term| haystack.contains(term))
}

fn matches_language(languages: &[String], selected: &str) -> bool {
    let selected = normalize_filter(selected);
    if selected.is_empty() {
        return true;
    }
    languages
        .iter()
        .any(|slug| normalize_filter(slug) == selected)
}

fn language_filter_options<'a>(
    content: &SiteContent,
    pages: impl Iterator<Item = &'a [String]>,
) -> Vec<(String, String)> {
    let mut seen = std::collections::BTreeMap::new();
    for slugs in pages {
        for slug in slugs {
            let normalized = normalize_filter(slug);
            if normalized.is_empty() {
                continue;
            }
            let label = content
                .language(&normalized)
                .map(|page| page.data.title.clone())
                .unwrap_or_else(|| normalized.clone());
            seen.entry(normalized).or_insert(label);
        }
    }
    let mut options: Vec<_> = seen.into_iter().collect();
    options.sort_by_key(|entry| entry.1.to_lowercase());
    options
}

struct ConceptGroupDefinition {
    id: &'static str,
    title: &'static str,
    summary: &'static str,
    slugs: &'static [&'static str],
}

fn concept_group_definitions() -> Vec<ConceptGroupDefinition> {
    vec![
        ConceptGroupDefinition {
            id: "type-systems",
            title: "Type Systems",
            summary: "How languages describe values, generic code, absence, inference, and type relationships.",
            slugs: &[
                "static-vs-dynamic-typing",
                "strong-vs-weak-typing",
                "type-inference",
                "structural-vs-nominal-typing",
                "generics-and-parametric-polymorphism",
                "algebraic-data-types-and-pattern-matching",
                "null-safety",
            ],
        },
        ConceptGroupDefinition {
            id: "memory",
            title: "Memory",
            summary: "Resource lifetime, allocation, reclamation, and safety models across native and managed runtimes.",
            slugs: &[
                "ownership",
                "garbage-collection",
                "reference-counting",
                "manual-memory-management",
                "raii-and-deterministic-cleanup",
                "stack-vs-heap-allocation",
                "memory-safety",
            ],
        },
        ConceptGroupDefinition {
            id: "runtime",
            title: "Runtime And Execution",
            summary: "Interpreters, bytecode, compilation targets, interop boundaries, ABIs, and standard libraries.",
            slugs: &[
                "interpreters-jit-and-aot",
                "virtual-machines-and-bytecode",
                "compilation-targets",
                "foreign-function-interface",
                "abi-stability",
                "standard-library-philosophy",
            ],
        },
        ConceptGroupDefinition {
            id: "concurrency",
            title: "Concurrency",
            summary: "Threads, async runtimes, message passing, lightweight tasks, memory models, and scoped task lifetimes.",
            slugs: &[
                "threads-and-shared-memory",
                "async-await-and-event-loops",
                "goroutines-and-green-threads",
                "actor-model-and-message-passing",
                "data-races-and-memory-models",
                "structured-concurrency",
            ],
        },
        ConceptGroupDefinition {
            id: "paradigms",
            title: "Paradigms And Language Design",
            summary: "Programming styles and design mechanisms that shape APIs, state, errors, code generation, and boundaries.",
            slugs: &[
                "functional-programming",
                "object-oriented-programming",
                "immutability-and-persistent-data-structures",
                "closures-and-first-class-functions",
                "errors-as-values-vs-exceptions",
                "metaprogramming-and-macros",
                "modules-and-namespacing",
            ],
        },
        ConceptGroupDefinition {
            id: "tooling",
            title: "Tooling",
            summary: "Ecosystem machinery for packages, builds, editors, formatting, tests, docs, and interactive work.",
            slugs: &[
                "package-managers",
                "build-systems",
                "formatters-and-linters",
                "language-servers-and-editor-tooling",
                "repl-and-interactive-development",
                "testing-cultures",
                "documentation-cultures",
            ],
        },
    ]
}

fn concept_card(page: &ConceptPage) -> String {
    let href = format!("/concepts/{}/", page.data.slug);
    format!(
        r#"<article class="card collection-card"><p class="eyebrow">Reference page</p><h3><a href="{}">{}</a></h3><p>{}</p><p class="card-footer"><a class="card-action" href="{}">Open page</a></p></article>"#,
        escape_attr(&href),
        escape(&page.data.title),
        escape(&page.data.summary),
        escape_attr(&href)
    )
}

fn render_markdown_page(meta: PageMeta, heading: &str, markdown: &str) -> Response {
    let mut body = index_intro(heading, &meta.description);
    body.push_str(r#"<section class="section"><div class="container content-body">"#);
    body.push_str(&markdown_body(markdown));
    body.push_str("</div></section>");
    render_page(meta, body, StatusCode::OK)
}

fn markdown_body(markdown: &str) -> String {
    use pulldown_cmark::{Options, Parser, html};

    let body = markdown
        .strip_prefix("---")
        .and_then(|rest| rest.split_once("---").map(|(_, body)| body))
        .unwrap_or(markdown);
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(body, options);
    let mut output = String::new();
    html::push_html(&mut output, parser);
    let output = output
        .replace("<pre>", r#"<pre tabindex="0">"#)
        .replace(r#"src="public/"#, r#"src="/"#);
    ammonia::Builder::default()
        .add_generic_attributes(["class", "tabindex"])
        .clean(&output)
        .to_string()
}

fn render_page(meta: PageMeta, body_html: String, status: StatusCode) -> Response {
    let view = view! {
        <Layout title=meta.title.clone() description=meta.description.clone() section=meta.section body_html=body_html />
    };
    let document = format!("<!DOCTYPE html>{}", view.to_html());
    (
        status,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        document,
    )
        .into_response()
}

#[component]
fn Layout(
    title: String,
    description: String,
    section: NavSection,
    body_html: String,
) -> impl IntoView {
    let full_title = if section == NavSection::Home {
        "LangIndex — source-backed programming language reference".to_string()
    } else {
        format!("{title} · LangIndex")
    };

    view! {
        <html lang="en" data-theme="dark">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <meta name="color-scheme" content="dark light" />
                <meta name="theme-color" content="#0d1117" media="(prefers-color-scheme: dark)" />
                <meta name="theme-color" content="#ffffff" media="(prefers-color-scheme: light)" />
                <meta name="description" content=description />
                <link rel="icon" type="image/png" href="/favicon.png" />
                <link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png" />
                <link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png" />
                <link rel="apple-touch-icon" href="/apple-touch-icon.png" />
                <link rel="manifest" href="/site.webmanifest" />
                <link rel="alternate" type="application/rss+xml" title="LangIndex RSS" href="/rss.xml" />
                <link rel="stylesheet" href="/assets/style.css?v=1.0.0" />
                <title>{full_title}</title>
                <script>{r#"(function(){try{var t=localStorage.getItem("langindex-theme");if(t!=="light"&&t!=="dark"){t="dark";}document.documentElement.setAttribute("data-theme",t);}catch(e){document.documentElement.setAttribute("data-theme","dark");}})();"#}</script>
            </head>
            <body>
                <a class="skip" href="#main">"Skip to content"</a>
                <SiteHeader section=section />
                <main id="main" inner_html=body_html></main>
                <SiteFooter />
                <script>{r#"
                    (function () {
                        var toggle = document.querySelector("[data-mobile-nav-toggle]");
                        var nav = document.querySelector("[data-mobile-nav]");
                        if (toggle && nav) {
                            toggle.addEventListener("click", function () {
                                var expanded = toggle.getAttribute("aria-expanded") === "true";
                                toggle.setAttribute("aria-expanded", String(!expanded));
                                nav.hidden = expanded;
                            });
                        }

                        var themeButton = document.querySelector("[data-theme-toggle]");
                        var themeLabel = document.querySelector("[data-theme-label]");
                        function syncLabel(theme) {
                            if (themeLabel) {
                                themeLabel.textContent = theme === "dark" ? "Switch to light theme" : "Switch to dark theme";
                            }
                        }
                        function setTheme(theme) {
                            document.documentElement.setAttribute("data-theme", theme);
                            syncLabel(theme);
                            try { localStorage.setItem("langindex-theme", theme); } catch (error) {}
                        }
                        syncLabel(document.documentElement.getAttribute("data-theme") || "dark");
                        if (themeButton) {
                            themeButton.addEventListener("click", function () {
                                var current = document.documentElement.getAttribute("data-theme") || "dark";
                                setTheme(current === "dark" ? "light" : "dark");
                            });
                        }

                        var searchRoot = document.querySelector("[data-live-search]");
                        if (searchRoot) {
                            var searchInput = searchRoot.querySelector("[data-live-search-input]");
                            var searchResults = searchRoot.querySelector("[data-live-search-results]");
                            var searchStatus = searchRoot.querySelector("[data-live-search-status]");
                            var searchItems = null;

                            function clearSearch(message) {
                                if (searchResults) searchResults.replaceChildren();
                                if (searchStatus) searchStatus.textContent = message || "";
                            }

                            function renderSearch(items) {
                                if (!searchResults || !searchStatus) return;
                                searchResults.replaceChildren();
                                if (!items.length) {
                                    searchStatus.textContent = "No matches yet.";
                                    return;
                                }
                                searchStatus.textContent = items.length + (items.length === 1 ? " match" : " matches");
                                items.slice(0, 6).forEach(function (item) {
                                    var link = document.createElement("a");
                                    link.className = "search-result";
                                    link.href = item.url;
                                    link.setAttribute("aria-label", item.kind + " " + item.title);
                                    var kind = document.createElement("span");
                                    kind.className = "search-kind";
                                    kind.textContent = item.kind;
                                    var title = document.createElement("strong");
                                    title.textContent = item.title;
                                    var summary = document.createElement("span");
                                    summary.textContent = item.summary;
                                    link.append(kind, title, summary);
                                    searchResults.append(link);
                                });
                            }

                            function runSearch() {
                                var query = (searchInput && searchInput.value || "").trim().toLowerCase();
                                if (query.length < 2) {
                                    clearSearch("Type at least two characters.");
                                    return;
                                }
                                var terms = query.split(/\s+/).filter(Boolean);
                                var source = searchItems || [];
                                var matches = source.map(function (item, index) {
                                    var haystack = [item.kind, item.title, item.summary, item.text].join(" ").toLowerCase();
                                    if (!terms.every(function (term) { return haystack.indexOf(term) !== -1; })) return null;
                                    var title = (item.title || "").toLowerCase();
                                    var summary = (item.summary || "").toLowerCase();
                                    var kind = (item.kind || "").toLowerCase();
                                    var score = 0;
                                    terms.forEach(function (term) {
                                        if (title === term) score += 120;
                                        if (title.indexOf(term) !== -1) score += 80;
                                        if (kind.indexOf(term) !== -1) score += 20;
                                        if (summary.indexOf(term) !== -1) score += 10;
                                    });
                                    return { item: item, score: score, index: index };
                                }).filter(Boolean).sort(function (left, right) {
                                    return right.score - left.score || left.index - right.index;
                                }).map(function (match) { return match.item; });
                                renderSearch(matches);
                            }

                            function ensureSearchLoaded() {
                                if (searchItems) {
                                    runSearch();
                                    return;
                                }
                                fetch("/search.json", { headers: { "Accept": "application/json" } })
                                    .then(function (response) { return response.ok ? response.json() : []; })
                                    .then(function (items) {
                                        searchItems = Array.isArray(items) ? items : [];
                                        runSearch();
                                    })
                                    .catch(function () {
                                        clearSearch("Search index is unavailable. The language filters still work.");
                                    });
                            }

                            if (searchInput) {
                                searchInput.addEventListener("input", ensureSearchLoaded);
                                searchInput.addEventListener("focus", ensureSearchLoaded);
                            }
                        }
                    })();
                "#}</script>
            </body>
        </html>
    }
}

#[component]
fn SiteHeader(section: NavSection) -> impl IntoView {
    view! {
        <header class="site-header">
            <div class="container header-inner">
                <a class="brand" href="/">
                    <span class="brand-mark" aria-hidden="true">
                        <img src="/brand/langindex-logo-64.png" alt="" width="28" height="28" />
                    </span>
                    <span class="brand-name">"LangIndex"</span>
                </a>
                <nav class="primary-nav" aria-label="Primary">
                    <NavLink href="/" active=section == NavSection::Home>"Home"</NavLink>
                    <NavLink href="/languages/" active=section == NavSection::Languages>"Languages"</NavLink>
                    <NavLink href="/comparisons/" active=section == NavSection::Comparisons>"Comparisons"</NavLink>
                    <NavLink href="/guides/" active=section == NavSection::Guides>"Guides"</NavLink>
                    <NavLink href="/concepts/" active=section == NavSection::Concepts>"Concepts"</NavLink>
                    <NavLink href="/about" active=section == NavSection::About>"About"</NavLink>
                    <NavLink href="/contribute" active=section == NavSection::Contribute>"Contribute"</NavLink>
                </nav>
                <div class="header-actions">
                    <a class="icon-button" href="https://github.com/dunamismax/langindex" aria-label="Source on GitHub" title="Source on GitHub" rel="noopener" target="_blank">
                        <svg aria-hidden="true" viewBox="0 0 16 16" width="16" height="16" fill="currentColor"><path d="M8 0c4.42 0 8 3.58 8 8a8.013 8.013 0 0 1-5.45 7.59c-.4.08-.55-.17-.55-.38 0-.27.01-1.13.01-2.2 0-.75-.25-1.23-.54-1.48 1.78-.2 3.65-.88 3.65-3.95 0-.88-.31-1.59-.82-2.15.08-.2.36-1.02-.08-2.12 0 0-.67-.22-2.2.82-.64-.18-1.32-.27-2-.27-.68 0-1.36.09-2 .27-1.53-1.03-2.2-.82-2.2-.82-.44 1.1-.16 1.92-.08 2.12-.51.56-.82 1.28-.82 2.15 0 3.06 1.86 3.75 3.64 3.95-.23.2-.44.55-.51 1.07-.46.21-1.61.55-2.33-.66-.15-.24-.6-.83-1.23-.82-.67.01-.27.38.01.53.34.19.73.9.82 1.13.16.45.68 1.31 2.69.94 0 .67.01 1.3.01 1.49 0 .21-.15.45-.55.38A7.995 7.995 0 0 1 0 8c0-4.42 3.58-8 8-8Z"/></svg>
                    </a>
                    <button type="button" class="icon-button theme-toggle" data-theme-toggle aria-label="Toggle color theme" title="Toggle color theme">
                        <span data-theme-label class="sr-only">"Switch to light theme"</span>
                        <svg class="theme-icon-moon" aria-hidden="true" viewBox="0 0 16 16" width="16" height="16" fill="currentColor"><path d="M9.598 1.591a.749.749 0 0 1 .785-.175 7.001 7.001 0 1 1-8.967 8.967.75.75 0 0 1 .961-.96 5.5 5.5 0 0 0 7.046-7.046.75.75 0 0 1 .175-.786Z"/></svg>
                        <svg class="theme-icon-sun" aria-hidden="true" viewBox="0 0 16 16" width="16" height="16" fill="currentColor"><path d="M8 12a4 4 0 1 1 0-8 4 4 0 0 1 0 8ZM8 0a.75.75 0 0 1 .75.75v1.5a.75.75 0 0 1-1.5 0V.75A.75.75 0 0 1 8 0Zm0 13a.75.75 0 0 1 .75.75v1.5a.75.75 0 0 1-1.5 0v-1.5A.75.75 0 0 1 8 13Zm8-5a.75.75 0 0 1-.75.75h-1.5a.75.75 0 0 1 0-1.5h1.5A.75.75 0 0 1 16 8ZM3 8a.75.75 0 0 1-.75.75H.75a.75.75 0 0 1 0-1.5h1.5A.75.75 0 0 1 3 8Zm10.657-5.657a.75.75 0 0 1 0 1.061l-1.06 1.06a.749.749 0 1 1-1.061-1.06l1.06-1.061a.75.75 0 0 1 1.061 0Zm-7.95 7.95a.75.75 0 0 1 0 1.061L4.646 12.41a.751.751 0 0 1-1.225-.244.751.751 0 0 1 .165-.817l1.06-1.06a.75.75 0 0 1 1.061 0Zm7.95 1.06a.749.749 0 1 1-1.06 1.061l-1.061-1.06a.749.749 0 1 1 1.06-1.061l1.061 1.06ZM5.708 5.707a.749.749 0 1 1-1.06 1.061L3.585 5.708a.749.749 0 1 1 1.061-1.06l1.06 1.06Z"/></svg>
                    </button>
                    <button type="button" class="icon-button mobile-nav-toggle" data-mobile-nav-toggle aria-expanded="false" aria-controls="mobile-nav" aria-label="Toggle navigation">
                        <svg aria-hidden="true" viewBox="0 0 16 16" width="16" height="16" fill="currentColor"><path d="M1 2.75A.75.75 0 0 1 1.75 2h12.5a.75.75 0 0 1 0 1.5H1.75A.75.75 0 0 1 1 2.75Zm0 5A.75.75 0 0 1 1.75 7h12.5a.75.75 0 0 1 0 1.5H1.75A.75.75 0 0 1 1 7.75ZM1.75 12h12.5a.75.75 0 0 1 0 1.5H1.75a.75.75 0 0 1 0-1.5Z"/></svg>
                    </button>
                </div>
            </div>
            <nav id="mobile-nav" class="mobile-nav container" aria-label="Mobile" data-mobile-nav hidden>
                <NavLink href="/" active=section == NavSection::Home>"Home"</NavLink>
                <NavLink href="/languages/" active=section == NavSection::Languages>"Languages"</NavLink>
                <NavLink href="/comparisons/" active=section == NavSection::Comparisons>"Comparisons"</NavLink>
                <NavLink href="/guides/" active=section == NavSection::Guides>"Guides"</NavLink>
                <NavLink href="/concepts/" active=section == NavSection::Concepts>"Concepts"</NavLink>
                <NavLink href="/about" active=section == NavSection::About>"About"</NavLink>
                <NavLink href="/contribute" active=section == NavSection::Contribute>"Contribute"</NavLink>
            </nav>
        </header>
    }
}

#[component]
fn NavLink(href: &'static str, active: bool, children: Children) -> impl IntoView {
    let class = if active {
        "nav-link active"
    } else {
        "nav-link"
    };
    view! { <a class=class href=href>{children()}</a> }
}

#[component]
fn SiteFooter() -> impl IntoView {
    view! {
        <footer class="site-footer">
            <div class="container footer-inner">
                <p>"LangIndex — open source, self-hosted, source-backed."</p>
                <nav aria-label="Footer">
                    <a href="/rss.xml">"RSS"</a>
                    <a href="/sitemap.xml">"Sitemap"</a>
                    <a href="https://github.com/dunamismax/langindex">"Source"</a>
                </nav>
            </div>
        </footer>
    }
}

fn home_search_panel() -> String {
    [
        r#"<aside class="search-panel" data-live-search>"#,
        r#"<div class="search-panel-head"><p class="eyebrow">Find the right page</p><h2>Search the reference</h2><p>Search profiles, comparisons, guides, concepts, sources, and examples.</p></div>"#,
        r#"<form class="site-search" action="/languages/" role="search"><label for="q">Search languages and topics</label><div class="search-row"><input id="q" name="q" type="search" placeholder="Rust, garbage collection, web..." autocomplete="off" data-live-search-input /><button type="submit">Search</button></div></form>"#,
        r#"<p class="search-status small" aria-live="polite" data-live-search-status>Type at least two characters.</p>"#,
        r#"<div class="search-results" data-live-search-results></div>"#,
        r#"<div class="quick-links" aria-label="Popular starting points"><a href="/languages/rust/">Rust</a><a href="/languages/python/">Python</a><a href="/comparisons/rust-vs-go/">Rust vs Go</a><a href="/guides/choosing-a-systems-language/">Systems guide</a></div>"#,
        r#"</aside>"#,
    ]
    .join("")
}

fn stats(content: &SiteContent) -> String {
    let stats = [
        (
            "Languages",
            content.languages.len(),
            "Profiles with fit, runtime, typing, tooling, sources, and examples.",
            "/languages/",
        ),
        (
            "Comparisons",
            content.comparisons.len(),
            "Tradeoff maps for adjacent language choices.",
            "/comparisons/",
        ),
        (
            "Guides",
            content.guides.len(),
            "Decision paths framed by real development constraints.",
            "/guides/",
        ),
        (
            "Concepts",
            content.concepts.len(),
            "Shared vocabulary for memory, runtime, typing, and tooling.",
            "/concepts/",
        ),
    ];
    let mut body = r#"<section class="stats"><div class="container stat-grid">"#.to_string();
    for (label, value, description, href) in stats {
        body.push_str(&format!(
            r#"<a class="stat" href="{href}"><span>{}</span><strong>{}</strong><em>{}</em></a>"#,
            escape(label),
            value,
            escape(description)
        ));
    }
    body.push_str("</div></section>");
    body
}

fn language_card(language: &LanguagePage) -> String {
    let mut tags = Vec::new();
    tags.push(language.data.status.clone());
    tags.push(language.data.typing.discipline.clone());
    tags.extend(language.data.paradigms.iter().take(2).cloned());

    format!(
        r#"<article class="card language-card"><header class="card-header"><h2><a href="/languages/{}/">{}</a></h2><span class="verified-mini"><time datetime="{}">{}</time></span></header><p>{}</p>{}<dl><div><dt>Runtime</dt><dd>{}</dd></div><div><dt>Memory</dt><dd>{}</dd></div><div><dt>Tooling</dt><dd>{}</dd></div></dl><p class="card-footer"><a class="card-action" href="/languages/{}/">Open {}</a></p></article>"#,
        escape_attr(&language.data.slug),
        escape(&language.data.title),
        language.data.last_verified,
        language.data.last_verified,
        escape(&language.data.summary),
        tag_list(&tags),
        escape(&language.data.runtime.model),
        escape(&language.data.memory.model),
        escape(&compact_tooling(&language.data.package_managers)),
        escape_attr(&language.data.slug),
        escape(&language.data.title)
    )
}

fn compact_tooling(package_managers: &[String]) -> String {
    if package_managers.is_empty() {
        "Varies by ecosystem".to_string()
    } else {
        package_managers
            .iter()
            .take(3)
            .cloned()
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn tag_list(tags: &[String]) -> String {
    if tags.is_empty() {
        return String::new();
    }
    let mut body = r#"<ul class="tag-list" aria-label="Language traits">"#.to_string();
    for tag in tags.iter().filter(|tag| !tag.trim().is_empty()).take(4) {
        body.push_str(&format!("<li>{}</li>", escape(tag)));
    }
    body.push_str("</ul>");
    body
}

fn need_card(title: &str, summary: &str, href: &str, languages: [&str; 4]) -> String {
    let mut body = format!(
        r#"<article class="need-card"><h3><a href="{}">{}</a></h3><p>{}</p><ul>"#,
        escape_attr(href),
        escape(title),
        escape(summary)
    );
    for language in languages {
        body.push_str(&format!("<li>{}</li>", escape(language)));
    }
    body.push_str("</ul></article>");
    body
}

#[derive(Debug, Default, Deserialize)]
pub struct LanguageFilters {
    q: Option<String>,
    paradigm: Option<String>,
    typing: Option<String>,
    runtime: Option<String>,
    ecosystem: Option<String>,
    use_case: Option<String>,
}

fn filter_languages<'a>(
    languages: &'a [LanguagePage],
    filters: &LanguageFilters,
) -> Vec<&'a LanguagePage> {
    languages
        .iter()
        .filter(|page| {
            matches_text(page, filters.q.as_deref())
                && matches_any(&page.data.paradigms, filters.paradigm.as_deref())
                && matches_any(
                    &[
                        page.data.typing.discipline.as_str(),
                        page.data.typing.strength.as_deref().unwrap_or_default(),
                    ],
                    filters.typing.as_deref(),
                )
                && matches_value(&page.data.runtime.model, filters.runtime.as_deref())
                && matches_any(&page.data.package_managers, filters.ecosystem.as_deref())
                && matches_any(
                    &use_case_tags(&page.data.best_for),
                    filters.use_case.as_deref(),
                )
        })
        .collect()
}

fn matches_text(page: &LanguagePage, query: Option<&str>) -> bool {
    let Some(query) = query
        .map(normalize_filter)
        .filter(|value| !value.is_empty())
    else {
        return true;
    };
    normalize_filter(&language_search_text(page)).contains(&query)
}

fn matches_value(value: &str, selected: Option<&str>) -> bool {
    let Some(selected) = selected
        .map(normalize_filter)
        .filter(|value| !value.is_empty())
    else {
        return true;
    };
    normalize_filter(value) == selected
}

fn matches_any<T: AsRef<str>>(values: &[T], selected: Option<&str>) -> bool {
    let Some(selected) = selected
        .map(normalize_filter)
        .filter(|value| !value.is_empty())
    else {
        return true;
    };
    values
        .iter()
        .any(|value| normalize_filter(value.as_ref()) == selected)
}

fn language_search_text(page: &LanguagePage) -> String {
    [
        page.data.title.as_str(),
        page.data.summary.as_str(),
        &page.data.paradigms.join(" "),
        page.data.typing.discipline.as_str(),
        page.data.typing.strength.as_deref().unwrap_or_default(),
        page.data.runtime.model.as_str(),
        page.data.memory.model.as_str(),
        &page.data.package_managers.join(" "),
        &page.data.best_for.join(" "),
        &page.data.poor_fit.join(" "),
        page.body_markdown.as_str(),
    ]
    .join(" ")
}

fn use_case_tags(items: &[String]) -> Vec<String> {
    let text = normalize_filter(&items.join(" "));
    let rules = [
        ("cli tools", ["command-line", "scripts"]),
        ("data", ["data", "analytics"]),
        ("infrastructure", ["infrastructure", "distributed"]),
        ("network services", ["network services", "services"]),
        ("systems software", ["systems software", "embedded"]),
        ("web applications", ["web", "node.js"]),
    ];

    rules
        .iter()
        .filter(|(_, terms)| terms.iter().any(|term| text.contains(term)))
        .map(|(label, _)| (*label).to_string())
        .collect()
}

fn unique_language_values(
    languages: &[LanguagePage],
    values: impl Fn(&LanguagePage) -> Vec<String>,
) -> Vec<String> {
    let mut normalized = std::collections::BTreeMap::new();
    for language in languages {
        for value in values(language) {
            if value.trim().is_empty() {
                continue;
            }
            normalized
                .entry(normalize_filter(&value))
                .or_insert_with(|| value.trim().to_string());
        }
    }
    normalized.into_values().collect()
}

fn select_filter(label: &str, name: &str, selected: &Option<String>, options: &[String]) -> String {
    let mut body = format!(
        r#"<label>{}<select name="{}"><option value="">Any</option>"#,
        escape(label),
        escape_attr(name)
    );
    let selected = selected
        .as_deref()
        .map(normalize_filter)
        .unwrap_or_default();
    for option in options {
        let value = normalize_filter(option);
        let is_selected = if value == selected { " selected" } else { "" };
        body.push_str(&format!(
            r#"<option value="{}"{}>{}</option>"#,
            escape_attr(&value),
            is_selected,
            escape(option)
        ));
    }
    body.push_str("</select></label>");
    body
}

fn normalize_filter(value: &str) -> String {
    value.trim().to_lowercase()
}

fn hub_card(title: &str, href: &str, description: &str, items: Vec<(&String, String)>) -> String {
    let mut body = format!(
        r#"<article class="card hub-card"><h3><a href="{}">{}</a></h3><p>{}</p><ul>"#,
        escape_attr(href),
        escape(title),
        escape(description)
    );
    for (item_title, item_href) in items {
        body.push_str(&format!(
            r#"<li><a href="{}">{}</a></li>"#,
            escape_attr(&item_href),
            escape(item_title)
        ));
    }
    body.push_str("</ul></article>");
    body
}

fn index_intro(title: &str, summary: &str) -> String {
    format!(
        r#"<section class="section intro"><div class="container"><p class="eyebrow">Index</p><h1>{}</h1><p class="lede">{}</p></div></section>"#,
        escape(title),
        escape(summary)
    )
}

fn breadcrumb(collection: &str, href: &str, title: &str) -> String {
    format!(
        r#"<nav class="container breadcrumb" aria-label="Breadcrumb"><a href="{}">{}</a><span aria-hidden="true">/</span><span>{}</span></nav>"#,
        escape_attr(href),
        escape(collection),
        escape(title)
    )
}

fn push_fact(body: &mut String, label: &str, value: &str) {
    body.push_str(&format!(
        r#"<div><dt>{}</dt><dd>{}</dd></div>"#,
        escape(label),
        escape(value)
    ));
}

fn list_panel(title: &str, items: &[String]) -> String {
    let mut body = format!(r#"<section class="panel"><h2>{}</h2><ul>"#, escape(title));
    for item in items {
        body.push_str(&format!("<li>{}</li>", escape(item)));
    }
    body.push_str("</ul></section>");
    body
}

fn link_list(title: &str, items: Vec<(&str, String)>) -> String {
    if items.is_empty() {
        return String::new();
    }
    let mut body = format!(
        r#"<section class="link-list"><h2>{}</h2><ul>"#,
        escape(title)
    );
    for (label, href) in items {
        body.push_str(&format!(
            r#"<li><a href="{}">{}</a></li>"#,
            escape_attr(&href),
            escape(label)
        ));
    }
    body.push_str("</ul></section>");
    body
}

fn source_list(sources: &[Source], last_verified: NaiveDate) -> String {
    let mut body = format!(
        r#"<section class="sources"><h2>Sources</h2><p class="verified">Last verified: <time datetime="{}">{}</time></p><ol>"#,
        last_verified, last_verified
    );
    for source in sources {
        body.push_str(&format!(
            r#"<li><a href="{}">{}</a>{}</li>"#,
            escape_attr(&source.url),
            escape(&source.title),
            source
                .publisher
                .as_ref()
                .map(|publisher| format!(" <span>{}</span>", escape(publisher)))
                .unwrap_or_default()
        ));
    }
    body.push_str("</ol></section>");
    body
}

fn language_links<'a>(content: &'a SiteContent, slugs: &'a [String]) -> Vec<(&'a str, String)> {
    slugs
        .iter()
        .map(|slug| {
            let title = content
                .language(slug)
                .map(|page| page.data.title.as_str())
                .unwrap_or(slug.as_str());
            (title, format!("/languages/{slug}/"))
        })
        .collect()
}

struct FeedItem {
    title: String,
    summary: String,
    path: String,
    last_verified: NaiveDate,
}

fn feed_items(content: &SiteContent) -> Vec<FeedItem> {
    let mut items = Vec::new();
    items.extend(content.languages.iter().map(|page| FeedItem {
        title: page.data.title.clone(),
        summary: page.data.summary.clone(),
        path: format!("/languages/{}/", page.data.slug),
        last_verified: page.data.last_verified,
    }));
    items.extend(
        content
            .comparisons
            .iter()
            .map(|page: &ComparisonPage| FeedItem {
                title: page.data.title.clone(),
                summary: page.data.summary.clone(),
                path: format!("/comparisons/{}/", page.data.slug),
                last_verified: page.data.last_verified,
            }),
    );
    items.extend(content.guides.iter().map(|page: &GuidePage| FeedItem {
        title: page.data.title.clone(),
        summary: page.data.summary.clone(),
        path: format!("/guides/{}/", page.data.slug),
        last_verified: page.data.last_verified,
    }));
    items.extend(content.concepts.iter().map(|page: &ConceptPage| FeedItem {
        title: page.data.title.clone(),
        summary: page.data.summary.clone(),
        path: format!("/concepts/{}/", page.data.slug),
        last_verified: page.data.last_verified,
    }));
    items.sort_by(|a, b| {
        b.last_verified
            .cmp(&a.last_verified)
            .then(a.title.cmp(&b.title))
    });
    items.truncate(30);
    items
}

fn xml_response(body: String, content_type: &'static str) -> Response {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, HeaderValue::from_static(content_type))],
        body,
    )
        .into_response()
}

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn escape_attr(value: &str) -> String {
    escape(value).replace('\'', "&#39;")
}
