import sqlite3
import random
import os
from datetime import datetime, timedelta

DB_PATH = os.path.expanduser("~/.local/share/focusd/db/history-test.db")
os.makedirs(os.path.dirname(DB_PATH), exist_ok=True)

conn = sqlite3.connect(DB_PATH)
conn.execute("""
  CREATE TABLE IF NOT EXISTS history(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    end_time TEXT NOT NULL,
    planned_duration INTEGER NOT NULL,
    completed_duration INTEGER NOT NULL,
    session_type TEXT NOT NULL
  )
""")

random.seed(51)
conn.execute("DELETE FROM history")

now = datetime.now()


def daily_session_count(day: datetime) -> int:
    wd = day.weekday()
    if wd >= 5:
        return random.choices([0, 1, 2, 3], weights=[20, 30, 35, 15])[0]
    return random.choices([2, 3, 4, 5, 6, 7, 8], weights=[5, 10, 20, 25, 20, 15, 5])[0]


def insert_session(dt, planned, completed, stype):
    conn.execute(
        "INSERT INTO history (end_time, planned_duration, completed_duration, session_type) VALUES (?, ?, ?, ?)",
        (dt.strftime("%Y-%m-%d %H:%M:%S"), planned, completed, stype),
    )


total_inserted = 0
for day_offset in range(60, -1, -1):
    day = now - timedelta(days=day_offset)
    sessions = daily_session_count(day)
    if sessions == 0:
        continue

    hour = random.randint(8, 10)
    minute = random.randint(0, 59)

    for s in range(sessions):
        if hour > 22:
            break
        dt = day.replace(hour=hour, minute=minute, second=random.randint(0, 59))

        planned = random.choice([1500, 1800, 2100, 2400, 2700, 3000, 3300, 3600])
        if random.random() < 0.2:
            completed = random.randint(int(planned * 0.3), planned)
        else:
            completed = planned

        insert_session(dt, planned, completed, "Work")
        total_inserted += 1

        hour += random.randint(1, 3)
        minute = random.randint(0, 59)

        if s < sessions - 1 and random.random() < 0.6 and hour <= 22:
            break_dt = day.replace(
                hour=hour, minute=minute, second=random.randint(0, 59)
            )
            insert_session(break_dt, 300, 300, "ShortBreak")
            total_inserted += 1
            hour += random.randint(0, 1)

    if random.random() < 0.15:
        lb_dt = day.replace(hour=random.randint(12, 14), minute=random.randint(0, 59))
        insert_session(lb_dt, 900, 900, "LongBreak")
        total_inserted += 1

conn.commit()
conn.close()

print(f"Inserted {total_inserted} rows into {DB_PATH}")
