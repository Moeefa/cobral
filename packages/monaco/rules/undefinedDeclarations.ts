import * as monaco from "monaco-editor-core";

import type { Scope } from "@packages/monaco/helpers/scope";
import type { Tokenizer } from "@packages/monaco/helpers/tokenizer";

export const checkUndefinedDeclarations = (
	tokenizer: Tokenizer,
	rootScope: Scope,
): monaco.editor.IMarkerData[] => {
	tokenizer.reset();
	const markers: monaco.editor.IMarkerData[] = [];
	let currentScope: Scope = rootScope;
	let skipNextIdentifier = false;

	while (true) {
		const token = tokenizer.next();
		if (!token) break;

		switch (token.type) {
			case "keyword":
				if (token.value === "funcao") {
					skipNextIdentifier = true; // Skip the function name itself
					const nextToken = tokenizer.next();
					if (nextToken?.type === "identifier") {
						const functionName = nextToken.value;
						const functionScope = currentScope.getInnerScopeByName(
							`function:${functionName}:${token.line}:${token.column}`,
						);

						if (functionScope) {
							currentScope = functionScope;
						}

						// Skip function parameters parsing since they are already defined
						const openParen = tokenizer.next();
						if (openParen?.type === "delimiter" && openParen.value === "(") {
							while (tokenizer.next()?.value !== ")") {
								// Skip tokens until closing parenthesis
							}
						}
					}
				} else if (token.value === "para") {
					// Handle loop scope
					const loopScope = currentScope.getInnerScopeByName(
						`loop:${token.line}:${token.column}`,
					);
					if (loopScope) {
						currentScope = loopScope;
					}
				} else if (token.value === "declare" || token.value === "constante") {
					skipNextIdentifier = true; // Skip the variable being declared
				}
				break;

			case "identifier":
				if (!skipNextIdentifier) {
					// Only check identifiers that aren't being declared
					if (!currentScope.isDefined(token.value)) {
						markers.push({
							severity: monaco.MarkerSeverity.Error,
							startLineNumber: token.line,
							endLineNumber: token.line,
							startColumn: token.column,
							endColumn: token.column + token.value.length,
							message: `Identificador '${token.value}' é usado mas não é declarado.`,
							code: "cobral.undefinedIdentifier",
						});
					}
				} else {
					skipNextIdentifier = false;
				}
				break;

			case "delimiter":
				if (token.value === "}") {
					// Only pop scope if we're in a function or loop scope
					if (
						currentScope.name.startsWith("function:") ||
						currentScope.name.startsWith("loop:")
					) {
						currentScope = currentScope.parentScope || rootScope;
					}
				}
				break;
		}
	}

	return markers;
};
