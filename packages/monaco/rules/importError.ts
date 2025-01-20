import * as monaco from "monaco-editor-core";

import type { Token, Tokenizer } from "@packages/monaco/helpers/tokenizer";

import type { Scope } from "@packages/monaco/helpers/scope";
import { exists } from "@tauri-apps/plugin-fs";

const createImportError = (
	path: string,
	pathToken: Token,
	token: Token,
): monaco.editor.IMarkerData => {
	return {
		severity: monaco.MarkerSeverity.Error,
		startLineNumber: token.line,
		endLineNumber: token.line,
		startColumn: pathToken.column + 1,
		endColumn: pathToken.column + pathToken.value.length - 1,
		message: `Erro ao carregar o arquivo: "${path}". Verifique o caminho ou as permissões.`,
		code: "cobral.importError",
	};
};

export const checkImportError = async (
	tokenizer: Tokenizer,
	_rootScope: Scope,
): Promise<monaco.editor.IMarkerData[]> => {
	tokenizer.reset();
	const importChecks: Promise<monaco.editor.IMarkerData[]>[] = [];

	// First pass: collect all import statements
	while (true) {
		const token = tokenizer.next();
		if (!token) break;

		if (token.type === "keyword" && token.value === "importe") {
			const pathToken = tokenizer.peek();
			if (pathToken?.type === "string") {
				tokenizer.next(); // consume the string token
				const importPath = pathToken.value.slice(1, -1);

				// Create a Promise for checking this import
				if (["matematica", "conversao"].includes(importPath)) return [];
				const checkPromise = exists(importPath)
					.then((fileExists) => {
						if (!fileExists) {
							return [createImportError(importPath, pathToken, token)];
						}
						return [];
					})
					.catch(() => {
						return [createImportError(importPath, pathToken, token)];
					});

				importChecks.push(checkPromise);
			}
		}
	}

	// Wait for all import checks to complete
	const results = await Promise.all(importChecks);
	return results.flat();
};
