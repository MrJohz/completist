pub fn normalise_extension(ext: String) -> String {
    if ext.starts_with(".") {
        ext
    } else {
        format!(".{}", ext)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalise_extension() {
        assert_eq!(normalise_extension("hello".to_string()), ".hello".to_string());
        assert_eq!(normalise_extension(".hello".to_string()), ".hello".to_string());
        assert_eq!(normalise_extension("he.llo".to_string()), ".he.llo".to_string());
        assert_eq!(normalise_extension(".he.llo".to_string()), ".he.llo".to_string());
        assert_eq!(normalise_extension("he.llo.".to_string()), ".he.llo.".to_string());
        assert_eq!(normalise_extension(".he.llo.".to_string()), ".he.llo.".to_string());
    }
}
