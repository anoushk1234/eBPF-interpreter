# Rust eBPF Interpreter

This project was inspired by seeing this [golang-eBPF-interpreter](https://github.com/mryalamanchi/eBPF-interpreter) built by my mentor Mr.Yalamanchi.

## Usage
```bash
cargo run --release
```

Not all instructions are supported yet, I have only added MOV_REG,ADD_REG,MOV_IMM,SUB_IMM,BRANCH_EXIT
