# Meginfo

This is a command-line tool for parsing and querying FIF format neuroimaging data.  Produces CSV-formatted output with multiple files.

For example, assuming you have .fif files in a directory `data`, the command

`find data | meginfo -t meas_date -t sfreq`

gives the following output:

```
file,meas_date,sfreq
file_0.fif,1924-12-31 16:47:34.000672177,1000
file_1.fif,1924-12-31 16:25:31.000818563,1000
file_2.fif,1924-12-31 17:16:02.000903171,1000
```

# Installation

Assuming you have rust installed, clone the repo and run `cargo build --release`.

# Usage

Specify files with `-f {filename}` and tags with `-t {tagname}`.  You can specify multiple files and tags:

`meginfo -f file1.fif -f file2.fif -t subj_id -t meas_date`

A file list can also be read from stdin:

`find data | meginfo -t sfreq -t subj_id -t meas_date`

You can also print a representation of the fiff tree structure using the `-s` flag:

`meginfo -f data/file_0.fif -s`

Change the log level with `-l`.  For example, `-l error` will suppress warnings.

Show all command line options using `meginfo --help`.

# Notes

This is my first ever rust project; I just got done reading the book.  As a novice to rust and systems programming in general, I mainly focused on implementing the functionality I wanted, at the expense of basically everything else: I've not published it on crates.io yet for this reason.  If you're interested in using this as an actual tool, I'm open to suggestions/pull requests/whatever, feel free to get in touch!

# Acknowledgements

The fiff tag, block and primitive definitions were sourced from `https://github.com/mne-tools/fiff-constants`.