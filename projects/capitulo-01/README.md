
### Capítulo 1: Getting Started
#### Instalando e verificando a versão no linux

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

#### Verificando a versão instalada

```bash
rustup --version
```

#### Atualizando a versão do Rust
```bash
rustup update
```

#### Cargo
[Cargo is Rust’s build system and package manager](https://rust-book.cs.brown.edu/ch01-03-hello-cargo.html)

##### Verificando a versão instalada

```bash
cargo --version
```

##### Criando um novo projeto com o Cargo
```bash
cargo new hello_cargo
```

##### Build do projeto com o Cargo
```bash
cargo build
```

##### Rodando o projeto
```bash
cargo run
```

##### Verifica se o projeto compila
```bash
cargo check
```

##### Criando a release do projeto
```bash
cargo build --release
```

##### Cargo Watch
Cargo does not watch your files by default. But you can use plugins like cargo-watch for this purpose.

```bash
cargo install cargo-watch --locked
cargo-watch
```

##### Gerando a doc das dependências com o Cargo
```bash
cargo doc --open
```
