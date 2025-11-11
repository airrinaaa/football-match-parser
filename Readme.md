# Football Match Parser

## Brief Description

This project is a simple parser for **football matches data**. 

Each line describes one match with the _date, time, teams, score, stadium, and status_.
The parser reads these lines and builds a structured result for each match.
 
## Technical description
### What is parsed
The parser reads text lines where match fields are separated by ";".
For example:
```angular2html
2025-10-26; 17:15; RealMadrid - Barcelona; 2:1; Santiago Bernabeu
2025-11-04; 20:00; PSG - Bavaria; LIVE; Parc des Princes
2025-11-04; 20:00; PSG - Bavaria; 15'; Parc des Princes
2025-11-04; 20:00; PSG - Bavaria; LIVE; Parc des Princes; ongoing
2025-12-06; 18:00; Kudrivka - Dynamo Kyiv; - ; scheduled
```
**Each line may include:**
* date – match day in format YYYY-MM-DD;
* time – start time in format HH:MM;
* teams – home and away separated by -;
* score or mark – examples: 2:1, LIVE, 45', or -;
* stadium – optional name;
* status – optional field (scheduled, ongoing, or played)

### How it is parsed
1. Read one line of text.
2. Split the line into smaller parts by ;.
3. Trim extra spaces.
4. Read the date and check it follows the YYYY-MM-DD format.
5. Read the time and check it follows the HH:MM format.
6. Read the teams and split by - into home_team and away_team.
7. Check the score or mark to make sure it has the correct format.
   If it looks like “3:1”, read both numbers as the match score.
   If it shows “LIVE” or a minute like “44'”, it means the match is ongoing.
   If it is “-” it means the match has not started yet and is only scheduled.
8. Read the stadium if it exists.
9. If there is no status field, the parser decides it automatically.
    When the score looks like “3:1”, the match is played.
    When it has “LIVE” or a minute like “44'”, it is ongoing.
    When it has “-”, the match is scheduled (not started yet).
10. Build a Match structure with all fields: Match { date, time, home_team, away_team, home_score, away_score, stadium, status }.

### Parsing results and usage

The parser checks for basic errors(wrong date or time format etc.) and logical errors, such as inconsistent match status (for example, a scheduled match with a score, or a played match without the score).
Also it creates one Match struct for each line. Due to this structure, matches can later be filtered, sorted, or used for counting goals and checking statistics.

