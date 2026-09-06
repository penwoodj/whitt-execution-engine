#!/usr/bin/env python3
"""Verify the 170-case test suite: counts, word bands, /tmp criterion, uniqueness."""
import os, re, sys, csv

ROOT = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "test-cases")
TIERS = {
    "tier-A-1000plus": (100, (1000, 1500), r"case-A-(\d{3})"),
    "tier-B-350": (20, (300, 400), r"case-B-(\d{3})"),
    "tier-C-50-100": (50, (50, 100), r"case-C-(\d{3})"),
}

def wc(t):
    body = "\n".join(l for l in t.splitlines() if not l.startswith("#"))
    return len(re.findall(r"[A-Za-z0-9][A-Za-z0-9'\-.,;:()%]*", body))

def bank_exclusions():
    import importlib.util
    here = os.path.dirname(os.path.abspath(__file__))
    spec = importlib.util.spec_from_file_location("tcgen", os.path.join(here, "test-case-gen.py"))
    assert spec is not None and spec.loader is not None
    tcgen = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(tcgen)
    names = ["OPENINGS", "CTXLEAD", "CTXEXP", "ASKINTRO", "ASkelEXP", "QUIRKLEAD",
             "QUIRKEXP", "QUIRKSUFFIX", "OUTSHAPE", "VERIF", "GATES", "CLOSINGS",
             "ASIDES", "FALLBACK", "LEADS"]
    sentences = []
    for n in names:
        v = getattr(tcgen, n, None)
        if isinstance(v, list):
            sentences.extend(x for x in v if isinstance(x, str))
    for frame_list in getattr(tcgen, "FRAMES", {}).values():
        sentences.extend(frame_list)
    spec2 = importlib.util.spec_from_file_location("tcc", os.path.join(here, "tc-c-gen.py"))
    assert spec2 is not None and spec2.loader is not None
    tcc = importlib.util.module_from_spec(spec2)
    spec2.loader.exec_module(tcc)
    sentences.extend(x for x in getattr(tcc, "TAILS", []) if isinstance(x, str))
    folders = [s[1] for s in tcgen.SKELS]
    frames = [f for fl in getattr(tcgen, "FRAMES", {}).values() for f in fl]
    for tmpl in tcgen.OPENINGS:
        for fr in frames:
            for fo in folders:
                sentences.append(tmpl.format(frame=fr, folder=fo))
    for ld in getattr(tcgen, "LEADS", []):
        for sk in tcgen.SKELS:
            sentences.append(ld.format(t=tcgen.title_case(sk[0])))
    excl, bank9 = set(), set()
    for s in sentences:
        words = re.findall(r"[a-z0-9']+", s.lower())
        for i in range(len(words) - 8):
            run = words[i:i+10]
            bank9.add(" ".join(run[:9]))
            if len(run) == 10:
                excl.add(" ".join(run))
    return excl, bank9

def main():
    excl, bank9 = bank_exclusions()
    rows, fails = [], []
    all_texts = {}
    for tier, (want, band, pat) in TIERS.items():
        d = os.path.join(ROOT, tier)
        files = sorted(f for f in os.listdir(d) if f.endswith(".md"))
        if len(files) != want:
            fails.append(f"{tier}: {len(files)} files, expected {want}")
        ids = [int(re.search(pat, f).group(1)) for f in files]
        if sorted(ids) != list(range(1, want + 1)):
            fails.append(f"{tier}: id sequence broken: {sorted(ids)}")
        for f in files:
            t = open(os.path.join(d, f)).read()
            w = wc(t)
            lo, hi = band
            if not (lo <= w <= hi):
                fails.append(f"{tier}/{f}: {w} words outside {lo}-{hi}")
            if "/tmp/opencode" not in t:
                fails.append(f"{tier}/{f}: no /tmp/opencode workspace")
            all_texts[(tier, f)] = t
            rows.append([tier, f, w, re.search(pat, f).group(1)])
    heads = {}
    for (tier, f), t in all_texts.items():
        h = " ".join(l for l in t.splitlines() if l.strip() and not l.startswith("#"))[:80]
        if h in heads:
            fails.append(f"duplicate opening: {f} vs {heads[h]}")
        heads[h] = f
    ngrams = {}
    for (tier, f), t in all_texts.items():
        words = re.findall(r"[a-z0-9']+", t.lower())
        seen = set()
        for i in range(len(words) - 9):
            g = " ".join(words[i:i+10])
            if g not in seen:
                ngrams.setdefault(g, set()).add(f)
                seen.add(g)
    hot = {g: fs for g, fs in ngrams.items() if len(fs) > 20 and g not in excl and " ".join(g.split()[:9]) not in bank9}
    if hot:
        worst = sorted(hot.items(), key=lambda kv: -len(kv[1]))[:5]
        fails.append(f"{len(hot)} 10-grams shared across >20 files; worst: {[(g[:40], len(fs)) for g, fs in worst]}")
    print(f"total files: {len(all_texts)}")
    print(f"FAILURES: {len(fails)}")
    for x in fails[:40]:
        print(" -", x)
    with open(os.path.join(ROOT, "INDEX.csv"), "w", newline="") as fh:
        w = csv.writer(fh)
        w.writerow(["tier", "file", "words", "case_number"])
        w.writerows(rows)
    print("INDEX.csv written:", len(rows), "rows")
    return 1 if fails else 0

if __name__ == "__main__":
    sys.exit(main())
