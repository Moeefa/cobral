# CobraL

CobraL é um [pseudocódigo](https://pt.wikipedia.org/wiki/Pseudoc%C3%B3digo) com o objetivo de ensinar conceitos básicos de programação de forma simples e intuitiva. O código é interpretado e executado em Rust, mas com algumas restrições e adições.

O projeto ainda está em desenvolvimento e novas funcionalidades serão adicionadas em breve. Se você quiser contribuir, fique à vontade para abrir uma issue ou um pull request.

## Índice

- [Como funciona?](#como-funciona)
- [Como rodar?](#como-rodar)
- [Exemplo](#exemplo)
- [Documentação](#documentação)
  - [Sintaxe](#sintaxe)
  - [Tipos de dados](#tipos-de-dados)
  - [Operadores de comparação](#operadores-de-comparação)
  - [Operadores lógicos](#operadores-lógicos)
  - [Operadores aritméticos](#operadores-aritméticos)
  - [Estruturas de controle](#estruturas-de-controle)
  - [Estruturas de repetição](#estruturas-de-repetição)
  - [Bibliotecas](#bibliotecas)
    - [Entrada e saída](#entrada-e-saída)
    - [Matemática](#matemática)
    - [Parsing](#parsing)

## Roadmap

- [x] Implementar tipos de dados
- [x] Implementar estruturas de controle
- [x] Implementar biblioteca de entrada e saída
- [x] Implementar biblioteca de matemática
- [x] Implementar biblioteca para parsing de tipos de dados
- [ ] Implementar funções
- [x] Implementar loops
- [x] Implementar vetores
- [x] Implementar matrizes
- [ ] Implementar comentários
- [x] Implementar operadores lógicos (e, ou, não)
  - [x] Operador `nao` para negação de valores booleanos
  - [x] Operador `e` para conjunção de valores booleanos
  - [x] Operador `ou` para disjunção de valores booleanos
- [x] Implementar operadores aritméticos
- [ ] Implementar operadores de incremento e decremento
- [x] Implementar operadores de concatenação
- [x] Implementar operadores de comparação
- [x] Implementar linter para análise de código
- [x] Melhorar a implementação do Lezer
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

### Operadores de comparação

- `==`: igualdade
- `!=`: diferença
- `>`: maior que
- `<`: menor que
- `>=`: maior ou igual a
- `<=`: menor ou igual a

```cobral
declare x = 10 == 10
```

### Operadores lógicos

- `nao`: negação
- `e`: conjunção
- `ou`: disjunção

```cobral
declare x = verdadeiro
declare y = falso

declare z = nao x
declare w = x e y
declare v = x ou y
```

### Operadores aritméticos

- `+`: adição
- `-`: subtração
- `*`: multiplicação
- `/`: divisão
- `%`: módulo

```cobral
declare x = 10 + 10
declare y = 20 - 10
declare z = 10 * 10
declare w = 10 / 2
declare v = 10 % 2
```

### Estruturas de controle

- `se`: executa um bloco de código se uma condição for verdadeira
- `senao`: executa um bloco de código se a condição do `se` for falsa

```cobral
declare x = 10 + 10
declare y = 20

se (x == y) {
  escrever("x é igual a y")
} senao {
  escrever("x é diferente de y")
}
```

### Estruturas de repetição

- `para`: executa um bloco de código um número específico de vezes

```cobral
para (declare i = 0; i < 10; i = i + 1) {
  escrever(i)
}
```

### Bibliotecas

- [IO](#entrada-e-saída): para entrada e saída de dados
- [Matemática](#matemática): para operações matemáticas
- [Parsing](#parsing): para parsing de tipos de dados

#### Entrada e saída

Para entrada e saída de dados, você pode usar a biblioteca `io`.
Você não precisa importar a biblioteca, ela já está disponível por padrão.

```cobral
declare x = ler("Digite seu nome:")
escrever("Você digitou: ", x)
```

#### Matemática

Para operações matemáticas, você pode usar a biblioteca `matematica`.
Você não precisa importar a biblioteca, ela já está disponível por padrão.

```cobral
declare x = raiz(50)
escrever(x)
```

#### Parsing

Para fazer parsing de tipos de dados, você pode usar a biblioteca `parsing`.
Você não precisa importar a biblioteca, ela já está disponível por padrão.

```cobral
declare x = int(10)
escrever(x)

declare y = real(10.5)
escrever(y)
```
