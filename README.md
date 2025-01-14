# Portugol CobraL

CobraL é uma linguagem extremamente rápida baseada no [Portugol (Português estruturado)](https://pt.wikipedia.org/wiki/Portugol) com o objetivo de ensinar conceitos básicos de programação de forma simples e intuitiva. O código é interpretado e executado em Rust, mas com algumas restrições e adições.
Vale ressaltar de que este projeto não tem nenhuma correlação com o Portugol Studio, cujo o mesmo é uma IDE para Portugol, e são projetos totalmente distintos.

O projeto ainda está em desenvolvimento e novas funcionalidades serão adicionadas em breve. Se você quiser contribuir, fique à vontade para abrir uma issue ou um pull request.

## Benchmarks

Os benchmarks a seguir foram realizados em um computador com as seguintes especificações (usado a média de tempo de 3 execuções):

- **Sistema operacional**: Windows 11 Pro
- **Processador**: Ryzen 5 5600G
- **Memória RAM**: 16 GB

| Programa          | CobraL (ms) | Portugol Studio (ms) | Diferença              |
| ----------------- | ----------- | -------------------- | ---------------------- |
| Iteração até 1000 | 1.6         | 4293                 | 2683 vezes mais rápido |

## Índice

- [Como funciona?](#como-funciona)
- [Como instalar?](#como-instalar)
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
    - [Conversão de tipos de dados](#conversão-de-tipos-de-dados)

## Roadmap

- [x] Implementar tipos de dados
  - [ ] Adicionar tipos de dados explicitamente de maneira opcional
- [x] Implementar estruturas de controle
- [x] Implementar biblioteca de entrada e saída
- [x] Implementar biblioteca de matemática
- [x] Implementar biblioteca para conversão de tipos de dados
- [x] Implementar funções
  - [x] Corrigir chamada de funções recursivas
- [x] Implementar loops
- [x] Implementar vetores
- [x] Implementar matrizes
- [x] Implementar comentários
- [x] Implementar operadores lógicos (e, ou, não)
  - [x] Operador `nao` para negação de valores booleanos
  - [x] Operador `e` para conjunção de valores booleanos
  - [x] Operador `ou` para disjunção de valores booleanos
- [x] Implementar operadores aritméticos
- [x] Implementar operadores de incremento e decremento
- [x] Implementar operadores de concatenação
- [x] Implementar operadores de comparação
- [x] Ajustar o Textmate para a sintaxe de CobraL
- [x] Implementar análise de código
- [ ] Implementar execução de código passo a passo
  - [ ] Destacar linha durante a execução passo a passo
- [ ] Incluir testes unitários
- [x] Leitura e escrita de arquivos
- [x] Console interativo para execução de código
- [x] Incluir exemplos de código e documentação
- [x] Implementar tipo de dados/dicas ao passar o mouse sobre uma variável, constante ou função
- [x] Implementar condições escolha-caso (switch-case)

## Como funciona?

CobraL é um pseudocódigo baseado em Rust, com algumas simplificações e adições. O código é interpretado e executado em Rust, mas com algumas restrições e adições.

## Como instalar?

Para instalar o CobraL, você pode baixar a última versão disponível em [releases](https://github.com/Moeefa/cobral/releases/latest) de acordo com o seu sistema operacional e arquitetura, ou compilar o código-fonte manualmente.
Para compilar o código-fonte, você precisa ter o [Rust](https://www.rust-lang.org/tools/install) instalado na sua máquina.

```bash
git clone https://github.com/Moeefa/cobral.git
```

E então, execute o comando abaixo para compilar o código-fonte:

```bash
bun tauri build
```

## Documentação

### Sintaxe

A sintaxe de CobraL é simples e intuitiva. Cada instrução é separada por uma quebra de linha e um bloco de código é delimitado por chaves `{}`.

**O uso de ponto e vírgula `;` é totalmente opcional.**

```cobral
declare x = 10;
escrever(x);

se (x == 10) {
  escrever("x é igual a 10");
} senao {
  escrever("x é diferente de 10");
};
```

### Tipos de dados

- `inteiro`: números inteiros
- `real`: números reais
- `texto`: sequência de caracteres
- `booleano`: verdadeiro ou falso
- `vetor`: conjunto de valores

**Eles são inferidos automaticamente**, então não é necessário declarar o tipo de uma variável.

```cobral
declare variavel = 10; // inteiro
declare variavel = 10.5; // real
declare variavel = "texto"; // texto
declare variavel = verdadeiro; // lógico
declare variavel = falso; // lógico
declare variavel = [1, 2, 3]; // vetor
declare variavel = [[1, 2], [3, 4]]; // matriz
```

### Operadores de comparação

- `==`: igualdade
- `!=`: diferença
- `>`: maior que
- `<`: menor que
- `>=`: maior ou igual a
- `<=`: menor ou igual a

```cobral
declare x = 10 == 10; // verdadeiro
```

### Operadores lógicos

- `nao`: negação
- `e`: conjunção
- `ou`: disjunção

```cobral
declare x = verdadeiro;
declare y = falso;

declare z = nao x; // z é falso
declare w = x e y; // w é falso
declare v = x ou y; // v é verdadeiro
```

### Operadores aritméticos

- `+`: adição
- `-`: subtração
- `*`: multiplicação
- `/`: divisão
- `%`: resto da divisão

```cobral
declare x = 10 + 10; // x é 20
declare y = 20 - 10; // y é 10
declare z = 10 * 10; // z é 100
declare w = 10 / 2; // w é 5
declare v = 10 % 2; // v é 0
```

### Estruturas de controle

- `se`: executa um bloco de código se uma condição for verdadeira
- `senao`: executa um bloco de código se a condição do `se` for falsa
- `escolha-caso`: executa um bloco de código com base em uma condição

```cobral
declare x = 10 + 10;
declare y = 20;

se (x == y) { // x é igual a y
  escrever("x é igual a y");
} senao {
  escrever("x é diferente de y");
};
```

```cobral
declare x = 3;

escolha x {
  caso 1:
    escrever("x é igual a 1");
    pare;
  caso 2:
    escrever("x é igual a 2");
    pare;
  caso 3:
    escrever("x é igual a 3");
    pare;
  padrao:
    escrever("x é diferente de 1 a 10");
    pare;
};
```

### Estruturas de repetição

- `para`: executa um bloco de código um número específico de vezes
- `enquanto`: executa um bloco de código enquanto uma condição for verdadeira

```cobral
/*
 * i começa com 0 e continua até 9,
 * incrementando 1 a cada iteração
 */

para (declare i = 0; i < 10; i++) {
  escrever(i);
};
```

```cobral
declare x = 0;

enquanto (x < 10) {
  escrever(x);
  x++;
};
```

### Entrada e saída

Para entrada e saída de dados, você pode usar a biblioteca de entrada e saída de dados.

```cobral
declare x = ler("Digite seu nome:");
escrever("Você digitou: ", x);
```

### Bibliotecas

- [Entrada e saída](#entrada-e-saída): para entrada e saída de dados
- [Matemática](#matemática): para operações matemáticas
- [Conversão de tipos de dados](#conversão-de-tipos-de-dados): para conversão de tipos de dados

Você precisa importar as bibliotecas no início do seu código.

#### Matemática

Para operações matemáticas, você pode usar a biblioteca de matemática.

```cobral
importe "matematica";

declare x = raiz(9);
escrever(x); // 3
```

#### Conversão de tipos de dados

Para fazer conversão de tipos de dados, você pode usar a biblioteca de conversão.

```cobral
importe "conversao";

declare x = real(10)
escrever(x) // 10.0
```

```cobral
importe "conversao";

declare y = int(10.5)
escrever(y) // 10
```
