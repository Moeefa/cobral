import * as monaco from "monaco-editor-core";

import type { Scope } from "@packages/monaco/helpers/scope";
import type { Tokenizer } from "@packages/monaco/helpers/tokenizer";
import { exists } from "@tauri-apps/plugin-fs";

export const checkImportError = async (
	tokenizer: Tokenizer,
	rootScope: Scope, // Added rootScope parameter for consistency
): Promise<monaco.editor.IMarkerData[]> => {
	tokenizer.reset();
	const markers: monaco.editor.IMarkerData[] = [];
	let currentScope: Scope = rootScope;

	while (true) {
		const token = tokenizer.next();
		if (!token) break;

		switch (token.type) {
			case "keyword":
				if (token.value === "importe") {
					const pathToken = tokenizer.next();
					if (pathToken?.type === "string") {
						const importPath = pathToken.value.slice(1, -1); // Remove quotes
						try {
							if (await exists(importPath)) continue;

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
				break;

			case "delimiter":
				if (token.value === "{") {
					const blockScope = currentScope.getInnerScopeByName(
						`block:${token.line}:${token.column}`,
					);
					if (blockScope) {
						currentScope = blockScope;
					}
				} else if (token.value === "}") {
					currentScope = currentScope.parentScope || rootScope;
				}
				break;
		}
	}

	return markers;
};
