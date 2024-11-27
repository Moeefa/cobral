import * as monaco from "monaco-editor-core";

import { reservedKeywords } from "@/lib/monaco/constants";

export const checkUnusedDeclarations = (
  lines: string[],
  scopes: {
    [scope: string]: { variables: Set<string>; functions: Set<string> };
  }
): monaco.editor.IMarkerData[] => {
  const markers: monaco.editor.IMarkerData[] = [];

  // Preprocess lines to remove comments and strings
  const sanitizedLines = lines.map(
    (line) =>
      line
        .replace(/\/\/.*/g, "") // Remove line comments
        .replace(/\/\*[\s\S]*?\*\//g, "") // Remove block comments
        .replace(/"(?:[^"\\]|\\.)*"/g, "") // Remove strings
  );

  // Collect all used identifiers
  const usedIdentifiers = new Set<string>();
  sanitizedLines.forEach((line) => {
    const identifierRegex = /\b([a-zA-Z_][a-zA-Z0-9_]*)\b/g;
    identifierRegex.lastIndex = 0; // Reset regex state

    let match;
    while ((match = identifierRegex.exec(line)) !== null) {
      const name = match[1];
      if (
        !reservedKeywords.has(name) &&
        !/\b(declare|funcao)\s+([a-zA-Z_][a-zA-Z0-9_]*)/g.test(line)
      ) {
        usedIdentifiers.add(name);
      }
    }
  });

  // Identify unused variables and functions
  Object.entries(scopes).forEach(([_currentScope, symbols]) => {
    symbols.variables.forEach((variable) => {
      if (usedIdentifiers.has(variable)) return;

      // Find where the variable was declared
      const lineIndex = sanitizedLines.findIndex((line) =>
        new RegExp(`\\b${variable}\\b`).test(line)
      );

      if (lineIndex !== -1) {
        const lineNumber = lineIndex + 1;
        const columnStart =
          sanitizedLines[lineIndex]?.indexOf(variable) + 1 || 0;

        markers.push({
          severity: monaco.MarkerSeverity.Warning,
          startLineNumber: lineNumber,
          endLineNumber: lineNumber,
          startColumn: columnStart,
          endColumn: columnStart + variable.length,
          message: `Variável '${variable}' é declarada mas não é usada.`,
          code: "cobral.unusedVariable",
        });
      }
    });

    symbols.functions.forEach((func) => {
      if (usedIdentifiers.has(func)) return;

      // Find where the function was declared
      const lineIndex = sanitizedLines.findIndex((line) =>
        new RegExp(`\\b${func}\\b`).test(line)
      );

      if (lineIndex !== -1) {
        const lineNumber = lineIndex + 1;
        const columnStart = sanitizedLines[lineIndex]?.indexOf(func) + 1 || 0;

        markers.push({
          severity: monaco.MarkerSeverity.Warning,
          startLineNumber: lineNumber,
          endLineNumber: lineNumber,
          startColumn: columnStart,
          endColumn: columnStart + func.length,
          message: `Função '${func}' é declarada mas não é usada.`,
          code: "cobral.unusedFunction",
        });
      }
    });
  });

  return markers;
};
