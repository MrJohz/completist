pub fn normalise_extension(ext: String) -> String {
    if ext.starts_with(".") {
        ext
    } else {
        format!(".{}", ext)
    }
}
