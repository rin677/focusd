use ratatui::{
  Frame,
  layout::Rect,
  text::{Line, Text},
  widgets::Paragraph,
};
use std::sync::Mutex;

static BOTTOM_TEES: Mutex<Vec<(u16, u16)>> = Mutex::new(Vec::new());

pub fn clear_bottom_tees() {
  BOTTOM_TEES.lock().unwrap().clear();
}

pub struct MergeBlock<'a> {
  title: &'a str,
  top: bool,
  left: bool,
  padding: bool,
}

impl<'a> MergeBlock<'a> {
  pub fn new(title: &'a str) -> Self {
    MergeBlock {
      title,
      top: false,
      left: false,
      padding: true,
    }
  }

  pub fn top(mut self) -> Self {
    self.top = true;
    self
  }

  pub fn left(mut self) -> Self {
    self.left = true;
    self
  }

  pub fn no_padding(mut self) -> Self {
    self.padding = false;
    self
  }

  pub fn render(&self, frame: &mut Frame, area: Rect) -> Rect {
    if self.top {
      let border_x = area.x.saturating_sub(1);
      let border_w = area.width + 2;
      let left_char = if self.left { "┬" } else { "├" };
      let right_char = "┤";
      let sep = if self.title.is_empty() {
        format!(
          "{}{}{}",
          left_char,
          "─".repeat((border_w as usize).saturating_sub(2)),
          right_char
        )
      } else {
        let fill = (border_w as usize).saturating_sub(4 + self.title.len());
        format!(
          "{}─ {}{}{}",
          left_char,
          self.title,
          "─".repeat(fill),
          right_char
        )
      };
      frame.render_widget(
        Paragraph::new(sep),
        Rect::new(border_x, area.y, border_w, 1),
      );

      let tees = BOTTOM_TEES.lock().unwrap();
      for &(col, row) in tees.iter() {
        if row == area.y {
          let idx = col as i32 - border_x as i32;
          if idx > 0 && idx < border_w as i32 - 1 {
            frame.render_widget(Paragraph::new("┴"), Rect::new(col, area.y, 1, 1));
          }
        }
      }
    }

    let content_y = area.y + if self.top { 1 } else { 0 };
    let content_height = area.height - if self.top { 1 } else { 0 };
    if self.left && content_height > 0 {
      let v: Vec<Line> = (0..content_height).map(|_| Line::from("│")).collect();
      frame.render_widget(
        Paragraph::new(Text::from(v)),
        Rect::new(area.x.saturating_sub(1), content_y, 1, content_height),
      );
      BOTTOM_TEES
        .lock()
        .unwrap()
        .push((area.x.saturating_sub(1), area.y + area.height));
    }

    let pad = if self.padding { 1 } else { 0 };
    Rect::new(
      area.x + pad,
      content_y + pad,
      area.width.saturating_sub(2 * pad),
      content_height.saturating_sub(2 * pad),
    )
  }
}
