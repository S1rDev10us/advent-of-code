get-puzzle year day:
  @echo "Year: {{year}}, Day: {{day}}"
  
  echo aoc download --day "{{ day }}" --year "{{ year }}" --input-file "puzzles/{{ year }}/{{ day }}/input" --puzzle-file "puzzles/{{ year }}/{{ day }}/puzzle.md"

get-puzzle-auto:
  @just get-puzzle "$(date +%Y)" "$(date +%d)" 

