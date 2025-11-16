CREATE TABLE IF NOT EXISTS source_group_source (
  source_group_id INTEGER NOT NULL,
  source_id INTEGER NOT NULL,
  PRIMARY KEY (source_group_id, source_id),
  FOREIGN KEY (source_group_id) REFERENCES source_group (id) ON DELETE CASCADE ON UPDATE CASCADE,
  FOREIGN KEY (source_id) REFERENCES source (id) ON DELETE CASCADE ON UPDATE CASCADE
);