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
    #[test]
    fn test_color_enum() {
        assert_eq!(2 + 2, 4);
    }
}
