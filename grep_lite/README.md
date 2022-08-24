# GrepLite

Welcome to logfind, the very light grep-like command line tool!

How to use this tool: install the tool on the command line by navigating into the project directory and then writing "python setup.py install"

You can then search through any directory you navigate to and calling logfind followed by any number of strings you want to look through.

Options: -o (for OR rather than AND search)

Out of the box, the tool looks through .txt, .py, and .log files. But you can configure different file extensions to look through by creating a file called '.logfind' in your home directory and put a regular expression on each line for the types of files you want to look through.