### Grammar rules
```
WHITESPACE = _{ " " | "\t" }
score_or_mark = {
      ASCII_DIGIT{1,3} ~ ":" ~ ASCII_DIGIT{1,3}
    | ASCII_DIGIT{1,3} ~ ("+" ~ ASCII_DIGIT{1,2})? ~ "'"
    | ^"live"
    | "-"
}
date = @{
    ASCII_DIGIT{4} ~ "-" ~
    ( "0" ~ ('1'..'9') | "1" ~ ('0'..'2') ) ~ "-" ~
    ( "0" ~ ('1'..'9') | ('1'..'2') ~ ASCII_DIGIT | "3" ~ ('0'..'1') )
}
time = @{
    ( "0" ~ ASCII_DIGIT
    | "1" ~ ASCII_DIGIT
    | "2" ~ ('0'..'3') )
    ~ ":"
    ~ ( "0" ~ ASCII_DIGIT
    | ('1'..'5') ~ ASCII_DIGIT )
}
SP = @{ " " }
name_char = { ('a'..'z' | 'A'..'Z' | '0'..'9') | "'" | "." | "(" | ")" | "&" }
team_word  = @{ name_char+ ~ ( "-" ~ name_char+ )* }
team_name = @{team_word ~ (SP ~ team_word )* ~ &( SP | ( "-" ~ WHITESPACE ) | ";" | !ANY )}
teams = {team_name ~ WHITESPACE* ~ "-" ~ WHITESPACE* ~ team_name}
stadium_name = { name_char+ ~ (SP ~ name_char+)* ~ &( ";" | !ANY ) }
status = {^"played" | ^"scheduled" | ^"ongoing"}
match_line = {date ~ ";" ~ time ~ ";" ~ teams ~ ";" ~ score_or_mark
  ~ (";" ~ stadium_name ~ (";" ~ status)? | ";" ~ status)?
  ~ !";"
}
```
| Rule           | Meaning (A1)                                                                                                                                                       | Examples                                                                                                                                                            |
|----------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `date`         | The match day. It must look like **YYYY-MM-DD**. The parser checks if the date is real.                                                                            | `2025-11-05`, `2024-02-28`                                                                                                                                          |
| `time`         | The match start time. It must look like **HH:MM** (24-hour clock). Hours 00–23, minutes 00–59.                                                                     | `17:45`, `09:00`, `23:59`                                                                                                                                           |
| `team_word`    | A small part of a team name. It can have letters, numbers, `' . ( ) &` and inside hyphens.                                                                         | `PSG`, `Dnipro-1`, `St.`, `Atletico`, `Queen's`, `R.C.D.`                                                                                                           |
| `team_name`    | The full team name. One or more `team_word` with spaces between them.                                                                                              | `Real Madrid`, `Paris Saint-Germain`, `Atletico Madrid (B)`                                                                                                         |
| `teams`        | Two team names separated by one dash `-` with one space before and after it.                                                                                       | `PSG - Bayern`, `RealMadrid-Barcelona`, `Metalist 1925 - Dynamo Kyiv`                                                                                               |
| `score_or_mark`| Shows the score or the current match situation. It can be a score (`2:1`), a minute (`45'`), `live`, or `-`.                                                       | `2:1`, `45'`, `90+3'`, `live`, `-`, `Live`, `120+3'`                                                                                                                |
| `stadium_name` | Optional. The name of the place where the match is played. Can have letters, spaces, and symbols `' . ( ) &`.                                                      | `Parc des Princes`, `St. Mary's Stadium`, `Dnipro Arena`                                                                                                            |
| `status`       | Optional. Shows match state — if it was played, is ongoing, or not started yet. It is case-insensitive.                                                            | `played`, `scheduled`, `ongoing`, `ONGOING`                                                                                                                         |
| `match_line`   | One full record about a football match. All fields go in this order: `date; time; teams; score_or_mark;` then optional `stadium; status`. No extra `;` at the end. | `2025-11-04; 20:00; PSG - Bavaria; 2:1; Parc des Princes`<br>`2025-12-06; 18:00; Kudrivka - Dynamo Kyiv; - ; scheduled`<br>`2025-11-04; 20:00; PSG - Bavaria; live` |

**Notes:**

- The parser ignores extra spaces and tabs between parts.

- No extra `;` at the end.  
  Invalid line: `2025-12-06; 18:99; Kudrivka - Dynamo Kyiv; - ; scheduled;`

- `stadium` and `status` are optional.  
  Valid: `date; time; teams; score` (without stadium and status)

- If the word after `;` is `played / scheduled / ongoing` (in any case), it is **status**, not stadium.  
  For example: `...; - ; PLAYED` → status = `played`, stadium = none

- Teams: must have one space before and after the dash.
  For example: `PSG-Real Madrid`(invalid line), `PSG - Real Madrid`(valid line)

- Team names can have hyphens inside a word.  
  Examples: `Dnipro-1`, `Paris Saint-Germain`

- Team and stadium names allow letters, numbers, `' . ( ) &`.  
  Invalid: `Camp*Nou`, `Dnipro-`, `Arena+`

- Score formats allowed:  
  `2:1` (numbers), `45'` (minute), `90+3'` (minute + added time), `live`(in any case), `-`, `120+2'`(extra time with added time)

- Spaces around `:` are OK.  
  For example: `2:1`, `2 : 1`, `2 :   1`

- Minute mark can have spaces around `+`.  
  For example: `90+3'`, `90 +3'`, `90  +  3'`

- Status is case-insensitive in input.  
  For example: `played`, `Played`, `PLAYED` → all OK

- Date must be `YYYY-MM-DD` and real (checked with chrono).  
  Invalid: `2025-13-26`, `2025-02-30`

- Time must be `HH:MM` with 00–23 / 00–59 (checked with rule).  
  Invalid: `25:00`, `20:75`

- Unknown words in place of status are not allowed as status. They stay as stadium text (if valid).

- Logic checks:
    - If there is a score like `2:1` → status should be **played**.
    - If mark is `-` → status should be **scheduled**.
    - If mark is `live` or `45'` → status should be **ongoing**.

- If the line has a written status, it **must** match the situation:
    - `2:1, scheduled` → **error** (scheduled cannot have a score)
    - `- , played` → **error** (played must have a score)

- If status is not written, the parser **infers** it from `score_or_mark`.  
  For example: `2:1` → `played`; `-` → `scheduled`; `live` or `45'` → `ongoing`






