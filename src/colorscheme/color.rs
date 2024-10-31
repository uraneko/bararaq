#[derive(Default, Debug)]
pub(super) struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub(super) fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub(super) fn text(&self, style: &mut String) {
        let color = format!("38;2;{};{};{};", self.r, self.g, self.b);
        style.push_str(&color)
    }

    pub(super) fn background(&self, style: &mut String) {
        let color = format!("48;2;{};{};{};", self.r, self.g, self.b);
        style.push_str(&color)
    }

    pub(super) fn red(&mut self, r: u8) {
        self.r = r;
    }

    pub(super) fn green(&mut self, g: u8) {
        self.g = g;
    }

    pub(super) fn blue(&mut self, b: u8) {
        self.b = b;
    }

    pub(super) fn array(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

#[cfg(test)]
mod color {
    use super::Color;

    #[test]
    fn color() {
        let color = Color::new(23, 42, 22);

        let mut s = String::new();

        color.text(&mut s);
        assert_eq!(&s[..], "38;2;23;42;22;");
        s.clear();
        color.background(&mut s);
        assert_eq!(&s[..], "48;2;23;42;22;");
    }

    #[test]
    fn atomic() {
        let mut color = Color::new(43, 5, 34);

        color.red(1);
        assert_eq!(color.r, 1);

        color.green(1);
        assert_eq!(color.g, 1);

        color.blue(1);
        assert_eq!(color.b, 1);
    }
}
