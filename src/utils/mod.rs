use crate::enums::Glyphs;

/// Common [String] utils that are used to simplify parsing.
pub trait StringUtils {
    fn ltrim(&mut self) -> &mut Self;
    fn rtrim(&mut self) -> &mut Self;
    fn trim(&mut self) -> &mut Self;
    fn substring(&self, start: usize, len: usize) -> Self;
}

impl StringUtils for String {
    /// Returns a copy of [self] with a subset of the contents
    /// starting from [start] to [start+len].
    fn substring(&self, start: usize, len: usize) -> Self {
        self.chars().skip(start).take(len).collect()
    }

    /// While [self] has leading whitespace, those space characters are
    /// consumed and [self] is modified in-place.
    ///
    /// If the first character of [self] is not a whitespace token, then
    /// this is a no-op.
    fn ltrim(&mut self) -> &mut Self {
        let b = self.as_str().bytes().enumerate();

        let mut substr = None;
        for (i, c) in b {
            if c != Glyphs::Space.value() as u8 {
                substr = Some(self.substring(i, self.len() - i));
                break;
            }
        }

        if let Some(s) = substr {
            *self = s
        }

        self
    }

    /// While [self] has trailing whitespace, those space characters are
    /// consumed and [self] is modified in-place.
    ///
    /// If the last character of [self] is not a whitespace token, then
    /// this is a no-op.
    fn rtrim(&mut self) -> &mut Self {
        let b = self.as_str().bytes().enumerate().rev();

        let mut substr = None;
        for (i, c) in b {
            if c != Glyphs::Space.value() as u8 {
                substr = Some(self.substring(0, i + 1));
                break;
            }
        }

        if let Some(s) = substr {
            *self = s
        }

        self
    }

    /// Clears all whitespace surrounding [self], if any.
    /// See [Self::ltrim] and [Self::rtrim].
    fn trim(&mut self) -> &mut Self {
        self.ltrim();
        self.rtrim();

        self
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::StringUtils;

    #[test]
    fn substring() {
        let hw = "Hello, world!";
        let str: String = hw.to_owned();
        assert_eq!(str.substring(7, 5), "world");
    }

    #[test]
    fn trim() {
        let hw = "Hello, world!";
        let mut padded_hw = "   Hello, world!    ".to_owned();
        let mut str = hw.to_owned();
        assert_eq!(str.trim(), hw);
        assert_eq!(padded_hw.trim(), hw);
    }
}
