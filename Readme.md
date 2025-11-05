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




