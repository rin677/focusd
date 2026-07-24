use ratatui::{Frame, layout::Rect, widgets::Paragraph};

pub struct MergeBlock<'a> {
  title: &'a str,
  top: bool,
  left: bool,
  padding: bool,
  frame: bool,
}

impl<'a> MergeBlock<'a> {
  pub fn new(title: &'a str) -> Self {
    MergeBlock {
      title,
      top: false,
      left: false,
      padding: true,
      frame: false,
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
      let border_w = if self.left {
        area.width + 1
      } else {
        area.width + 2
      };
      let (left_char, right_char) = if self.frame {
        ("├", "┐")
      } else {
        ("├", "┤")
      };
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
    }

    let content_y = area.y + if self.top { 1 } else { 0 };
    let content_height = area.height - if self.top { 1 } else { 0 };
    if self.left && content_height > 0 {
      frame.render_widget(
        Paragraph::new("│"),
        Rect::new(area.x.saturating_sub(1), content_y, 1, content_height),
      );
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
