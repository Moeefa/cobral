import * as monaco from "monaco-editor-core";

import { Token, Tokenizer } from "@/lib/monaco/helpers/tokenizer";

import { reservedKeywords } from "@/lib/monaco/constants";

export const checkUndefinedDeclarations = (
  tokenizer: Tokenizer,
  scopes: {
    [scope: string]: { variables: Set<string>; functions: Set<string> };
  }
): monaco.editor.IMarkerData[] => {
  tokenizer.reset(); // Reset tokenizer to the beginning of the text
  const markers: monaco.editor.IMarkerData[] = [];

  // Pre-calculate inherited variables and functions for each scope
  const inheritedData: {
    [scope: string]: { variables: Set<string>; functions: Set<string> };
  } = {};

  Object.entries(scopes).forEach(([currentScope, symbols]) => {
    const inheritedVariables = new Set<string>(symbols.variables);
    const inheritedFunctions = new Set<string>(symbols.functions);

    Object.entries(scopes).forEach(([parentScope, parentSymbols]) => {
      if (
        parentScope === "global" || // Always inherit from global
        currentScope.startsWith(parentScope) // Or from parent scopes
      ) {
        parentSymbols.variables.forEach((v) => inheritedVariables.add(v));
        parentSymbols.functions.forEach((f) => inheritedFunctions.add(f));
      }
    });

    inheritedData[currentScope] = {
      variables: inheritedVariables,
      functions: inheritedFunctions,
    };
  });

  // Tokenize the entire input text for processing
  let token: Token | null;

  // Process tokens for undefined identifiers
  while ((token = tokenizer.next())) {
    const { type, value, line, column } = token;

    if (type !== "identifier") continue; // Skip non-identifiers
    if (reservedKeywords.has(value)) continue; // Skip reserved keywords

    const isDefined = Object.values(inheritedData).some(
      (data) => data.variables.has(value) || data.functions.has(value)
    );

    if (!isDefined) {
      markers.push({
        severity: monaco.MarkerSeverity.Error,
        startLineNumber: line,
        endLineNumber: line,
        startColumn: column,
        endColumn: column + value.length,
        message: `Identificador '${value}' é usado mas não é declarado.`,
        code: "cobral.undefinedIdentifier",
      });
    }
  }

  return markers;
};
