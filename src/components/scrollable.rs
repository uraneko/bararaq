pub trait ScrollableComponent {
    fn is_scrollable(&self) -> bool;

    fn make_scrollable(&mut self);

    // if you unmake scrollable and parts of the value of the component
    // can not be displayed anymore
    // just log it in the journal
    // and display only up to height limits from start of value
    fn make_unscrollable(&mut self);
}
