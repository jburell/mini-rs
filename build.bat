rustc src/main.rs -O -C no-stack-check -C relocation-model=static -L syscall.rs/target --emit obj
crinkler.exe /SUBSYSTEM:CONSOLE /TINYHEADER /TINYIMPORT /ENTRY:WinMain main.o kernel32.lib user32.lib