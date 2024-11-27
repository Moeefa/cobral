import * as monaco from "monaco-editor-core";

import cobral from "./cobral.json";
import { createHighlighter } from "shiki";
import { shikiToMonaco } from "@shikijs/monaco";

const shikiHighlight = async () => {
  const highlighter = await createHighlighter({
    themes: ["vitesse-dark", "vitesse-light"],
    langs: [cobral],
  });

  shikiToMonaco(highlighter, monaco);
};

shikiHighlight();

monaco.languages.register({ id: "cobral" });

monaco.languages.setMonarchTokensProvider("cobral", {
  tokenizer: {
    root: [
      [/funcao/, "keyword"],
      [/se|senao|para|enquanto|retorne/, "keyword"],
      [/{|}/, "delimiter.bracket"],
      [/\(|\)/, "delimiter.parenthesis"],
      [/\[|\]/, "delimiter.square"],
      [/"([^"\\]|\\.)*"/, "string"],
      [/\b\d+(\.\d+)?\b/, "number"],
      [/\/\/.*/, "comment"],
    ],
  },
});

monaco.languages.setLanguageConfiguration("cobral", {
  autoClosingPairs: [
    { open: "(", close: ")" },
    { open: "{", close: "}" },
    { open: "[", close: "]" },
    { open: '"', close: '"' },
  ],
  brackets: [
    ["{", "}"],
    ["[", "]"],
    ["(", ")"],
  ],
  surroundingPairs: [
    { open: "(", close: ")" },
    { open: "{", close: "}" },
    { open: "[", close: "]" },
    { open: '"', close: '"' },
  ],
  indentationRules: {
    increaseIndentPattern: /^\s*(funcao|se|para|enquanto).*{\s*$/,
    decreaseIndentPattern: /^\s*}\s*$/,
  },
  comments: {
    blockComment: ["/*", "*/"],
    lineComment: "//",
  },
});

// monaco.languages.registerHoverProvider("cobral", {
//   provideHover: (model, position) => {
//     return {
//       range: new monaco.Range(
//         position.lineNumber,
//         position.column,
//         position.lineNumber,
//         position.column
//       ),
//       contents: [{ value: "Hovering over" }],
//     };
//   },
// });
