use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Region {
    /// Global endpoint (auto-redirects to appropriate region)
    #[default]
    Global,
    /// United Arab Emirates
    AE,
    /// Asia-Pacific
    AP,
    /// Australia
    AU,
    /// Canada
    CA,
    /// Germany
    DE,
    /// Europe
    EU,
    /// Europe 2
    EU2,
    /// France
    FR,
    /// Japan
    JP,
    /// United States
    US,
    /// Latin America
    LA,
    /// Russia
    RU,
    /// China
    CN,
}

impl Region {
    /// Get the base API URL for this region
    /// 
    /// Returns a static string reference (no allocation)
    /// 
    /// # Examples
    /// ```
    /// use libre_link_up_api_client::Region;
    /// 
    /// let url = Region::US.base_url();
    /// assert_eq!(url, "https://api-us.libreview.io");
    /// ```
    pub const fn base_url(&self) -> &'static str {
        match self {
            Region::Global => "https://api.libreview.io",
            Region::AE => "https://api-ae.libreview.io",
            Region::AP => "https://api-ap.libreview.io",
            Region::AU => "https://api-au.libreview.io",
            Region::CA => "https://api-ca.libreview.io",
            Region::DE => "https://api-de.libreview.io",
            Region::EU => "https://api-eu.libreview.io",
            Region::EU2 => "https://api-eu2.libreview.io",
            Region::FR => "https://api-fr.libreview.io",
            Region::JP => "https://api-jp.libreview.io",
            Region::US => "https://api-us.libreview.io",
            Region::LA => "https://api-la.libreview.io",
            Region::RU => "https://api.libreview.ru",
            Region::CN => "https://api-cn.myfreestyle.cn",
        }
    }

    /// Convert Region enum to string key (lowercase)
    /// 
    /// # Examples
    /// ```
    /// use libre_link_up_api_client::Region;
    /// 
    /// assert_eq!(Region::US.as_str(), "us");
    /// assert_eq!(Region::EU2.as_str(), "eu2");
    /// ```
    pub const fn as_str(&self) -> &'static str {
        match self {
            Region::Global => "global",
            Region::AE => "ae",
            Region::AP => "ap",
            Region::AU => "au",
            Region::CA => "ca",
            Region::DE => "de",
            Region::EU => "eu",
            Region::EU2 => "eu2",
            Region::FR => "fr",
            Region::JP => "jp",
            Region::US => "us",
            Region::LA => "la",
            Region::RU => "ru",
            Region::CN => "cn",
        }
    }
}

/// Parse a string into a Region (case-insensitive)
/// 
/// Returns `Region::Global` for unrecognized region strings
/// 
/// # Examples
/// ```
/// use std::str::FromStr;
/// use libre_link_up_api_client::Region;
/// 
/// let region = Region::from_str("US").unwrap();
/// assert_eq!(region, Region::US);
/// 
/// let region = Region::from_str("invalid").unwrap();
/// assert_eq!(region, Region::Global);
/// ```
impl FromStr for Region {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let region = match s.to_lowercase().as_str() {
            "us" => Region::US,
            "eu" => Region::EU,
            "eu2" => Region::EU2,
            "fr" => Region::FR,
            "jp" => Region::JP,
            "de" => Region::DE,
            "ap" => Region::AP,
            "au" => Region::AU,
            "ae" => Region::AE,
            "ca" => Region::CA,
            "la" => Region::LA,
            "ru" => Region::RU,
            "cn" => Region::CN,
            _ => Region::Global,
        };
        Ok(region)
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl AsRef<str> for Region {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
