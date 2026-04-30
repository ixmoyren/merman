use crate::color::{Hsl, fmt_js_1e10, hue_to_rgb, round_1e10};

#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl std::fmt::Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = (self.r.clamp(0.0, 1.0) * 255.0).round() as i64;
        let g = (self.g.clamp(0.0, 1.0) * 255.0).round() as i64;
        let b = (self.b.clamp(0.0, 1.0) * 255.0).round() as i64;
        write!(
            f,
            "#{:02x}{:02x}{:02x}",
            r.clamp(0, 255),
            g.clamp(0, 255),
            b.clamp(0, 255)
        )
    }
}

impl From<Hsl> for Rgb {
    fn from(
        Hsl {
            h_deg,
            s_pct,
            l_pct,
        }: Hsl,
    ) -> Self {
        let h = (h_deg / 360.0) % 1.0;
        let s = (s_pct / 100.0).clamp(0.0, 1.0);
        let l = (l_pct / 100.0).clamp(0.0, 1.0);

        if s == 0.0 {
            return Rgb { r: l, g: l, b: l };
        }

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;
        Rgb {
            r: hue_to_rgb(p, q, h + 1.0 / 3.0),
            g: hue_to_rgb(p, q, h),
            b: hue_to_rgb(p, q, h - 1.0 / 3.0),
        }
    }
}

impl TryFrom<&str> for Rgb {
    type Error = crate::error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let Some(hex) = value.trim().strip_prefix("#") else {
            return Err(Self::Error::InvalidRGBFormat);
        };
        let (r, g, b) = match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)
                    .map_err(|_| Self::Error::InvalidRGBValue)?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)
                    .map_err(|_| Self::Error::InvalidRGBValue)?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)
                    .map_err(|_| Self::Error::InvalidRGBValue)?;
                (r, g, b)
            }
            6 => {
                let r =
                    u8::from_str_radix(&hex[0..2], 16).map_err(|_| Self::Error::InvalidRGBValue)?;
                let g =
                    u8::from_str_radix(&hex[2..4], 16).map_err(|_| Self::Error::InvalidRGBValue)?;
                let b =
                    u8::from_str_radix(&hex[4..6], 16).map_err(|_| Self::Error::InvalidRGBValue)?;
                (r, g, b)
            }
            _ => return Err(Self::Error::InvalidRGBFormat),
        };
        Ok(Rgb {
            r: (r as f64) / 255.0,
            g: (g as f64) / 255.0,
            b: (b as f64) / 255.0,
        })
    }
}

impl TryFrom<String> for Rgb {
    type Error = crate::error::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl TryFrom<&String> for Rgb {
    type Error = crate::error::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl Rgb {
    pub fn invert_rgb_to_rgb_string(&self) -> String {
        let r = round_1e10((1.0 - self.r) * 255.0);
        let g = round_1e10((1.0 - self.g) * 255.0);
        let b = round_1e10((1.0 - self.b) * 255.0);
        format!(
            "rgb({}, {}, {})",
            fmt_js_1e10(r),
            fmt_js_1e10(g),
            fmt_js_1e10(b)
        )
    }

    pub fn parse_like(s: &str, prefix: &str) -> Result<Self, crate::error::Error> {
        let Some(inner) = s.trim().strip_prefix(prefix) else {
            return Err(crate::error::Error::InvalidRGBExpressionPrefix {
                prefix: prefix.to_string(),
            });
        };
        let Some(inner) = inner.strip_suffix(')') else {
            return Err(crate::error::Error::InvalidRGBExpressionSuffix {
                suffix: ")".to_string(),
            });
        };
        let mut values = inner.split(',');
        if values.clone().count() != 3 {
            return Err(crate::error::Error::InvalidRGBExpressionTooMuchValue);
        }
        let r = values
            .next()
            .unwrap()
            .trim()
            .parse::<f64>()
            .map_err(|_| crate::error::Error::InvalidRGBExpressionValue)?;
        let g = values
            .next()
            .unwrap()
            .trim()
            .parse::<f64>()
            .map_err(|_| crate::error::Error::InvalidRGBExpressionValue)?;
        let b = values
            .next()
            .unwrap()
            .trim()
            .parse::<f64>()
            .map_err(|_| crate::error::Error::InvalidRGBExpressionValue)?;
        Ok(Self { r, g, b })
    }

    pub fn to_rgb_expr(&self) -> String {
        format!("rgb({}, {}, {})", self.r, self.g, self.b)
    }

    pub fn to_u8_expr(&self) -> String {
        let r = (self.r * 255.0).round() as u8;
        let g = (self.g * 255.0).round() as u8;
        let b = (self.b * 255.0).round() as u8;
        format!("rgb({r}, {g}, {b})")
    }
}
