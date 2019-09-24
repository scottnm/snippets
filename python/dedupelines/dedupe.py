#!/usr/bin/python

import sys

def main():
    if len(sys.argv) < 2:
        print("Usage: python dedupe.py <file>")
        return

    unique_lines = set()
    filename = sys.argv[1]
    with open(filename, "r") as f:
        for line in f.readlines():
            unique_lines.add(line)

    with open(filename + "_deduped.txt", "w") as f:
        f.writelines(unique_lines)

if __name__ == "__main__":
    main()
