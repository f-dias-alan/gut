# gut

Mini sistema de controle de versão inspirado no Git, escrito em Rust.

O objetivo do projeto é estudar:

* armazenamento baseado em conteúdo
* SHA1
* bancos de objetos imutáveis
* histórico encadeado
* sistemas de versionamento
* arquitetura interna do Git
* desenvolvimento de CLIs em Rust

---

# Funcionalidades implementadas

## `gut init`

Inicializa um repositório:

```bash
gut init
```

Cria:

```text
.gut/
├── HEAD
├── index
├── objects/
└── refs/
    └── heads/
        └── main
```

---

## `gut add`

Adiciona arquivos ao staging area:

```bash
gut add arquivo.txt
```

O comando:

1. lê o conteúdo do arquivo
2. gera SHA1
3. cria um blob
4. salva o blob em `.gut/objects`
5. adiciona ao `.gut/index`

---

## `gut commit`

Cria um commit:

```bash
gut commit -m "mensagem"
```

Cada commit contém:

* parent
* author
* date
* message
* arquivos staged

Exemplo:

```text
commit
parent c7b9...
author Alan Dias
date 2026-05-19T18:20:01

mudança no teste.txt

6931dd52da428baee2283a6d7018d1de86db41cb teste.txt
```

---

## `gut log`

Mostra histórico:

```bash
gut log
```

Exemplo:

```text
commit c9d85e...
parent 4b4206...
author Alan Dias
...
```

---

## `gut status`

Mostra staging area:

```bash
gut status
```

---

# Como funciona internamente

## Banco de objetos

Os objetos são armazenados em:

```text
.gut/objects/
```

Estrutura:

```text
HASH:
6931dd52da428baee2283a6d7018d1de86db41cb
```

vira:

```text
.gut/objects/69/31dd52da428baee2283a6d7018d1de86db41cb
```

Igual ao Git real.

---

# Tipos de objetos

## Blob

Armazena conteúdo do arquivo:

```text
blob
hello world
```

---

## Commit

Armazena snapshot do repositório:

```text
commit
parent ...
author ...
date ...
```

---

# Conceitos implementados

| Conceito                    | Implementado |
| --------------------------- | ------------ |
| SHA1 objects                | Sim          |
| Object database             | Sim          |
| Immutable commits           | Sim          |
| Staging area                | Sim          |
| HEAD                        | Sim          |
| refs/heads                  | Sim          |
| Commit chain                | Sim          |
| Content-addressable storage | Sim          |

---

# Estrutura do projeto

```text
src/
├── commands/
│   ├── add.rs
│   ├── commit.rs
│   ├── init.rs
│   ├── log.rs
│   ├── mod.rs
│   └── status.rs
├── object/
│   ├── mod.rs
│   └── store.rs
├── utils/
│   └── mod.rs
└── main.rs
```

---

# Dependências

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
sha1 = "0.10"
hex = "0.4"
chrono = "0.4"
anyhow = "1"
walkdir = "2"
```

---

# Como compilar

## Debug

```bash
cargo build
```

---

## Release

```bash
cargo build --release
```

Binário:

```text
target/release/gut
```

---

# Como usar

## Inicializar repositório

```bash
gut init
```

---

## Criar arquivo

```bash
echo "hello world" > teste.txt
```

---

## Adicionar

```bash
gut add teste.txt
```

---

## Commit

```bash
export GUT_AUTHOR="Alan Dias"

gut commit -m "primeiro commit"
```

---

## Histórico

```bash
gut log
```

---

# Instalação global

Depois de compilar:

```bash
cargo build --release
```

Copie o binário:

```bash
sudo cp target/release/gut /usr/local/bin/
```

Agora funciona globalmente:

```bash
gut init
```

igual:

```bash
git init
```

# Melhorias futuras

## cat-file

Visualizar objetos:

```bash
gut cat-file HASH
```

---

## Branches

```bash
gut branch
gut checkout
```

---

## Trees reais

Estrutura:

```text
commit -> tree -> blobs
```

---

## Diff

Comparar mudanças.

---

## Merge

Mesclar branches.

---

## Compressão zlib

Igual ao Git real.


---

# Licença

MIT

---

# Objetivo do projeto

O projeto foi criado para estudar os conceitos internos do Git e sistemas de armazenamento orientados a conteúdo utilizando Rust.
