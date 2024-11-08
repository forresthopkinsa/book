use mdbook::{book::Book, errors::Result, preprocess::Preprocessor};

use pulldown_cmark::Event::{self, *};

use crate::config::Mode;

/// A simple preprocessor to rewrite `<figure>`s with `<img>`s.
///
/// This is a no-op by default; it only operates on the book chapters when the
/// `[preprocessor.trpl-figure]` has `output-mode = "simple"`.
///
/// Takes in Markdown containing like this:
///
/// ```markdown
/// <figure>
///
/// <img src="http://www.example.com/some-cool-image.jpg">
///
/// <figcaption>Figure 1-2: A description of the image</figcaption>
///
/// </figure>
/// ```
///
/// Spits out Markdown like this:
///
/// ```markdown
/// <img src="http://www.example.com/some-cool-image.jpg">
///
/// Figure 1-2: A description of the image
/// ```
pub struct TrplFigure;
impl TrplFigure {
    pub fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html" || renderer == "markdown" || renderer == "test"
    }
}

impl Preprocessor for TrplFigure {
    fn name(&self) -> &str {
        "trpl-figure"
    }

    fn run(
        &self,
        ctx: &mdbook::preprocess::PreprocessorContext,
        book: Book,
    ) -> Result<Book> {
        if let Mode::Simple = Mode::from_context(ctx, self.name())? {}
        todo!();
        Ok(book)
    }
}

mod tests;
