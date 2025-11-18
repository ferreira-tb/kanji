CREATE TABLE IF NOT EXISTS quiz_chunk_history (
  id INTEGER NOT NULL PRIMARY KEY,
  last_quiz TEXT NOT NULL,
  last_quiz_version TEXT NOT NULL
)
