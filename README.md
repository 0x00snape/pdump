______________________________________________
![maxresdefault](https://i.imgur.com/O7ofQOa.jpeg)
______________________________________________
# pdump
pdump is a running process dumper written in rustlang which uses the syscall process_vm_readv().

<h2>Working</h2>
pdump identifies the PID of processs passed. Memory map of process is stored in /proc/process_pid/maps, using it for getting addresses range from start to end which is stored on the heap. Using the memory address startADDRESS and endADDRESS to get the local (iovec buffer) and remote (iovec buffer) and passing it to syscall process_vm_readv() and dumps the bytes of data.

## Installation and Usage
```bash
:$ git clone https://github.com/0x00snape/pdump.git
:$ cd pdump
:$ cargo build --release
```
## License
This project is licensed under [MIT](https://github.com/0x00snape/pdump/blob/main/LICENSE)
