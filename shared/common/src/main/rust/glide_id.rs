use regex_macro::regex;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlideID {
    local: i64,
    global: String,
}

impl Display for GlideID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let base36 = radix_fmt::radix_36(self.local);
        write!(f, "{}", base36)?;
        write!(f, ":")?;
        write!(f, "{}", self.global)?;
        Ok(())
    }
}

impl FromStr for GlideID {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let expr = regex!(r"^([a-zA-Z0-9]{1,13}):(\S{1,255})$");
        let s = s.to_lowercase();
        let captures = expr.captures(&s).ok_or(())?;
        let local = i64::from_str_radix(&captures[1], 36).map_err(|_| ())?;
        let global = captures[2].to_string();
        Ok(GlideID { local, global })
    }
}

impl GlideID {
    pub fn new(local: i64, global: String) -> Self {
        GlideID { local, global }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glide_id() {
        let glide_id = GlideID::new(1, "global".to_string());
        assert_eq!(glide_id.to_string(), "1:global");
    }

    #[test]
    fn test_glide_id_from_str() {
        let glide_id = GlideID::from_str("1:global").unwrap();
        assert_eq!(glide_id.local, 1);
        assert_eq!(glide_id.global, "global");
    }
}
