import * as monaco from "monaco-editor-core";

import { exists } from "@tauri-apps/plugin-fs";

export const checkImportError = async (lines: string[]) => {
  const markers: monaco.editor.IMarkerData[] = [];

  for (const [index, line] of lines.entries()) {
    const importMatch = /importe\s+"([^"]+)"/g.exec(line);
    if (!importMatch) continue;

    const importPath = importMatch[1];
    try {
      if (await exists(importPath)) continue;

      markers.push({
        severity: monaco.MarkerSeverity.Error,
        startLineNumber: index + 1,
        endLineNumber: index + 1,
        startColumn: line.indexOf(importPath) + 1,
        endColumn: line.indexOf(importPath) + importPath.length + 1,
        message: `Erro ao carregar o arquivo: "${importPath}". Verifique o caminho ou as permissões.`,
        code: "cobral.importError",
      });
    } catch (error) {
      markers.push({
        severity: monaco.MarkerSeverity.Error,
        startLineNumber: index + 1,
        endLineNumber: index + 1,
        startColumn: line.indexOf(importPath) + 1,
        endColumn: line.indexOf(importPath) + importPath.length + 1,
        message: `Erro ao carregar o arquivo: "${importPath}". Verifique o caminho ou as permissões.`,
        code: "cobral.importError",
      });
    }
  }

  return markers;
};
