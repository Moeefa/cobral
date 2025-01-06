import * as monaco from "monaco-editor-core";

import type { Token, Tokenizer } from "@packages/monaco/helpers/tokenizer";

import type { Scope } from "@packages/monaco/helpers/scope";
import { reservedKeywords } from "@packages/monaco/constants";

export const checkUnusedDeclarations = (
	tokenizer: Tokenizer,
	rootScope: Scope,
): monaco.editor.IMarkerData[] => {
	tokenizer.reset();
	const markers: monaco.editor.IMarkerData[] = [];
	const usedIdentifiers = new Set<string>();

	let token: Token | null;
	let previousToken: Token | null = null;

	// First pass: collect all used identifiers
	while (true) {
		token = tokenizer.next();
		if (!token) break;
		if (token.type === "identifier" && !reservedKeywords.has(token.value)) {
			if (
				previousToken &&
				previousToken.type === "keyword" &&
				(previousToken.value === "declare" ||
					previousToken.value === "constante" ||
					previousToken.value === "funcao")
			) {
				previousToken = token;
				continue;
			}
			usedIdentifiers.add(token.value);
		}
		previousToken = token;
	}

	// Check for unused declarations in a scope
	const checkScope = (scope: Scope) => {
		// Check unused variables
		scope.variables.forEach((_, variable) => {
			if (!usedIdentifiers.has(variable)) {
				// Find declaration token
				tokenizer.reset();
				while (true) {
					token = tokenizer.next();
					if (!token) break;
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
			}
		});

		// Check unused functions
		scope.functions.forEach((_, func) => {
			if (!usedIdentifiers.has(func)) {
				// Find declaration token
				tokenizer.reset();
				while (true) {
					token = tokenizer.next();
					if (!token) break;
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
			}
		});

		// Process inner scopes recursively
		for (const innerScope of scope.innerScopes) {
			checkScope(innerScope);
		}
	};

	// Start checking from root scope
	checkScope(rootScope);
	return markers;
};
