# temprs - A temporary file stack manager in Rust

## [temprs on Crates.io](https://crates.io/crates/temprs)

## Install

cargo install temprs

## Usage
#### Replace CMD with any command, FILE with any file and INDEX with any index

- read stdin into new tempfile on top of stack
  ```CMD | tp```

- read stdin into new tempfile on top of stack and contents to stdout
  ```CMD | tp -v```

- contents of tempfile on top of stack to stdout
  ```tp | CMD```

- read stdin into tempfile at index 1
  ```CMD | tp -i 1```

- read stdin into tempfile at index 1 and write to stdout
  ```CMD | tp -i 1 -v```

- output tempfile at index 1 to stdout
  ```tp -o 1 | CMD```

- read from FILE and create tempfile on top of stack with contents of file.
  ```tp FILE | CMD```

- read from FILE and create tempfile on top of stack with contents of file.  write contents of FILE to CMD.
  ```tp -v FILE | CMD```
 
- write contents of FILE to tempfile 1
  ```tp -i 1 FILE | CMD```

- write contents of FILE to tempfile 1 then to stdout
  ```tp -vi 1 FILE | CMD```
 
- read from stdin to tempfile 1 then write to stdout
  ```CMD | tp -vi 1 | CMD```

- choose input tempfile and write to tempfile at index 2 and stdout
  ```CMD | tp -vi 2```

- choose output tempfile and write to stdout
  ```tp -o 1 | CMD```

- list all tempfiles on the stack to stdout
  ```tp -l```

- list all tempfiles with contents on the stack to stdout
  ```tp -L```

- remove all tempfiles
  ```tp -c```

- remove tempfile at INDEX
  ```tp -r INDEX```

- insert tempfile at INDEX
  ```CMD | tp -a INDEX```

- insert FILE at INDEX
  ```tp -a INDEX FILE```

- remove tempfile at top of stack
  ```tp -p```

- add tempfile at bottom of stack
  ```CMD | tp -u```

- add tempfile at bottom of stack
  ```CMD | tp -a 1```

- remove tempfile at bottom of stack
  ```tp -s```

### Notes

Temporary files are numbered with ascending order. The highest numbered tempfile is the top of the stack and tempfile 1
is at the bottom of the stack.

Negative indices are allowed at any INDEX argument position.  Indices go from 1 .. stack size and -1 .. -stack size.  INDEX of 0 is always invalid.

tp and temprs binaries are installed.

# created by MenkeTechnologies
