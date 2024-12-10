import * as monaco from "monaco-editor-core";

import { Tokenizer } from "@/lib/monaco/helpers/tokenizer";
import { checkImportError } from "./rules/importError";
import { checkIncompatibleComparisons } from "@/lib/monaco/rules/incompatibleComparison";
import { checkUndefinedDeclarations } from "./rules/undefinedDeclarations";
import { checkUnusedDeclarations } from "./rules/unusedDeclarations";
import { extractImports } from "@/lib/monaco/helpers/extractImports";
import { extractSymbols } from "./helpers/extractSymbols";

export const linter = async (model: monaco.editor.ITextModel) => {
  const markers: monaco.editor.IMarkerData[] = [];
  const text = model.getValue();
  const lines = text.split("\n");
  const tokenizer = new Tokenizer(text);

  const symbols = extractSymbols(text);
  const imports = await extractImports(text);

  symbols.global = {
    variables: new Set([
      ...symbols.global.variables,
      ...imports.global.variables,
    ]),
    functions: new Set([
      ...symbols.global.functions,
      ...imports.global.functions,
    ]),
  };

  markers.push(
    ...(await checkImportError(tokenizer)),
    ...checkUndefinedDeclarations(tokenizer, symbols),
    ...checkUnusedDeclarations(tokenizer, symbols),
    ...checkIncompatibleComparisons(tokenizer, symbols)
  );

  return markers;
};
