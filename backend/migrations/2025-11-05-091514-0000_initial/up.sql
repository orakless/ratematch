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
  event_id INTEGER NOT NULL,
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
    FOREIGN KEY (language_code) REFERENCES Language(code) ON DELETE CASCADE,
  CONSTRAINT CHK_Rating CHECK (score >= 0 AND score <= 5)
);

INSERT INTO Language (code) VALUES ('FRE'), ('ENG');

DO $$
DECLARE 
  event_id INTEGER;
  match_id INTEGER;
BEGIN
  INSERT INTO Event (name, promotion, date)
  VALUES 
    ('AEW Double or Nothing 2025', 'All Elite Wrestling','2025-05-25')
  RETURNING id INTO event_id;

  INSERT INTO Match (event_id, workers) VALUES 
    (event_id, 'Mercedes Moné vs. Jamie Hayter')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'Non Title Owen Hart Foundation 2025 Women''s Tournament Final Match', 'ENG'),
    (match_id, 'Finale du tournoi féminin 2025 de la Fondation Owen Hart sans chance pour le titre', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'FTR vs. Daniel Garcia, Nigel McGinness')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'Tag Team Match', 'ENG'),
    (match_id, 'Match en équipe de deux', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Ricochet vs. Mark Briscoe')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'Stretcher Match', 'ENG'),
    (match_id, 'Match du brancard', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'The Hurt Syndicate vs. The Sons Of Texas')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'AEW Men''s World Tag Team Title Match', 'ENG'),
    (match_id, 'Match pour le titre de champion du monde en équipe AEW masculin', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Kazuchika Okada vs. Mike Bailey')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'AEW Continental Title Match', 'ENG'),
    (match_id, 'Match pour le titre de champion continental AEW', 'FRE');

  INSERT INTO Match (event_id,  workers) VALUES
    (event_id, 'Toni Storm vs. Mina Shirakawa')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'AEW Women''s World Title Match', 'ENG'),
    (match_id, 'Match pour le titre de championne du monde', 'FRE');

  INSERT INTO Match (event_id,  workers) VALUES
    (event_id, 'Kenny Omega, Swerve Strickland, The Opps, Willow Nightingale vs. Death Riders, The Young Bucks')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'Anarchy in the Arena Match', 'ENG'),
    (match_id, 'Match Anarchie dans l''Arène', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'The Don Callis Family vs. Paragon')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'Six Man Tag Team Match', 'ENG'),
    (match_id, 'Match à six personnes en équipes de trois', 'FRE');

  INSERT INTO Match (event_id,  workers) VALUES
    (event_id, '«Hangman» Adam Page vs. Will Ospreay')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'Owen Hart Foundation 2025 Men''s Tournament Final Match', 'ENG'),
    (match_id, 'Finale du tournoi masculin 2025 de la Fondation Owen Hart', 'FRE');
END $$;

DO $$
DECLARE 
  event_id INTEGER;
  match_id INTEGER;
