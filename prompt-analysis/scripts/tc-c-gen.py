import os
BASE = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "test-cases", "tier-C-50-100")
os.makedirs(BASE, exist_ok=True)
C = [
("folder-summary", "make a folder /tmp/opencode/c1 with a dozen text files about whatever, then give me a one-paragraph summary of the whole folder and a file list with word counts. paste the summary in chat. don't overthink it."),
("file-merger", "create /tmp/opencode/c2 with three csv files sharing a header, write a script that concatenates them into one, verify the row count is the sum of the parts, show me the counts."),
("top-ten", "generate a file of 100 random numbers in /tmp/opencode/c3, then tell me the top ten and the median, with a tiny script doing the work so it's checkable. paste both in chat."),
("line-counter", "write a script that counts lines per file across a folder you seed at /tmp/opencode/c4 with mixed text files, output one table sorted by count descending, paste it here."),
("dupe-check", "make /tmp/opencode/c5 containing ten files where two are exact duplicates, find the dupes with a script, hash-based, and tell me which pair and their hash."),
("rename-batch", "seed /tmp/opencode/c6 with files named like IMG_001.jpg through IMG_020.jpg, write a plan that renames them to 2026-08-29_XX.jpg, apply it, list before and after for five of them."),
("word-count-goal", "create a text file at /tmp/opencode/c7 that is exactly 500 words about anything, verify the count with wc or a script, and show me the first and last sentence plus the count."),
("reverse-lines", "put 30 lines in a file at /tmp/opencode/c8, reverse the line order into a new file, don't touch the original, and paste both first five lines side by side so i can see it worked."),
("sort-pass", "generate 50 messy strings into /tmp/opencode/c9, mixed case and spaces, sort them case-insensitively into an output file, show the before and after top ten."),
("csv-stats", "make a small csv at /tmp/opencode/c10, 20 rows, two numeric columns, give me min max and mean per column from a script, not your head, paste the numbers."),
("json-validate", "write 15 tiny json files into /tmp/opencode/c11 where exactly three are invalid, validate them all with a script, list the three failures with the parse error quoted."),
("env-extract", "create a fake .env at /tmp/opencode/c12 with 12 key-value lines and some comments, extract just the keys to a list, count them, show me the list."),
("log-count", "seed /tmp/opencode/c13 with a fake log where the word error appears on 17 lines, count those lines with a script, tell me the count and the first two matching lines."),
("placeholder-swap", "put a text file at /tmp/opencode/c14 containing the placeholder NAME five times, replace all of them with testuser, show the count of replacements and one swapped line."),
("tail-check", "generate a 200-line file at /tmp/opencode/c15, show me the last ten lines via a script, and tell me what line number the file ends on, then verify with a count."),
("spacing-fix", "make /tmp/opencode/c16 with a text file full of double spaces, collapse them all to single, report how many replacements happened, and verify none remain."),
("upper-pass", "write a file of 25 lowercase words at /tmp/opencode/c17, produce a second file with them uppercased, diff count should be 25 lines, verify and show three examples."),
("uniq-count", "create a list file at /tmp/opencode/c18 with repeated words, some appearing 4 times, run it through a script, give me each unique word with its count, sorted by count."),
("date-list", "generate the dates of every monday in 2026 into a file at /tmp/opencode/c19, verify there are 52 or 53 of them, say which, and paste the first and last three."),
("size-report", "drop files of varying sizes into /tmp/opencode/c20, give me a table of each file and its bytes sorted biggest first, plus the total, script-generated."),
("char-census", "write a paragraph into /tmp/opencode/c21, then count vowels, consonants, digits, and spaces with a script, table in chat, and the paragraph's word count too."),
("range-split", "put numbers 1 through 100 in a file at /tmp/opencode/c22, split them into three output files, under 34, 34 to 66, over 66, verify counts are 33, 33, 34, show them."),
("acronym-pass", "create a file at /tmp/opencode/c23 with five full names, one per line, produce their initials as acronyms in a second file, show all five pairs in chat."),
("dotfile-sweep", "seed /tmp/opencode/c24 with visible and dot files mixed, write a script listing just the dotfiles with sizes, count them, paste the list."),
("ext-census", "fill /tmp/opencode/c25 with 30 files across four extensions, count per extension with a script, table in chat, and note any extension with only one file."),
("empty-check", "make /tmp/opencode/c26 with 12 files where three are empty, find the empty ones with a script, list them, and give the total count so i know nothing else is empty."),
("trim-pass", "generate 40 lines with trailing whitespace into /tmp/opencode/c27, strip the trailing whitespace in place, report lines changed, verify with a rescan showing zero remain."),
("longest-line", "write a text file at /tmp/opencode/c28 with varied line lengths, tell me the longest line's number, its length, and its content, script-found, paste all three."),
("case-fix", "create /tmp/opencode/c29 with filenames in random case like README.Md and TXT.txt, normalize them all to lowercase, list before and after pairs, count them."),
("interval-gen", "generate timestamps every 15 minutes from 09:00 to 12:00 into a file at /tmp/opencode/c30, verify there are 13 lines, paste first three and last three."),
("sum-check", "put 25 prices in a file at /tmp/opencode/c31, sum them with a script in integer cents, show the total, and the highest and lowest price with their line numbers."),
("mask-emails", "create a file at /tmp/opencode/c32 with ten fake emails in text, mask them all as [redacted], count replacements, show two before-after line pairs."),
("repeat-finder", "write a text file at /tmp/opencode/c33 where one exact sentence appears three times, find repeated sentences with a script, report the sentence and its count."),
("slugify-pass", "put ten messy titles in a file at /tmp/opencode/c34, slugify them, lowercase-hyphenated, into an output file, show all ten mappings in chat."),
("matrix-write", "write a 5 by 5 grid of numbers to /tmp/opencode/c35, compute row sums and column sums with a script, paste both lists, and the grand total."),
("palindrome-hunt", "generate a word list of 50 entries at /tmp/opencode/c36 including a few real palindromes, find them with a script, list them, and if there are none plant some and rerun."),
("folder-tree", "build a nested folder structure three levels deep under /tmp/opencode/c37 with about 15 files scattered, print the tree, count files per top-level folder, paste both."),
("delta-lines", "create two versions of a 40-line file at /tmp/opencode/c38 differing in five lines, diff them with a script, tell me the five line numbers changed, show two of them."),
("mode-finder", "put 60 numbers in a file at /tmp/opencode/c39 with one value repeated more than any other, find the mode with a script, report value and count, and the runner-up too."),
("bullet-rewrite", "write a paragraph of four sentences at /tmp/opencode/c40, convert it to a markdown bullet list in a new file, one bullet per sentence, show both in chat."),
("oldest-file", "seed /tmp/opencode/c41 with ten files, then set their mtimes to staggered fake times, find the oldest and newest with a script, paste both with timestamps."),
("char-swap", "put a text file at /tmp/opencode/c42 full of the word color, produce a copy with every occurrence switched to colour, count swaps, verify none of the original remain in the copy."),
("negatives-split", "generate 30 numbers, some negative, into /tmp/opencode/c43, split into positives and negatives files, verify counts sum to 30, show both counts and one example each."),
("tab-fix", "create a file at /tmp/opencode/c44 using tabs between fields, convert to commas into a new file, report line count unchanged, show one converted line."),
("hex-dump-mini", "write the word opencode to a file at /tmp/opencode/c45, hex-dump it with a script, show the bytes, and tell me the length in bytes, explain any surprise."),
("range-report", "put 100 sensor readings in /tmp/opencode/c46 with a few physically impossible ones, flag readings outside 0 to 100 with a script, list them with line numbers."),
("acronym-expand", "write ten common tech acronyms one per line at /tmp/opencode/c47 with their expansions in a second file, verify line counts match, show five pairs."),
("midpoint-check", "generate an even-count list of numbers at /tmp/opencode/c48, compute the true median as the mean of the middle two with a script, show the pair and the result."),
("banner-maker", "take any five-word phrase, write it to /tmp/opencode/c49, produce an ascii banner version in a second file using hash characters, show it in chat, that's the whole job."),
("final-audit", "run one script over /tmp/opencode/c1 through c49 if they exist, count files and folders total, tell me the count, and note any folder with zero files, that's it."),
]
TAILS = [
 "paste the result in chat, quick work, don't gold-plate it.",
 "everything stays in that folder, no network, no installs, standard library only.",
 "script does the counting, not your head, and exit nonzero if anything surprises you.",
 "show me the actual output lines, not a summary of them, i'll be the judge.",
 "rerun-safe by design, no prompts, nothing interactive anywhere.",
 "small job, do it in one pass and paste the numbers when done.",
 "if a file's missing or a count's off, say so plainly, don't paper over it.",
]
def wc(t):
    import re
    return len(re.findall(r"[A-Za-z0-9][A-Za-z0-9'\-.,;:()%]*", t))

def main():
    for i, (slug, body) in enumerate(C, 1):
        import random
        rng = random.Random(31000 + i * 617)
        text = f"# Tier-C Case {i:03d} — {slug.replace('-', ' ')}\n\n{body}\n"
        w = wc(text)
        tails = TAILS[:]
        rng.shuffle(tails)
        while w < 55 and tails:
            body = body.rstrip() + " " + tails.pop()
            text = f"# Tier-C Case {i:03d} — {slug.replace('-', ' ')}\n\n{body}\n"
            w = wc(text)
        assert 50 <= w <= 100, f"{slug}: {w} out of band"
        open(os.path.join(BASE, f"case-C-{i:03d}-{slug}.md"), "w").write(text)
    print(f"tier-C: {len(C)} cases, all 50-100 words")

if __name__ == "__main__":
    main()
