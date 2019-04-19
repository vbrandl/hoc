use crate::error::Error;
use std::convert::TryFrom;

pub(crate) trait ToCode {
    fn to_code(&self) -> String;
}

#[derive(Debug)]
pub(crate) enum ColorName {
    BrightGreen,
    Green,
    YellowGreen,
    Yellow,
    Orange,
    Red,
    Blue,
    LightGrey,
    Success,
    Important,
    Critical,
    Informational,
    Inactive,
}

impl TryFrom<&str> for ColorName {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "brightgreen" => Ok(ColorName::BrightGreen),
            "green" => Ok(ColorName::Green),
            "yellowgreen" => Ok(ColorName::YellowGreen),
            "yellow" => Ok(ColorName::Yellow),
            "orange" => Ok(ColorName::Orange),
            "red" => Ok(ColorName::Red),
            "blue" => Ok(ColorName::Blue),
            "lightgrey" => Ok(ColorName::LightGrey),
            "success" => Ok(ColorName::Success),
            "important" => Ok(ColorName::Important),
            "critical" => Ok(ColorName::Critical),
            "informational" => Ok(ColorName::Informational),
            "inactive" => Ok(ColorName::Inactive),
            _ => Err(Error::ParseColor),
        }
    }
}

impl ToCode for ColorName {
    fn to_code(&self) -> String {
        use ColorName::*;
        match self {
            BrightGreen | Success => "#44cc11",
            Green => "#97ca00",
            YellowGreen => "#a4a61d",
            Yellow => "#dfb317",
            Orange | Important => "#fe7d37",
            Red | Critical => "#e05d44",
            Blue | Informational => "#007ec6",
            LightGrey | Inactive => "#9f9f9f",
        }
        .to_string()
    }
}

#[derive(Debug)]
pub(crate) struct ColorCode(String);

impl TryFrom<&str> for ColorCode {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let s = if s.starts_with('#') { &s[1..] } else { s };
        let len = s.len();
        if (len == 3 || len == 6) && s.chars().all(|c| c.is_digit(16)) {
            Ok(ColorCode(s.to_lowercase().to_string()))
        } else {
            Err(Error::ParseColor)
        }
    }
}

impl ToCode for ColorCode {
    fn to_code(&self) -> String {
        format!("#{}", self.0)
    }
}

#[derive(Debug)]
pub(crate) enum ColorKind {
    Name(ColorName),
    Code(ColorCode),
}

impl TryFrom<&str> for ColorKind {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        ColorName::try_from(s)
            .map(|c| ColorKind::Name(c))
            .or_else(|_| ColorCode::try_from(s).map(|c| ColorKind::Code(c)))
    }
}

impl ToCode for ColorKind {
    fn to_code(&self) -> String {
        match self {
            ColorKind::Name(name) => name.to_code(),
            ColorKind::Code(code) => code.to_code(),
        }
    }
}

impl Default for ColorKind {
    fn default() -> Self {
        ColorKind::Name(ColorName::Success)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_colorcode() {
        let valid_long = "aaBB11";
        let valid_short = "aB1";
        let pound_valid_long: &str = &format!("#{}", valid_long);
        let pound_valid_short: &str = &format!("#{}", valid_short);
        let valid_long = ColorCode::try_from(valid_long);
        let valid_short = ColorCode::try_from(valid_short);
        let pound_valid_long = ColorCode::try_from(pound_valid_long);
        let pound_valid_short = ColorCode::try_from(pound_valid_short);

        let too_short = "ab";
        let too_long = "aaaaaab";
        let non_hex = "aag";
        let too_short = ColorCode::try_from(too_short);
        let too_long = ColorCode::try_from(too_long);
        let non_hex = ColorCode::try_from(non_hex);

        assert_eq!(&valid_long.unwrap().to_code(), "#aabb11");
        assert_eq!(&valid_short.unwrap().to_code(), "#ab1");
        assert_eq!(&pound_valid_long.unwrap().to_code(), "#aabb11");
        assert_eq!(&pound_valid_short.unwrap().to_code(), "#ab1");

        assert!(too_short.is_err());
        assert!(too_long.is_err());
        assert!(non_hex.is_err());
    }
}
