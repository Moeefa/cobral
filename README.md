# CobraL

CobraL é um [pseudocódigo](https://pt.wikipedia.org/wiki/Pseudoc%C3%B3digo) com o objetivo de ensinar conceitos básicos de programação de forma simples e intuitiva. O código é interpretado e executado em Rust, mas com algumas restrições e adições.

O projeto ainda está em desenvolvimento e novas funcionalidades serão adicionadas em breve. Se você quiser contribuir, fique à vontade para abrir uma issue ou um pull request.

## Índice

- [Como funciona?](#como-funciona)
- [Como rodar?](#como-rodar)
- [Exemplo](#exemplo)
- [Documentação](#documentação)
  - [Tipos de dados](#tipos-de-dados)
  - [Operadores](#operadores)
  - [Estruturas de controle](#estruturas-de-controle)
  - [Bibliotecas](#bibliotecas)
    - [Entrada e saída](#entrada-e-saída)

## Roadmap

- [x] Implementar tipos de dados
- [x] Implementar estruturas de controle
- [x] Implementar biblioteca de entrada e saída
- [x] Implementar biblioteca de matemática
- [ ] Implementar funções
- [ ] Implementar loops
- [x] Implementar vetores
- [x] Implementar matrizes
- [ ] Implementar recursão
- [ ] Implementar comentários
- [ ] Implementar operadores lógicos (e, ou, não)
  - [x] Operador `nao` para negação de valores booleanos
  - [ ] Operador `e` para conjunção de valores booleanos
  - [ ] Operador `ou` para disjunção de valores booleanos
- [ ] Implementar operadores aritméticos
- [ ] Implementar operadores de incremento e decremento
- [ ] Implementar operadores de concatenação
- [x] Implementar operadores de comparação
- [x] Implementar linter para análise de código
- [ ] Melhorar a implementação do Lezer
- [ ] Destacar linha durante a execução passo a passo
- [ ] Incluir testes unitários
- [x] Leitura e escrita de arquivos
- [x] Console interativo para execução de código
- [x] Incluir exemplos de código e documentação
- [x] Implementar tipo de dados/dicas ao passar o mouse sobre uma variável, constante ou função

## Como funciona?

CobraL é um pseudocódigo baseado em Rust, com algumas simplificações e adições. O código é interpretado e executado em Rust, mas com algumas restrições e adições.

## Como rodar?

Para rodar um código em CobraL, você precisa ter o Rust instalado. Depois, basta rodar o comando `npm tauri dev` no diretório do projeto.

## Documentação

- [Sintaxe](#sintaxe)
- [Tipos de dados](#tipos-de-dados)
- [Operadores](#operadores)
- [Estruturas de controle](#estruturas-de-controle)
- [Bibliotecas](#bibliotecas)
  - [Entrada e saída](#io-entrada-e-saída)

### Sintaxe

A sintaxe de CobraL é simples e intuitiva. Cada instrução é separada por uma quebra de linha e um bloco de código é delimitado por chaves `{}`.
O uso de ponto e vírgula `;` é opcional.

```cobral
declare x = 10
escrever(x)

se (x == 10) {
  escrever("x é igual a 10")
} senao {
  escrever("x é diferente de 10")
}
```

### Tipos de dados

- `inteiro`: números inteiros
- `real`: números reais
- `texto`: sequência de caracteres
- `booleano`: verdadeiro ou falso
- `vetor`: conjunto de valores

Eles são inferidos automaticamente, então não é necessário declarar o tipo de uma variável.

```cobral
declare variavel = 10
declare variavel = 10.5
declare variavel = "texto"
declare variavel = verdadeiro
declare variavel = falso
declare variavel = [1, 2, 3]
```

### Operadores

- `==`: igualdade
- `!=`: diferença
- `>`: maior que
- `<`: menor que
- `>=`: maior ou igual a
- `<=`: menor ou igual a

```cobral
declare x = 10 == 10
```

### Estruturas de controle

- `se`: executa um bloco de código se uma condição for verdadeira
- `senao`: executa um bloco de código se a condição do `se` for falsa

```cobral
declare x = 10
declare y = 20

se (x == y) {
  escrever("x é igual a y")
} senao {
  escrever("x é diferente de y")
}
```

### Bibliotecas

- [IO](#io-entrada-e-saída): para entrada e saída de dados

#### IO: Entrada e saída

Para entrada e saída de dados, você pode usar a biblioteca `io`.
Você não precisa importar a biblioteca, ela já está disponível por padrão.

```cobral
declare x = ler("Digite seu nome:")
escrever("Você digitou: ", x)
```
