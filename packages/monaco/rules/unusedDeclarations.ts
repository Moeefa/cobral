import * as monaco from "monaco-editor-core";

import type { Token, Tokenizer } from "@packages/monaco/helpers/tokenizer";

import type { Scope } from "@packages/monaco/helpers/scope";

export const checkUnusedDeclarations = async (
	tokenizer: Tokenizer,
	rootScope: Scope,
): Promise<monaco.editor.IMarkerData[]> => {
	tokenizer.reset();
	const usedIdentifiers = new Set<string>();

	let token: Token | null;
	let previousToken: Token | null = null;

	// First pass: collect all used identifiers
	while (true) {
		token = tokenizer.next();
		if (!token) break;
		if (token.type === "identifier") {
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
	const checkScope = async (
		scope: Scope,
	): Promise<monaco.editor.IMarkerData[]> => {
		const scopeChecks: Promise<monaco.editor.IMarkerData[]>[] = [];

		// Check unused variables
		scope.variables.forEach((_, variable) => {
			if (!usedIdentifiers.has(variable)) {
				const variableCheck = new Promise<monaco.editor.IMarkerData[]>(
					(resolve) => {
						tokenizer.reset();
						while (true) {
							token = tokenizer.next();
							if (!token) break;
							if (token.type === "identifier" && token.value === variable) {
								resolve([
									{
										severity: monaco.MarkerSeverity.Warning,
										startLineNumber: token.line,
										endLineNumber: token.line,
										startColumn: token.column,
										endColumn: token.column + variable.length,
										message: `Variável '${variable}' é declarada mas não é usada.`,
										code: "cobral.unusedVariable",
									},
								]);
								return;
							}
						}
						resolve([]);
					},
				);
				scopeChecks.push(variableCheck);
			}
		});

		// Check unused functions
		scope.functions.forEach((_, func) => {
			if (!usedIdentifiers.has(func)) {
				const functionCheck = new Promise<monaco.editor.IMarkerData[]>(
					(resolve) => {
						tokenizer.reset();
						while (true) {
							token = tokenizer.next();
							if (!token) break;
							if (token.type === "identifier" && token.value === func) {
								resolve([
									{
										severity: monaco.MarkerSeverity.Warning,
										startLineNumber: token.line,
										endLineNumber: token.line,
										startColumn: token.column,
										endColumn: token.column + func.length,
										message: `Função '${func}' é declarada mas não é usada.`,
										code: "cobral.unusedFunction",
									},
								]);
								return;
							}
						}
						resolve([]);
					},
				);
				scopeChecks.push(functionCheck);
			}
		});

		// Process inner scopes recursively
		const innerScopeChecks = scope.innerScopes.map((innerScope) =>
			checkScope(innerScope),
		);
		scopeChecks.push(...innerScopeChecks);

		const results = await Promise.all(scopeChecks);
		return results.flat();
	};

	const markers = await checkScope(rootScope);
	return markers;
};
