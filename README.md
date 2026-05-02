# cs-lang

`cs-lang` e um projeto experimental de linguagem de programacao escrito em Rust.
Nesta fase, o projeto implementa a base do lexer e uma CLI simples para testar a
tokenizacao de entradas digitadas pelo usuario.

## Estado atual

- Lexer para identificadores, numeros e operadores aritmeticos basicos.
- Tokens suportados: `Identifier`, `Number`, `Plus`, `Minus`, `Multiply` e
  `Divide`.
- Binario CLI que le uma linha de codigo e imprime a lista de tokens gerada.
- Estrutura inicial para evoluir parser, AST, tratamento de erros e
  interpretador.

## Requisitos

- Rust com Cargo instalado.

Este projeto usa a edicao Rust 2024.

## Como executar

Clone o repositorio e execute:

```bash
cargo run
```

Depois digite uma expressao simples, por exemplo:

```text
idade + 10 * total
```

A saida sera uma representacao `Debug` dos tokens encontrados.

## Comandos uteis

```bash
cargo check
cargo fmt
```

## Estrutura

```text
src/
  lib.rs          Modulos publicos da biblioteca
  main.rs         Entrada da CLI
  token.rs        Definicao dos tokens
  lexer.rs        Tokenizador
  ast.rs          Reservado para a arvore sintatica abstrata
  parser.rs       Reservado para o parser
  error.rs        Reservado para erros da linguagem
  interpreter.rs  Reservado para o interpretador
```

## Proximos passos

- Implementar o parser.
- Definir a AST.
- Criar mensagens de erro estruturadas.
- Implementar avaliacao/interpretação das expressoes.
- Adicionar testes automatizados para o lexer e os proximos componentes.
