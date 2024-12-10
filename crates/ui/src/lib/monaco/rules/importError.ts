import * as monaco from "monaco-editor-core";

import { Token, Tokenizer } from "@/lib/monaco/helpers/tokenizer";

import { exists } from "@tauri-apps/plugin-fs";

export const checkImportError = async (
  tokenizer: Tokenizer
): Promise<monaco.editor.IMarkerData[]> => {
  tokenizer.reset(); // Reset tokenizer to the beginning of the text
  const markers: monaco.editor.IMarkerData[] = [];

  // Iterate through tokens to find and validate "importe" statements
  let token: Token | null;
  while ((token = tokenizer.next())) {
    if (token.type === "keyword" && token.value === "importe") {
      const pathToken = tokenizer.next();
      if (pathToken && pathToken.type === "string") {
        const importPath = pathToken.value.slice(1, -1); // Remove quotes around the string
        try {
          if (await exists(importPath)) continue;

          // Create an error marker for invalid import paths
          markers.push({
            severity: monaco.MarkerSeverity.Error,
            startLineNumber: token.line,
            endLineNumber: token.line,
            startColumn: pathToken.column + 1,
            endColumn: pathToken.column + pathToken.value.length - 1,
            message: `Erro ao carregar o arquivo: "${importPath}". Verifique o caminho ou as permissões.`,
            code: "cobral.importError",
          });
        } catch (error) {
          markers.push({
            severity: monaco.MarkerSeverity.Error,
            startLineNumber: token.line,
            endLineNumber: token.line,
            startColumn: pathToken.column + 1,
            endColumn: pathToken.column + pathToken.value.length - 1,
            message: `Erro ao carregar o arquivo: "${importPath}". Verifique o caminho ou as permissões.`,
            code: "cobral.importError",
          });
        }
      }
    }
  }

  return markers;
};