BEGIN
  INSERT INTO Event (name, promotion, date)
  VALUES 
      ('AEW All In 2025', 'All Elite Wrestling', '2025-07-12')
    RETURNING id INTO event_id;

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'The Opps vs. Death Riders, Gabe Kidd')
  RETURNING id INTO match_id;
 
  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'AEW Men''s World Trios Title Match', 'ENG'),
    (match_id, 'Match pour le titre de champion du monde masculin AEW en trio', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'MJF vs. Anthony Bowens vs. Bandido vs. Brody King vs. Josh Alexander vs. Juice Robinson vs. Konosuke Takeshita vs. Kota Ibushi vs. Mark Briscoe vs. Max Caster vs. Mistico vs. Ricochet vs. Roderick Strong vs. The Beast Mortos')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'Casino Gauntlet Match', 'ENG'),
    (match_id, 'Match "Épreuve du Casino"', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Dustin Rhodes vs. Daniel Garcia vs. Kyle Fletcher vs. Sammy Guevara')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'AEW TNT Title Four Way Match', 'ENG'),
    (match_id, 'Match à 4 pour le titre de champion AEW TNT', 'FRE');
  
  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Swerve Strickland, Will Ospreay vs. The Young Bucks')
  RETURNING id INTO match_id;
  
  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'Tag Team Match', 'ENG'),
    (match_id, 'Match en équipe de deux', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Athena vs. Alex Windsor vs. Julia Hart vs. Kris Statlander vs. Megan Bayne vs. Mina Shirakawa vs. Queen Aminata vs. Syuri vs. Tay Melo vs. Thekla vs. Thunder Rosa vs. Willow Nightingale')
  RETURNING id INTO match_id;
  
  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'Casino Gauntlet Match', 'ENG'),
    (match_id, 'Match "Épreuve du Casino"', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'The Hurt Syndicate vs. JetSpeed')
  RETURNING id INTO match_id;
 
  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'AEW Men''s World Tag Team Title Match', 'ENG'),
    (match_id, 'Match pour le titre de champion du monde masculin AEW en duo', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id,  'Toni Storm vs. Mercedes Moné')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'AEW Women''s World Title Match', 'ENG'),
    (match_id, 'Match pour le titre de championne du monde AEW', 'FRE');
  
  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Kazuchika Okada vs. Kenny Omega')
  RETURNING id INTO match_id;
 
  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'AEW Unified Title / AEW Continental Title / AEW International Title Winner Takes All Match', 'ENG'),
    (match_id, 'Match pour les titres de champion unifié, continental et international AEW', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, '«Hangman» Adam Page vs. Jon Moxley')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'AEW Men''s World Title Texas Death Match', 'ENG'),
    (match_id, 'Match à mort du Texas pour le titre de champion du monde AEW', 'FRE');

END $$;

DO $$
DECLARE 
  event_id INTEGER;
  match_id INTEGER;
