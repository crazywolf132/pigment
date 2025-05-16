#!/usr/bin/env python3
"""
Scrapes color tables from the three Wikipedia pages and spits
`generated/colors.rs`, ready to be `include!`-d by the Rust crate.

Run:  python tools/scrape.py
Requires: beautifulsoup4, requests, tqdm
"""
import re, json, pathlib, textwrap, itertools, requests
from bs4 import BeautifulSoup
from tqdm import tqdm

PAGES = [
    "https://en.wikipedia.org/wiki/List_of_colors:_A%E2%80%93F",
    "https://en.wikipedia.org/wiki/List_of_colors:_G%E2%80%93M",
    "https://en.wikipedia.org/wiki/List_of_colors:_N%E2%80%93Z",
]

def canonical(name: str) -> str:
    """lowercase, remove anything that isn’t [a-z0-9]"""
    return re.sub(r'[^a-z0-9]', '', name.lower())

out = []
seen_keys = set()
for url in PAGES:
    soup = BeautifulSoup(requests.get(url, timeout=30).text, "html.parser")
    for row in tqdm(soup.select("table.wikitable tr")):
        cells = row.select("td")
        if len(cells) < 2:
            continue
        name = cells[0].get_text(strip=True)
        hex_match = re.search(r"#([0-9A-Fa-f]{6})", cells[1].get_text())
        if not hex_match:
            continue
        hex_code = "#" + hex_match.group(1).upper()
        rgb = tuple(int(hex_code[i : i + 2], 16) for i in (1, 3, 5))
        key = canonical(name)

        # Handle duplicate keys by adding a number suffix
        if key in seen_keys:
            counter = 2
            while f"{key}{counter}" in seen_keys:
                counter += 1
            key = f"{key}{counter}"

        seen_keys.add(key)
        out.append((key, name, hex_code, *rgb))

# write Rust source
colors_rs = pathlib.Path("generated/colors.rs")
colors_rs.parent.mkdir(exist_ok=True)
with colors_rs.open("w") as f:
    f.write("///  ***  AUTO-GENERATED  – DO NOT EDIT BY HAND  ***\n")
    f.write("use phf::{phf_map};\n\n")
    f.write("pub static COLORS: phf::Map<&'static str, crate::Color> = phf_map! {\n")
    for key, name, hx, r, g, b in out:
        f.write(f'    "{key}" => crate::Color{{ name:"{name}", hex:"{hx}", rgb:({r},{g},{b}) }},\n')
    f.write("};\n")

print(f"Wrote {colors_rs} with {len(out)} colors")
