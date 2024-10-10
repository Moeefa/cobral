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
  }),

  snippet("declare constante ${nome} = ${valor}", {
    label: "declare constante",
    detail: "atribuição",
    type: "keyword",
  }),

  snippet("se (${}) {\n\t${}\n}", {
    label: "se",
    detail: "condicional",
    type: "keyword",
  }),

  snippet("se (${}) {\n\t${}\n} senao {\n\t${}\n}", {
    label: "se",
    detail: "/ senao condicional",
    type: "keyword",
  }),

  snippet("escrever(${})", {
    label: "escrever",
    detail: "saída",
    type: "function",
  }),

  snippet("ler(${})", {
    label: "ler",
    detail: "entrada",
    type: "function",
  }),
];
