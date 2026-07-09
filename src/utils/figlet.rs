use figlet_rs::FIGlet;

pub fn big_text(text: &str) -> String {
  let font = FIGlet::from_content(include_str!("../../resources/terminus.flf")).unwrap();
  font.convert(text).unwrap().to_string()
}
