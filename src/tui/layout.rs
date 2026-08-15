use ratatui::layout::Rect;

pub fn split_vertical(area: Rect) -> (Rect, Rect) {
  let w = area.width;
  let mid = w.saturating_sub(1) / 2;
  let left = Rect::new(area.x, area.y, mid + 1, area.height);
  let right = Rect::new(area.x + mid, area.y, w - mid, area.height);
  (left, right)
}
