# [Minimal Reproducible
Example](https://stackoverflow.com/help/minimal-reproducible-example) of a bug
in `include_str!`

`cargo publish` fails when you use `include_str!` on a file with the `rs`
file extension.


## Steps to Reproduce

1. Clone this repo

`git clone https://github.com/masonforest/include_str_rs_file_mre`

2. Attempt to publish the repo

## Work around

As a work around you can append a different suffix to the file eg `file.rs.txt`
