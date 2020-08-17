//! Implementations of the `Renderer` trait.
//!
//! This module implements helpers and concrete types for rendering from HTML
//! into a raw text format without any formatting.

use super::Renderer;

/// A renderer which just outputs plain raw text without annotations.
pub struct RawRenderer {
    buffer: String,
}

impl RawRenderer {
    /// Construct a new empty RawRenderer.
    pub fn new() -> Self {
        RawRenderer {
            buffer: String::new(),
        }
    }

    /// Consumes this renderer and return a multiline `String` with the result.
    pub fn into_string(self) -> String {
        let mut output = String::with_capacity(self.buffer.len());

        let mut whitespace = false;

        for c in self.buffer.chars() {
            if c.is_whitespace() {
                if !whitespace {
                    whitespace = true;
                    output.push(' ');
                }
            } else {
                whitespace = false;
                output.push(c);
            }
        }

        output
    }
}

impl Renderer for RawRenderer {
    /// Add an empty line to the output (ie between blocks).
    fn add_empty_line(&mut self) {
        self.new_line_hard()
    }

    /// Create a sub-renderer for nested blocks.
    fn new_sub_renderer(&self, _width: usize) -> Self {
        RawRenderer::new()
    }

    /// Start a new block.
    fn start_block(&mut self) {
        // no-op
    }

    /// Mark the end of a block.
    fn end_block(&mut self) {
        // no-op
    }

    /// Start a new line, if necessary (but don't add a new line).
    fn new_line(&mut self) {
        // no-op
    }

    /// Start a new line.
    fn new_line_hard(&mut self) {
        self.buffer.push(' ')
    }

    /// Add a horizontal table border.
    fn add_horizontal_border(&mut self) {
        // no-op
    }

    /// Begin a preformatted block.  Until the corresponding end,
    /// whitespace will used verbatim.  Pre regions can nest.
    fn start_pre(&mut self) {
        // no-op
    }

    /// Finish a preformatted block started with `start_pre`.
    fn end_pre(&mut self) {
        // no-op
    }

    /// Add a new block of preformatted text.
    fn add_preformatted_block(&mut self, text: &str) {
        self.add_inline_text(text);
        self.new_line_hard()
    }

    /// Add some inline text (which should be wrapped at the
    /// appropriate width) to the current block.
    fn add_inline_text(&mut self, text: &str) {
        self.buffer.push_str(text)
    }

    /// Return the current width in character cells
    fn width(&self) -> usize {
        80
    }

    /// Add a line to the current block without starting a new one.
    fn add_block_line(&mut self, line: &str) {
        self.add_inline_text(line);
        self.new_line_hard()
    }

    /// Add a new block from a sub renderer, and prefix every line by the
    /// corresponding text from each iteration of prefixes.
    fn append_subrender<'a, I>(&mut self, other: Self, _prefixes: I)
    where
        I: Iterator<Item = &'a str>,
    {
        self.buffer.push_str(&other.buffer);
        self.new_line_hard()
    }

    /// Append a set of sub renderers joined left-to-right with a vertical line,
    /// and add a horizontal line below.
    /// If collapse is true, then merge top/bottom borders of the subrenderer
    /// with the surrounding one.
    fn append_columns_with_borders<I>(&mut self, cols: I, _collapse: bool)
    where
        I: IntoIterator<Item = Self>,
        Self: Sized,
    {
        cols.into_iter().for_each(|r| {
            self.buffer.push_str(&r.buffer);
            self.new_line_hard()
        })
    }

    /// Returns true if this renderer has no content.
    fn empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Return the length of the contained text.
    fn text_len(&self) -> usize {
        self.buffer.len()
    }

    /// Start a hyperlink
    fn start_link(&mut self, _target: &str) {
        // no-op
    }

    /// Finish a hyperlink started earlier.
    fn end_link(&mut self) {
        // no-op
    }

    /// Start an emphasised region
    fn start_emphasis(&mut self) {
        // no-op
    }

    /// Finish emphasised text started earlier.
    fn end_emphasis(&mut self) {
        // no-op
    }

    /// Start an strong region
    fn start_strong(&mut self) {
        // no-op
    }

    /// Finish strong text started earlier.
    fn end_strong(&mut self) {
        // no-op
    }

    /// Start a code region
    fn start_code(&mut self) {
        // no-op
    }

    /// End a code region
    fn end_code(&mut self) {
        // no-op
    }

    /// Add an image
    fn add_image(&mut self, title: &str) {
        self.add_inline_text(title);
        self.new_line_hard()
    }

    /// Record the start of a named HTML fragment
    fn record_frag_start(&mut self, _fragname: &str) {
        // no-op
    }
}
