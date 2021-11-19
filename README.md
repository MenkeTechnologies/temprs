# temp - A tempfile stack manager in Rust

Tempfile files are named with increasing numbers are their filenames The highest numbered temp file is on the top of the
stack

## [temp on Crates.io](https://crates.io/crates/temp)

## Usage

- send stdin into temp, create temp file on top of stack and write that tempfile to stdout
  ```cmd | temp```

- read from temp file on top of stack and write to stdout
  ```temp | nl```

- read stdin into chosen input tempfile 1 and write to stdout
  ```ls | temp -i 1```

- choose output tempfile and write to stdout
  ```temp -o 1 | nl```

- read from file and create temp file on top of stack with contents of file writes contents of <file> to nl.
  ```temp <file> | nl```

- write contents of file to temp file 1 then write to stdout 
  ```temp -i 1 <file> | nl```

- read from stdin to temp file 1 then write to stdout
  ```ls | temp -i 1 | nl```

- choose input tempfile and write to temp file 2 and stdout
  ```ls | temp -i 2```

- choose ouput tempfile and write to stdout
  ```temp -o 1 | nl```

- list all temp files on the stack to stdout
  ```temp -l```

- list all temp files with contents on the stack to stdout
  ```temp -L```

- remove all temp files
  ```temp -c```
 
# created by MenkeTechnologies
