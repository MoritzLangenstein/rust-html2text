//! Module containing the `Renderer` interface for constructing a
//! particular text output.

pub mod raw_renderer;
pub mod text_renderer;

/// A type which is a backend for HTML to text rendering.
pub trait Renderer {
    /// Add an empty line to the output (ie between blocks).
    fn add_empty_line(&mut self);

    /// Create a sub-renderer for nested blocks.
    fn new_sub_renderer(&self, width: usize) -> Self;

    /// Start a new block.
    fn start_block(&mut self);

    /// Mark the end of a block.
    fn end_block(&mut self);

    /// Start a new line, if necessary (but don't add a new line).
    fn new_line(&mut self);

    /// Start a new line.
    fn new_line_hard(&mut self);

    /// Add a horizontal table border.
    fn add_horizontal_border(&mut self);

    /// Begin a preformatted block.  Until the corresponding end,
    /// whitespace will used verbatim.  Pre regions can nest.
    fn start_pre(&mut self);

    /// Finish a preformatted block started with `start_pre`.
    fn end_pre(&mut self);

    /// Add a new block of preformatted text.
    fn add_preformatted_block(&mut self, text: &str);

    /// Add some inline text (which should be wrapped at the
    /// appropriate width) to the current block.
    fn add_inline_text(&mut self, text: &str);

    /// Return the current width in character cells
    fn width(&self) -> usize;

    /// Add a line to the current block without starting a new one.
    fn add_block_line(&mut self, line: &str);

    /// Add a new block from a sub renderer, and prefix every line by the
    /// corresponding text from each iteration of prefixes.
    fn append_subrender<'a, I>(&mut self, other: Self, prefixes: I)
    where
        I: Iterator<Item = &'a str>;

    /// Append a set of sub renderers joined left-to-right with a vertical line,
    /// and add a horizontal line below.
    /// If collapse is true, then merge top/bottom borders of the subrenderer
    /// with the surrounding one.
    fn append_columns_with_borders<I>(&mut self, cols: I, collapse: bool)
    where
        I: IntoIterator<Item = Self>,
        Self: Sized;

    /// Returns true if this renderer has no content.
    fn empty(&self) -> bool;

    /// Return the length of the contained text.
    fn text_len(&self) -> usize;

    /// Start a hyperlink
    /// TODO: return sub-builder or similar to make misuse
    /// of start/link harder?
    fn start_link(&mut self, target: &str);

    /// Finish a hyperlink started earlier.
    fn end_link(&mut self);

    /// Start an emphasised region
    fn start_emphasis(&mut self);

    /// Finish emphasised text started earlier.
    fn end_emphasis(&mut self);

    /// Start an strong region
    fn start_strong(&mut self);

    /// Finish strong text started earlier.
    fn end_strong(&mut self);

    /// Start a code region
    fn start_code(&mut self);

    /// End a code region
    fn end_code(&mut self);

    /// Add an image
    fn add_image(&mut self, title: &str);

    /// Record the start of a named HTML fragment
    fn record_frag_start(&mut self, fragname: &str);
}
