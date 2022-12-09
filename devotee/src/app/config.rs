/// An application configuration.
pub trait Config {
    /// The root node to handle App's events.
    type Node;
    /// The palette used to render on the main canvas.
    type Palette;
    /// The converter to transform pallette values into `[u8; 4]` values.
    type Converter;
    /// The input handler.
    type Input;

    /// Provide palette converter.
    fn converter() -> Self::Converter;
    /// Provide default background color for the canvas.
    fn background_color() -> Self::Palette;
    /// Provide default background color for the window.
    fn window_background_color() -> [u8; 3];
}
