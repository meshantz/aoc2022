#! /bin/env python

"""Requires requests, and a cookie for AOC.

Usage:

    python3 -m venv venv
    . venv/bin/activate
    pip insall -U pip
    pip install requests
    chmod a+x start.py
    export AOC_COOKIE=<grab your cookie from dev tools>
    ./start <daynumber>
    # eg:
    # ./start 7
"""

from os import environ
import sys
import requests

TEMPLATE = """
pub fn solve() {
    println!("Solution Pending...");
}
"""

MOD_REPLACE = """mod day{day:02};
// NEXTMOD
"""

ENUM_REPLACE = """Day{day:02},
    // NEXTENUM
"""

MATCH_REPLACE = """Days::Day{day:02} => day{day:02}::solve(),
            // NEXTMATCH
"""

if __name__ == "__main__":
    day = int(sys.argv[1])
    cookies = {
        "session": environ.get("AOC_COOKIE")
    }
    data = requests.get(
        f"https://adventofcode.com/2022/day/{day}/input",
        cookies=cookies,
    )
    with open(f"data/day{day:02}.txt", "w") as fp:
        fp.write(data.content.decode("utf-8"))

    with open(f"src/day{day:02}.rs", "w") as fp:
        fp.write(TEMPLATE)

    with open("src/main.rs") as fp:
        raw = fp.read()

    raw = raw.replace("// NEXTMOD\n", MOD_REPLACE.format(day=day))
    raw = raw.replace("// NEXTENUM\n", ENUM_REPLACE.format(day=day))
    raw = raw.replace("// NEXTMATCH\n", MATCH_REPLACE.format(day=day))

    with open("src/main.rs", "w") as fp:
        fp.write(raw)
