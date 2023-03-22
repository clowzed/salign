# salign
Aligner of comment separator in asm files
![build](https://github.com/clowzed/salign/actions/workflows/build.yml/badge.svg)
![Crates.io](https://img.shields.io/crates/v/salign?color=green)
![Crates.io](https://img.shields.io/crates/d/salign?color=green)

# Installation
```bash
cargo install salign
```
#### Or
```bash
git clone https://github.com/clowzed/salign.git
cd salign.git
cargo build --release
sudo mv ./target/release/salign /usr/bin
```

# Usage
```bash
salign main.asm
```

# Showcase
```asm
.model tiny  ;set memory model
.dosseg
.data
        msg db "hello, world!", 0dh, 0ah, '$'; message
.code
.startup
        mov ah, 09h ; moves 09h into ah
        mov dx, offset msg
        int 21h           ;run int 21h
        mov ah, 4ch
        int 21h      ;exit
end
```

With -e flag
```asm
.model tiny                                      ;    set memory model
.dosseg                                          ;
.data                                            ;
        msg db "hello, world!", 0dh, 0ah, '$'    ;    message
.code                                            ;
.startup                                         ;
        mov ah, 09h                              ;    moves 09h into ah
        mov dx, offset msg                       ;
        int 21h                                  ;    run int 21h
        mov ah, 4ch                              ;
        int 21h                                  ;    exit
end                                              ;

```

Without -e flag
```asm
.model tiny                                      ;    set memory model
.dosseg
.data
        msg db "hello, world!", 0dh, 0ah, '$'    ;    message
.code
.startup
        mov ah, 09h                              ;    moves 09h into ah
        mov dx, offset msg
        int 21h                                  ;    run int 21h
        mov ah, 4ch
        int 21h                                  ;    exit
end
```

# Arguments

| Short | Long                           | What                                                            |
|-------|--------------------------------|-----------------------------------------------------------------|
| -h    | --help                         | Prints help information                                         |
| -V    | --version                      | Prints version information                                      |
| -e    | --place-separator-on-each-line | If setted we will place the separator on each line              |
| -l    | --lmargin <lmargin>            | Set amount of spaces between code and separator [default: 4]    |
| -r    | --rmargin <rmargin>            | Set amount of spaces between separator and comment [default: 4] |
| -s    | --separator <separator>        | Set separator(devider) between code and comments [default: ;]   |
