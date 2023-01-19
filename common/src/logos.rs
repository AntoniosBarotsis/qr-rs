const GOOGLE_LOGO: &[u8] = include_bytes!("../../assets/logo.png");

#[derive(Debug)]
pub struct InvalidLogo(pub String);

#[derive(Debug, Clone, Copy)]
pub enum Logo {
  Google,
}

impl TryFrom<Option<String>> for Logo {
  type Error = InvalidLogo;

  fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
    Self::try_from(value.unwrap_or_else(|| "google".to_string()))
  }
}

impl TryFrom<&String> for Logo {
  type Error = InvalidLogo;

  fn try_from(value: &String) -> Result<Self, Self::Error> {
    match value.trim().to_lowercase().as_str() {
      "google" => Ok(Self::Google),
      e => Err(InvalidLogo(e.to_owned())),
    }
  }
}

impl TryFrom<String> for Logo {
  type Error = InvalidLogo;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    match value.trim().to_lowercase().as_str() {
      "google" => Ok(Self::Google),
      e => Err(InvalidLogo(e.to_owned())),
    }
  }
}

impl From<Logo> for Vec<u8> {
  fn from(value: Logo) -> Self {
    match value {
      Logo::Google => GOOGLE_LOGO.to_vec(),
    }
  }
}
