CREATE TABLE Event 
(
  id SERIAL,
  name VARCHAR(128) NOT NULL,
  promotion VARCHAR(64) NOT NULL,
  date DATE NOT NULL,
  CONSTRAINT PK_Event PRIMARY KEY (id),
  CONSTRAINT UQ_Name_Promotion
    UNIQUE (name, promotion)
);

CREATE TABLE Match 
(
  id SERIAL,
  event_id INTEGER,
  workers TEXT NOT NULL,
  CONSTRAINT PK_Match 
    PRIMARY KEY (id),
  CONSTRAINT FK_Match_Event 
    FOREIGN KEY (event_id) REFERENCES Event(id) 
    ON DELETE CASCADE
);

CREATE TABLE Language (
  code CHAR(3),
  CONSTRAINT PK_Language
    PRIMARY KEY (code)
);

CREATE TABLE Match_Desc
(
  id SERIAL,
  match_id INTEGER NOT NULL,
  description TEXT NOT NULL,
  language_code CHAR(3) NOT NULL,
  CONSTRAINT PK_MatchDesc 
    PRIMARY KEY (id),
  CONSTRAINT UQ_Match_Language
    UNIQUE (match_id, language_code),
  CONSTRAINT FK_Match_MatchDesc
    FOREIGN KEY (match_id) REFERENCES Match(id) ON DELETE CASCADE,
  CONSTRAINT FK_Language_MatchDesc
    FOREIGN KEY (language_code) REFERENCES Language(code) ON DELETE CASCADE
);

CREATE TABLE Rating 
(
  id SERIAL,
  match_id INTEGER NOT NULL,
  language_code CHAR(3) NOT NULL,
  username VARCHAR(32) NOT NULL,
  score NUMERIC(3, 1) NOT NULL,
  opinion TEXT,
  CONSTRAINT PK_Rating
    PRIMARY KEY (id),
  CONSTRAINT FK_Rating_Match
    FOREIGN KEY (match_id) REFERENCES Match(id) 
    ON DELETE CASCADE,
  CONSTRAINT FK_Language_Rating 
    FOREIGN KEY (language_code) REFERENCES Language(code) ON DELETE CASCADE
);
