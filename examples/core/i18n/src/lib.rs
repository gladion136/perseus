mod error_pages;
mod templates;

use perseus::{Html, PerseusApp};

#[perseus::main]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template)
        .template(crate::templates::about::get_template)
        .template(crate::templates::post::get_template)
        .error_pages(crate::error_pages::get_error_pages)
        .locales_and_translations_manager("en-US", &["fr-FR", "es-ES"])
}
