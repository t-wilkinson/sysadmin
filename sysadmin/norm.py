#!/usr/bin/env python
r"""Normalize Strings

Usage::

    $ echo 'asdf--*--.asdf' | python -m sysadmin.norm
    asdf.asdf
    $ ls .
    We1d*fi^leN@m..
    $ python -m sysadmin.norm .
    $ ls .
    we1dfilenm
"""
import sys
import re
import unicodedata
import os


def string(file):
    value = unicodedata.normalize('NFKD', value).encode(
        'ascii', 'ignore').decode('ascii')
    value = re.sub(r'[^\w\s-]', '', value.lower())
    value = re.sub(r'[-\.\s]+', '-', ext).strip('-_')
    return value


def file_name(value):
    value = unicodedata.normalize('NFKD', value).encode(
        'ascii', 'ignore').decode('ascii')
    value = re.sub(r'[^./\w\s-]', '', value.lower())

    # split by `.` and normalize each portion
    parts = []
    for i, ext in enumerate(value.split('.')):
        ext = re.sub(r'[-\.\s]+', '-', ext).strip('-_')
        if ext or i == 0:  # preserve leading dot
            parts.append(ext)

    return '.'.join(parts)


def move(paths):
    def rename(path):
        normalized = file_name(path)
        if path != normalized and os.path.exists(path):
            os.renames(path, normalized)

    if isinstance(paths, str):
        rename(paths)

    elif isinstance(paths, list):
        for path in paths:
            if os.path.isdir(path):
                for root, dirs, files in os.walk(path):
                    for file in files:
                        rename(os.path.join(root, file))

            else:
                rename(path)


def run(args):
    if len(args) > 0:
        move(args)
    else:
        for line in sys.stdin:
            print(file_name(line))


if __name__ == '__main__':
    run(sys.argv[:1])