BEGIN
  INSERT INTO Event (name, promotion, date)
  VALUES 
    ('AEW Forbidden Door 2025', 'All Elite Wrestling', '2025-08-24')
  RETURNING id INTO event_id;
  
  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Adam Copeland, Christian Cage vs. Killswitch, Kip Sabian')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'Tag Team Match', 'ENG'),
    (match_id, 'Match en équipes de 2', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Kyle Fletcher vs. Hiromu Takahashi')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'AEW TNT Title Match', 'ENG'),
    (match_id, 'Match pour le titre de champion AEW TNT', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Mercedes Moné vs. Alex Windsor vs. Bozilla vs. Persephone')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc(match_id, description, language_code) VALUES 
    (match_id, 'AEW TBS Title Four Way Match', 'ENG'),
    (match_id, 'Match à 4 pour le titre de championne AEW TBS', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Zack Sabre Jr. vs. Nigel McGuinness')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'IWGP World Heavyweight Title Match', 'ENG'),
    (match_id, 'Match pour le titre de champion du monde poids-lourd IWGP', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Brodido vs. The Hurt Syndicate vs. FTR')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'AEW Men''s World Tag Team Title Three Way Match', 'ENG' ),
    (match_id, 'Match à 3 équipes de 3 pour le titre de champion du monde masculin AEW en trio', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Kazuchika Okada vs. Swerve Strickland')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'AEW Unified Title / AEW Continental Title / AEW International Title Match', 'ENG'),
    (match_id, 'Match pour les titres de champion unifié, continental et international AEW', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Toni Storm vs. Athena')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'AEW Women''s World Title Match', 'ENG'),
    (match_id, 'Match pour le titre de championne de monde AEW', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, '«Hangman» Adam Page vs. MJF')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'AEW Men''s World Title', 'ENG'),
    (match_id, 'Match pour le titre de champion du monde AEW', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Darby Allin, Golden Lovers, Hiroshi Tanahashi, Will Ospreay vs. Death Riders, The Young Bucks, Gabe Kidd')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'Lights Out Steel Cage Match', 'ENG'),
    (match_id, 'Match en cage sans règles hors programme', 'FRE');
END $$;
    
DO $$
DECLARE 
  event_id INTEGER;
  match_id INTEGER;
BEGIN
  INSERT INTO Event (name, promotion, date)
  VALUES 
    ('WWE Wrestlemania 41 - Night 1', 'World Wrestling Entertainment', '2025-04-19')
  RETURNING id INTO event_id;

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Jey Uso vs. Gunther')
  RETURNING id INTO match_id;
 
  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'WWE Men''s World Heavyweight Title Match', 'ENG'),
    (match_id, 'Match pour le titre de champion du monde poids-lourd WWE', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'The New Day vs. The War Raiders')
  RETURNING id INTO match_id;
  
  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'WWE Men''s World Tag Team Title Match', 'ENG'),
    (match_id, 'Match pour le titre de champions du monde WWE en duo', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Jade Cargill vs. Naomi')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'Singles Match', 'ENG'),
    (match_id, 'Match simple', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Jacob Fatu vs. LA Knight')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'WWE Men''s United States Title Match', 'ENG'),
    (match_id, 'Match pour le titre de champion des États-Unis d''Amérique WWE', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'El Grande Americano vs. Rey Fenix')
  RETURNING id INTO match_id;
  
  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'Singles Match', 'ENG'),
    (match_id, 'Match simple', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Tiffany Stratton vs. Charlotte Flair')
  RETURNING id INTO match_id;
  
  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'WWE Women''s Title', 'ENG'),
    (match_id, 'Match pour le titre de championne WWE', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Seth Rollins vs. CM Punk vs. Roman Reigns')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'Triple Threat Match', 'ENG'),
    (match_id, 'Match Triple Menace', 'FRE');
  
END $$;
    
DO $$
DECLARE 
  event_id INTEGER;
  match_id INTEGER;
BEGIN
  INSERT INTO Event (name, promotion, date)
  VALUES 
    ('WWE Wrestlemania 41 - Night 2', 'World Wrestling Entertainment', '2025-04-20')
  RETURNING id INTO event_id;
  
  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'IYO SKY vs. Bianca Belair vs. Rhea Ripley')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'WWE Women''s World Title Triple Threat Match', 'ENG'),
    (match_id, 'Match Triple Menace pour le titre de championne du monde WWE', 'FRE');
   
  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Drew McIntyre vs. Damian Priest')
  RETURNING id INTO match_id;
    
  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'Sin City Street Fight', 'ENG'),
    (match_id, 'Bagarre de rue de la Cité des Pêchés', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Dominik Mysterio vs. Bron Breakker vs. Finn Balor vs. Penta')
  RETURNING id INTO match_id;
     
  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'WWE Men''s Intercontinental Title Match', 'ENG'),
    (match_id, 'Match pour le titre de champion intercontinental WWE', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Randy Orton vs. Joe Hendry')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'Singles Match', 'ENG'),
    (match_id, 'Match simple', 'FRE');
   
  INSERT INTO Match (event_id, workers) VALUES
     (event_id, 'Logan Paul vs. AJ Styles')
  RETURNING id INTO match_id;
 
  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'Singles Match', 'ENG'),
    (match_id, 'Match simple', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Becky Lynch, Lyra Valkyria vs. The Judgment Day')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'WWE Women''s Tag Team Title Match', 'ENG'),
    (match_id, 'Match pour le titre de championnes du monde WWE en duo', 'FRE');
     
  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'John Cena vs. Cody Rhodes')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'Undisputed WWE Men''s Title Match', 'ENG'),
    (match_id, 'Match pour le titre de champion WWE indisputé', 'FRE');
END $$;
    
DO $$
DECLARE 
  event_id INTEGER;
  match_id INTEGER;
BEGIN
  INSERT INTO Event (name, promotion, date) VALUES 
    ('Stardom All-Star Grand Queendom 2025', 'World Wonder Ring Stardom', '2025-04-27')
  RETURNING id INTO event_id;
  
  INSERT INTO Match (event_id, workers) VALUES 
    (event_id, 'HANAKO vs. Akira Kurogane vs. Azusa Inaba vs. Fukigen Death vs. Kiyoka Kotatsu vs. Lady C vs. Miyu Amasaki vs. Momo Kohgo vs. Momo Watanabe vs. Rian vs. Ruaka vs. Tomoka Inaba vs. Waka Tsukiyama vs. Yuna Mizumori vs. Yuria Hime')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'Pre-Show Rumble Match', 'ENG'),
    (match_id, 'Bataille d''avant-spectacle ', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Hina vs. Ranna Yagami')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'Future Of Stardom Title Match', 'ENG'),
    (match_id, 'Match pour le titre du Futur de Stardom', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
      (event_id, 'Sayaka Kurara vs. Thekla')
  RETURNING id INTO match_id;
 
  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'Singles Match', 'ENG'),
    (match_id, 'Match simple', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
      (event_id, 'Yumiko Hotta vs. Rina')
  RETURNING id INTO match_id;
 
  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'Singles Match', 'ENG'),
    (match_id, 'Match simple', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
      (event_id, 'Meiko Satomura, Mika Iwata, YUNA vs. Cosmic Angels')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'Six Man Tag Team Match', 'ENG'),
    (match_id, 'Match en équipes de 3', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
      (event_id, 'Suzu Suzuki vs. Mei Seira')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'No Disqualification Match', 'ENG'),
    (match_id, 'Match sans disqualification', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
      (event_id, 'Chihiro Hashimoto vs. Maika')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'Sendai Girls World Title Match', 'ENG'),
    (match_id, 'Match pour le titre de championne du monde Sendai Girls', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
      (event_id, 'STARS vs. STARS')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'Goddesses Of Stardom Title Match', 'ENG'),
    (match_id, 'Match pour le titre de Déesses de Stardom', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
      (event_id, 'Syuri vs. Mayu Iwatani')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'IWGP Women''s Title Match', 'ENG'),
    (match_id, 'Match pour le titre de championne IWGP', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES 
      (event_id, 'Starlight Kid vs. AZM')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'Wonder Of Stardom Title Match', 'ENG'),
    (match_id, 'Match pour le titre de Merveille de Stardom', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
      (event_id, 'Saya Kamitani vs. Tam Nakano')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'World of Stardom Title Career Vs. Career Match', 'ENG'),
    (match_id, 'Match Carrière contre Carrière pour le titre de championne du monde de Stardom', 'FRE');

END $$;

DO $$
DECLARE 
  event_id INTEGER;
  match_id INTEGER;
BEGIN
  INSERT INTO Event (name, promotion, date) VALUES 
    ('NJPW Wrestle Kingdom 19 In Tokyo Dome', 'New Japan Pro Wrestling', '2025-01-04')
  RETURNING id INTO event_id;

  INSERT INTO Match (event_id, workers) VALUES 
    (event_id, 'Hirooki Goto vs. Alex Zayne vs. Great-O-Khan vs. Hiroyoshi Tenzan vs. Jsh Barnett vs. KENTA vs. Oleg Boltin vs. SANADA vs. Satoshi Kojima vs. Taichi vs. Togi Makabe vs. Tomoaki Honma vs. Tomohiro Ishii vs. Toru Yano vs. YOSHI-HASHI vs. Yuji Nagata vs. Yujiro Takahashi')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'Pre-Show IWGP Men''s World Heavyweight Title #1 Contendership New Japan Rambo Match', 'ENG'),
    (match_id, 'Bataille royale ''New Japan Rambo'' d''avant-spectacle pour une chance pour le titre de champion IWGP', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Ichiban Sweet Boys vs. Intergalactic Jet Setters vs. BULLET CLUB War Dogs vs. Catch 22')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'IWGP Men''s Junior Heavyweight Tag Team Title Ladder Match', 'ENG'),
    (match_id, 'Match à échelles pour le titre de champion IWGP junior en équipe', 'FRE');
 
  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Mayu Iwatani vs. AZM')
  RETURNING id INTO match_id;
 
  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'IWGP Women''s Title Match', 'ENG'),
    (match_id, 'Match pour le titre de championne IWGP', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'El Phantasmo vs. Ren Narita vs. Jeff Cobb vs. Ryohei Oiwa')
  RETURNING id INTO match_id;
 
  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'NJPW World Television Title Four Way Match', 'ENG'),
    (match_id, 'Match à 4 pour le titre de champion IWGP télévision', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Hiroshi Tanahashi vs. EVIL')
  RETURNING id INTO match_id;
 
  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'Lumberjack Match', 'ENG'),
    (match_id, 'Match de bûcheron', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Konosuke Takeshita vs. Shingo Takagi')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'AEW International Title / NEVER Openweight Title Match', 'ENG'),
    (match_id, 'Match pour les titres de champion international AEW et NEVER Openweight', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'El Desperado vs. DOUKI')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'IWGP Men''s Junior Heavyweight Title Match', 'ENG'),
    (match_id, 'Match pour le titre de champion poids-lourd junior IWGP', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Yota Tsuji vs. David Finlay')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'IWGP Men''s Global Heavyweight Title Match', 'ENG'),
    (match_id, 'Match pour le titre de champion poids-lourd global IWGP', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Tetsuya Naito vs. Hiromu Takahashi')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'Singles Match', 'ENG'),
    (match_id, 'Match simple', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Zack Sabre Jr. vs. Shota Umino')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'IWGP Men''s World Heavyweight Title Match', 'ENG'),
    (match_id, 'Match pour le titre de champion du monde poids-lourd IWGP', 'FRE');
  
END $$;

DO $$
DECLARE 
  event_id INTEGER;
  match_id INTEGER;
BEGIN
  INSERT INTO Event (name, promotion, date)
  VALUES 
    ('APC×BZW French Touch', 'Association les Professionnels du Catch, Banger Zone Wrestling', '2025-08-30')
  RETURNING id INTO event_id;

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Kuro vs. Connor Mills')
  RETURNING id INTO match_id;
 
  INSERT into Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'APC Men''s Title Match', 'ENG'),
    (match_id, 'Match pour le titre de champion APC', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Megan Bayne vs. Celine')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'Singles Match', 'ENG'),
    (match_id, 'Match simple', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Thiago Montero vs. Joey Janela')
  RETURNING id INTO match_id;
  
  INSERT INTO Match_Desc (match_id, description, language_code) VALUES 
    (match_id, 'Singles Match', 'ENG'),
    (match_id, 'Match simple', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Georges Balzac vs. Griff vs. Jack Sans-Nom vs. Ravage')
  RETURNING id INTO match_id;

  INSERT into Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'BZW Men''s Hardcore Title Four Way Match', 'ENG'),
    (match_id, 'Match à 4 pour le titre de champion hardcore BZW', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES 
    (event_id, 'Rivality vs. Idolatry vs. Suplex Republik vs. Cian Noonan, LJ Clearly')
  RETURNING id INTO match_id;    

  INSERT into Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'APC Men''s Tag Team Title / CZW Men''s Tag Team Title Four Way Match', 'ENG'),
    (match_id, 'Match pour les titres de champion en équipe APC et BZW', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Joseph Fenech Jr. vs. Cara Noir')
  RETURNING id INTO match_id;
  
  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'BZW Men''s Title Match', 'ENG'),
    (match_id, 'Match pour le titre de champion BZW', 'FRE');

  INSERT INTO Match (event_id, workers) VALUES
    (event_id, 'Bobby Lashley vs. Aigle Blanc vs. Mecca')
  RETURNING id INTO match_id;

  INSERT INTO Match_Desc (match_id, description, language_code) VALUES
    (match_id, 'Three Way Match', 'ENG'),
    (match_id, 'Match à trois', 'FRE');
END $$;
