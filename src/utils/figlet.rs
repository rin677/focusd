use crate::config::settings::{Fonts, get_config};
use figlet_rs::{FIGlet, Toilet};

pub enum Font {
  Figlet(FIGlet),
  Toilet(Toilet),
}

impl Font {
  pub fn convert(&self, text: &str) -> String {
    match self {
      Font::Figlet(font) => font.convert(text).unwrap().to_string(),
      Font::Toilet(font) => font.convert(text).unwrap().to_string(),
    }
  }
}

fn get_font(font: Fonts) -> Font {
  let make_toilet = |f: Result<Toilet, _>| Font::Toilet(f.unwrap());

  match font {
    Fonts::Terminus => {
      Font::Figlet(FIGlet::from_content(include_str!("../../resources/terminus.flf")).unwrap())
    }
    Fonts::SmBlock => make_toilet(Toilet::smblock()),
    Fonts::Future => make_toilet(Toilet::future()),
    Fonts::Mono9 => make_toilet(Toilet::mono9()),
  }
}

pub fn big_text(text: &str) -> String {
  let font = get_font(get_config().font);
  font.convert(text)
}
