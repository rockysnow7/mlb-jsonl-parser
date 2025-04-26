from mlb_jsonl_parser import Parser

import exrex


def limit_regex(regex: str) -> str:
    return regex.replace("+", "{1,10}").replace("*", "{0,10}")

parser = Parser(False)

with open("test_data/748236.jsonl") as f:
    game_lines = f.readlines()
context = game_lines[0]

regex = limit_regex(parser.parse_line(context))
for _ in range(10):
    line = exrex.getone(regex)
    print(line)

    regex = limit_regex(parser.parse_line(line))

game = parser.finish()
print(game.plays[0].inning.number)
