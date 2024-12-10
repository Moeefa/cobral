import * as monaco from "monaco-editor-core";

import { Token, Tokenizer } from "@/lib/monaco/helpers/tokenizer";

import { reservedKeywords } from "@/lib/monaco/constants";

export const checkUnusedDeclarations = (
  tokenizer: Tokenizer,
  scopes: {
    [scope: string]: { variables: Set<string>; functions: Set<string> };
  }
): monaco.editor.IMarkerData[] => {
  tokenizer.reset(); // Reset tokenizer to the beginning of the text
  const markers: monaco.editor.IMarkerData[] = [];

  // Collect all used identifiers
  const usedIdentifiers = new Set<string>();
  let token: Token | null;
  let previousToken: Token | null = null;

  while ((token = tokenizer.next())) {
    // Check if the current token is an identifier and not a reserved keyword
    if (token.type === "identifier" && !reservedKeywords.has(token.value)) {
      // Check if the previous token was a "declare" or "funcao" keyword
      if (
        previousToken &&
        previousToken.type === "keyword" &&
        (previousToken.value === "declare" ||
          previousToken.value == "constante" ||
          previousToken.value === "funcao")
      ) {
        // Skip adding this identifier to usedIdentifiers as it is part of a declaration
        previousToken = token; // Update previousToken to the current token
        continue;
      }

      // Add the identifier to usedIdentifiers if it is not part of a declaration
      usedIdentifiers.add(token.value);
    }

    // Update previousToken to the current token
    previousToken = token;
  }

  // Identify unused variables and functions
  Object.entries(scopes).forEach(([_currentScope, symbols]) => {
    // Check unused variables
    symbols.variables.forEach((variable) => {
      if (usedIdentifiers.has(variable)) return;

      // Find the token where the variable was declared
      tokenizer.reset();
      while ((token = tokenizer.next())) {
        if (token.type === "identifier" && token.value === variable) {
          markers.push({
            severity: monaco.MarkerSeverity.Warning,
            startLineNumber: token.line,
            endLineNumber: token.line,
            startColumn: token.column,
            endColumn: token.column + variable.length,
            message: `Variável '${variable}' é declarada mas não é usada.`,
            code: "cobral.unusedVariable",
          });
          break;
        }
      }
    });

    // Check unused functions
    symbols.functions.forEach((func) => {
      if (usedIdentifiers.has(func)) return;

      // Find the token where the function was declared
      tokenizer.reset();
      while ((token = tokenizer.next())) {
        if (token.type === "identifier" && token.value === func) {
          markers.push({
            severity: monaco.MarkerSeverity.Warning,
            startLineNumber: token.line,
            endLineNumber: token.line,
            startColumn: token.column,
            endColumn: token.column + func.length,
            message: `Função '${func}' é declarada mas não é usada.`,
            code: "cobral.unusedFunction",
          });
          break;
        }
      }
    });
  });

  return markers;
};
