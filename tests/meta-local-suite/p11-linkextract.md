# Task: HTML Link Extractor

Write a Python tool named linkextract.py using only the standard library (html.parser or regex). It reads an HTML file (path as first CLI argument) and prints every unique absolute http:// or https:// URL found in href or src attributes, one per line, sorted alphabetically. Relative URLs, fragment-only links, mailto: and javascript: links are excluded. Duplicate URLs are printed once. No network access. A missing file prints a clear error to stderr and exits nonzero without a traceback.
