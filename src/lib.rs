
mod dom_string_renderer;
mod partial_renderer;

pub use partial_renderer::{
    ReadSize,
    DomServerRenderer,
};
pub use dom_string_renderer::{
    render_to_string,
    render_to_static_markup,
};


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
