mod error_pages;
mod templates;

use perseus::{Html, PerseusApp, PerseusRoot};

pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template)
        .template(crate::templates::about::get_template)
        .error_pages(crate::error_pages::get_error_pages)
        .index_view(|| {
            sycamore::view! {
                head {}
                body {
                    p { "Test" }
                    PerseusRoot()
                }
            }
        })
}
