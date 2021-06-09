
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

#[cfg(test)]
mod tests {
    use cichlid::ColorRGB;

    #[test]
    fn test_color_enum() {
        assert_eq!(2 + 2, 4);
    }


    #[test]
    fn test_cichlid() {

        let red = ColorRGB::Red;
        let blue = ColorRGB::Blue;
        let mut purple = red + blue;
        assert_eq!(purple, ColorRGB::new(255, 0, 255));

        purple.scale(128); // Scale by half
        assert_eq!(purple, ColorRGB::new(128, 0, 128));

        purple *= 2;  // Multiple all components by two
        assert_eq!(purple, red + blue);

    }
}
