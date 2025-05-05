use aidoku::{
    error::AidokuError,
    prelude::*,
    std::{net::Request, String},
    Manga, MangaContentRating, MangaPageResult, MangaStatus, MangaViewer, Filter,
};
use scraper::{Html, Selector};
use std::format; // ✅ Disambiguates `format!`

// Fetch HTML from the given URL
fn fetch_url(url: &str) -> Result<String, AidokuError> {
    Request::get(url)
        .header("User-Agent", "Aidoku Plugin")
        .html()
        .map(|html| html.html().read())
        .map_err(AidokuError::from)
}

#[get_manga_list]
fn get_manga_list(_filters: Vec<Filter>, _page: i32) -> Result<MangaPageResult, AidokuError> {
    let url = format!("https://readcomiconline.li/ComicList?page={}", _page);
    let html = fetch_url(&url)?;
    let document = Html::parse_document(&html);
    let manga_elements = Selector::parse("div.manga-list > div.item").unwrap();

    let mut manga_list = Vec::new();

    for element in document.select(&manga_elements) {
        let title_selector = Selector::parse("a").unwrap();
        let img_selector = Selector::parse("img").unwrap();

        let title_elem = element.select(&title_selector).next();
        let img_elem = element.select(&img_selector).next();

        let title = title_elem
            .as_ref()
            .map(|e| e.inner_html())
            .unwrap_or_else(|| String::from("Unknown"));

        let url = title_elem
            .and_then(|e| e.value().attr("href"))
            .unwrap_or("")
            .to_string();

        let cover = img_elem
            .and_then(|e| e.value().attr("src"))
            .unwrap_or("")
            .to_string();

        let manga = Manga {
            id: url.clone(),
            cover,
            title,
            author: String::from("Unknown"),
            artist: String::from("Unknown"),
            description: String::new(),
            url,
            categories: vec![],
            status: MangaStatus::Unknown,
            nsfw: MangaContentRating::Safe,
            viewer: MangaViewer::Ltr,
        };

        manga_list.push(manga);
    }

    Ok(MangaPageResult {
        manga: manga_list.clone(),
        has_more: !manga_list.is_empty(),
    })
}
