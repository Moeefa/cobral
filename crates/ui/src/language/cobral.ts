import {
  LRLanguage,
  LanguageSupport,
  delimitedIndent,
  foldInside,
  foldNodeProp,
  indentNodeProp,
} from "@codemirror/language";

import { highlight } from "./highlight.js";
import { localCompletionSource } from "@/language/completions.js";
import { parser } from "./parser.js";
import { snippets } from "@/language/snippets.js";

export const cobralLanguage = LRLanguage.define({
  name: "cobral",
  parser: parser.configure({
    props: [
      indentNodeProp.add({
        Application: delimitedIndent({ closing: ")", align: false }),
      }),
      foldNodeProp.add({
        Application: foldInside,
      }),
      highlight,
    ],
  }),
  languageData: {
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
