import {
  Completion,
  snippetCompletion as snippet,
} from "@codemirror/autocomplete";

/// A collection of JavaScript-related
/// [snippets](#autocomplete.snippet).
export const snippets: readonly Completion[] = [
  snippet("declare ${nome} = ${valor}", {
    label: "declare",
    detail: "atribuição",
    type: "keyword",
    section: "Declaração de Variáveis",
    info: "Declara uma variável.",
  }),

  snippet("declare constante ${nome} = ${valor}", {
    label: "declare constante",
    detail: "atribuição",
    type: "keyword",
    section: "Declaração de Variáveis",
    info: "Declara uma constante.",
  }),

  snippet("se (${}) {\n\t${}\n}", {
    label: "se",
    detail: "condicional",
    type: "keyword",
    section: "Estruturas de Controle",
    info: "Executa um bloco de código se a condição for verdadeira.",
  }),

  snippet("se (${}) {\n\t${}\n} senao {\n\t${}\n}", {
    label: "se",
    detail: "/ senao condicional",
    type: "keyword",
    section: "Estruturas de Controle",
    info: "Executa um bloco de código se a condição for verdadeira, senão executa outro bloco.",
  }),

  snippet("escrever(${})", {
    label: "escrever",
    detail: "saída",
    type: "function",
    section: "Entrada/Saída",
    info: "Escreve um valor na saída padrão.",
  }),

  snippet("ler(${})", {
    label: "ler",
    detail: "entrada",
    type: "function",
    section: "Entrada/Saída",
    info: "Lê um valor da entrada padrão.",
  }),

  snippet("raiz(${})", {
    label: "raiz",
    detail: "matemática",
    type: "function",
    section: "Matemática",
    info: "Calcula a raiz quadrada de um número.",
  }),

  snippet("potencia(${})", {
    label: "potencia",
    detail: "matemática",
    type: "function",
    section: "Matemática",
    info: "Calcula a potência de um número.",
  }),

  snippet("int(${})", {
    label: "int",
    detail: "conversão",
    type: "function",
    section: "Conversão de Tipos",
    info: "Converte um valor para inteiro.",
  }),

  snippet("real(${})", {
    label: "real",
    detail: "conversão",
    type: "function",
    section: "Conversão de Tipos",
    info: "Converte um valor para real.",
  }),
];
