# fnd

Replacement for find command because I always forgot how to use find properly!

Usage:

fnd [-ir] [search] [dir]

optional flags:
 - i : case insensitive
 - r : search as regex
 
search : part of filename or regex (returns all files if ommited)

dir : dir to search in (current dir if ommited)

eg:

`fnd pizza /`
returns all files with pizza in the filename on the whole disk.

`fnd -r '\.js$'`
returns all .js files in current dir (and subdirectories)
