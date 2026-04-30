use crate::color::{Rgb, fmt_js_1e10, round_hsl_1e10};

#[derive(Debug, Clone, Copy)]
pub struct Hsl {
    pub h_deg: f64,
    pub s_pct: f64,
    pub l_pct: f64,
}

impl std::fmt::Display for Hsl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "hsl({}, {}%, {}%)",
            fmt_js_1e10(self.h_deg),
            fmt_js_1e10(self.s_pct),
            fmt_js_1e10(self.l_pct)
        )
    }
}

impl From<Rgb> for Hsl {
    fn from(Rgb { r, g, b }: Rgb) -> Self {
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let l = (max + min) / 2.0;

        if max == min {
            return round_hsl_1e10(Hsl {
                h_deg: 0.0,
                s_pct: 0.0,
                l_pct: l * 100.0,
            });
        }

        let d = max - min;
        let s = if l > 0.5 {
            d / (2.0 - max - min)
        } else {
            d / (max + min)
        };
        let mut h = if max == r {
            (g - b) / d + if g < b { 6.0 } else { 0.0 }
        } else if max == g {
            (b - r) / d + 2.0
        } else {
            (r - g) / d + 4.0
        };
        h /= 6.0;

        round_hsl_1e10(Hsl {
            h_deg: h * 360.0,
            s_pct: s * 100.0,
            l_pct: l * 100.0,
        })
    }
}

const HSL_EXPRESSION_PREFIX: &str = "hsl(";
const HSL_EXPRESSION_SUFFIX: &str = ")";

impl TryFrom<&str> for Hsl {
    type Error = crate::error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let Some(inner) = value.trim().strip_prefix(HSL_EXPRESSION_PREFIX) else {
            return Err(Self::Error::InvalidHslExpressionPrefix {
                prefix: HSL_EXPRESSION_PREFIX.to_owned(),
            });
        };
        let Some(inner) = inner.strip_suffix(HSL_EXPRESSION_SUFFIX) else {
            return Err(Self::Error::InvalidHslExpressionSuffix {
                suffix: HSL_EXPRESSION_PREFIX.to_owned(),
            });
        };
        let mut parts = inner.split(',');
        if parts.clone().count() != 3 {
            return Err(Self::Error::InvalidHslExpressionTooMuchValue);
        }
        let h_deg = parts
            .next()
            .unwrap()
            .trim()
            .parse::<f64>()
            .map_err(|_| crate::error::Error::InvalidHslExpressionValue)?;
        let s_pct = parts
            .next()
            .unwrap()
            .trim()
            .parse::<f64>()
            .map_err(|_| crate::error::Error::InvalidHslExpressionValue)?;
        let l_pct = parts
            .next()
            .unwrap()
            .trim()
            .parse::<f64>()
            .map_err(|_| crate::error::Error::InvalidHslExpressionValue)?;
        Ok(Self {
            h_deg,
            s_pct,
            l_pct,
        })
    }
}

impl Hsl {
    pub fn adjust_hsl(mut self, h_delta: f64, s_delta: f64, l_delta: f64) -> Hsl {
        self.h_deg = (self.h_deg + h_delta) % 360.0;
        self.s_pct = (self.s_pct + s_delta).clamp(0.0, 100.0);
        self.l_pct = (self.l_pct + l_delta).clamp(0.0, 100.0);
        round_hsl_1e10(self)
    }
}
