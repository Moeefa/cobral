import * as monaco from "monaco-editor-core";

import { reservedKeywords } from "@/lib/monaco/constants";

// Check for undefined variables and functions
export const checkUndefinedDeclarations = (
  lines: string[],
  scopes: {
    [scope: string]: { variables: Set<string>; functions: Set<string> };
  }
): monaco.editor.IMarkerData[] => {
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

  // Process each line for undefined identifiers
  lines.forEach((line, lineIndex) => {
    const sanitizedLine = line
      .replace(/\/\/.*/g, "") // Remove line comments
      .replace(/\/\*[\s\S]*?\*\//g, "") // Remove block comments
      .replace(/"(?:[^"\\]|\\.)*"/g, ""); // Remove strings

    let match;
    const identifierRegex = /\b([a-zA-Z_][a-zA-Z0-9_]*)\b/g;
    identifierRegex.lastIndex = 0; // Reset regex state

    while ((match = identifierRegex.exec(sanitizedLine)) !== null) {
      const name = match[1];

      // Ignore reserved keywords or declarations
      if (reservedKeywords.has(name)) continue;

      // Skip declared identifiers
      if (/\b(declare|funcao)\s+([a-zA-Z_][a-zA-Z0-9_]*)/g.test(line)) continue;

      // Check if the identifier is undefined
      const scope = Object.values(inheritedData).find(
        (data) => data.variables.has(name) || data.functions.has(name)
      );

      if (!scope) {
        const lineNumber = lineIndex + 1;
        const columnStart = match.index + 1;

        markers.push({
          severity: monaco.MarkerSeverity.Error,
          startLineNumber: lineNumber,
          endLineNumber: lineNumber,
          startColumn: columnStart,
          endColumn: columnStart + name.length,
          message: `Identificador '${name}' é usado mas não é declarado.`,
          code: "cobral.undefinedIdentifier",
        });
      }
    }
  });

  return markers;
};
