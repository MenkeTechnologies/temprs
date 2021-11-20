# temprs - A tempfile stack manager in Rust

Tempfile files are named with increasing numbers are their filenames The highest numbered temprs file is on the top of the
stack

## [temprs on Crates.io](https://crates.io/crates/temprs)

## Usage (tp is shorthand for temprs)

- send stdin into temprs, create temprs file on top of stack and write that tempfile to stdout
  ```cmd | temprs```

- read from temprs file on top of stack and write to stdout
  ```temprs | nl```

- read stdin into chosen input tempfile 1 and write to stdout
  ```ls | temprs -i 1```

- choose output tempfile and write to stdout
  ```temprs -o 1 | nl```

- read from file and create temprs file on top of stack with contents of file writes contents of <file> to nl.
  ```temprs <file> | nl```

- write contents of file to temprs file 1 then write to stdout 
  ```temprs -i 1 <file> | nl```

- read from stdin to temprs file 1 then write to stdout
  ```ls | temprs -i 1 | nl```

- choose input tempfile and write to temprs file 2 and stdout
  ```ls | temprs -i 2```

- choose ouput tempfile and write to stdout
  ```temprs -o 1 | nl```

- list all temprs files on the stack to stdout
  ```temprs -l```

- list all temprs files with contents on the stack to stdout
  ```temprs -L```

- remove all temprs files
  ```temprs -c```
 
# created by MenkeTechnologies
