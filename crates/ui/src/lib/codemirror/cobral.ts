import {
  LRLanguage,
  LanguageSupport,
  delimitedIndent,
  foldInside,
  foldNodeProp,
  indentNodeProp,
} from "@codemirror/language";

import { highlight } from "./highlight.js";
import { localCompletionSource } from "@/lib/codemirror/completions.js";
import { parser } from "./parser.js";
import { snippets } from "@/lib/codemirror/snippets.js";

export const cobralLanguage = LRLanguage.define({
  name: "cobral",
  parser: parser.configure({
    props: [
      indentNodeProp.add({
        Block: delimitedIndent({ closing: "}", align: false }),
      }),
      foldNodeProp.add({
        Block: foldInside,
      }),
      highlight,
    ],
  }),
  languageData: {
    indentOnInput: /^\s*[{}]$/,
    closeBrackets: { brackets: ["(", "[", "{", '"'] },
    wordChars: "$",
  },
});

export function cobral() {
  return new LanguageSupport(cobralLanguage, [
    cobralLanguage.data.of({
      autocomplete: localCompletionSource,
    }),

    cobralLanguage.data.of({
      autocomplete: snippets,
    }),
  ]);
}
