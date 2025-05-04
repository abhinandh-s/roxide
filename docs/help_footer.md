By default, roxide does not remove directories.  Use the --recursive (-r or -R)
option to remove each listed directory, too, along with all of its contents.

Any attempt to remove a file whose last file name component is '.' or '..'
is rejected with a diagnostic.

To remove a file whose name starts with a '-', for example '-foo',
use one of these commands:
  roxide -- -foo

  roxide ./-foo

If you use roxide to remove a file, it might be possible to recover
some of its contents, given sufficient expertise and/or time.  For greater
assurance that the contents are unrecoverable, consider using shred(1).

online help: <https://>
Full documentation <https://>
or available locally via: 
