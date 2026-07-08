// :TODO:
// - [x] Calculate focus time today
// - [x] Calculate focus time this week
// - [x] Calculate focus time this month
// - [x] Calculate total focus time
// - [ ] Calculate completed sessions
// - [ ] Calculate completion rate
// - [ ] Calculate current streak
// - [ ] Calculate longest streak

use crate::database::history::get_db;

pub enum DurType {
  Today,
  Week,
  Month,
  All,
}

fn get_sql_condition<'a>(dur: DurType) -> &'a str {
  match dur {
    DurType::Today => "WHERE date(end_time) = date('now')",
    DurType::Week => {
      "WHERE date(end_time) >= date('now', 'weekday 1', '-7 days')
       AND date(end_time) <  date('now', 'weekday 1')"
    }
    DurType::Month => "WHERE strftime('%Y-%m', end_time) = strftime('%Y-%m', 'now')",
    DurType::All => "",
  }
}

fn get_total_time(dur: DurType) -> isize {
  let db = match get_db() {
    Ok(db) => db,
    Err(_) => return 0,
  };
  let r#where = get_sql_condition(dur);
  let sql = format!("SELECT SUM(completed_duration) FROM history {where}");
  db.query_row(&sql, [], |row| row.get(0)).unwrap_or(0) / 60
}

pub fn print_stats() {
  println!("Total focused {} mins", get_total_time(DurType::All));
  println!(
    "Total focused (this month) {} mins",
    get_total_time(DurType::Month)
  );
  println!(
    "Total focused (this week) {} mins",
    get_total_time(DurType::Week)
  );
  println!(
    "Total focused (today) {} mins",
    get_total_time(DurType::Today)
  );
}
