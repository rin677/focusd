pub fn cycle_item<T>(current_item: T, all_items: &[T], offset: isize) -> T
where
  T: PartialEq + Copy,
{
  let len = all_items.len() as isize;
  if len == 0 {
    panic!("all_items cannot be empty");
  }

  let index = all_items
    .iter()
    .position(|f| *f == current_item)
    .unwrap_or(0);
  let next_index = (index as isize + offset).rem_euclid(len) as usize;
  all_items[next_index]
}

pub fn cycle_index(current_index: usize, num_items: usize, offset: isize) -> usize {
  if num_items == 0 {
    panic!("num_items cannot be zero");
  }

  ((current_index as isize + offset).rem_euclid(num_items as isize)) as usize
}
