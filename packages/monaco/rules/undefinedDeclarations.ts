import * as monaco from "monaco-editor-core";

import type { Scope } from "@packages/monaco/helpers/scope";
import type { Tokenizer } from "@packages/monaco/helpers/tokenizer";

export const checkUndefinedDeclarations = async (
	tokenizer: Tokenizer,
	rootScope: Scope,
): Promise<monaco.editor.IMarkerData[]> => {
	tokenizer.reset();
	const undefinedChecks: Promise<monaco.editor.IMarkerData[]>[] = [];
	let currentScope: Scope = rootScope;
	let skipNextIdentifier = false;

	while (true) {
		const token = tokenizer.next();
		if (!token) break;

		switch (token.type) {
			case "keyword":
				if (token.value === "funcao") {
					skipNextIdentifier = true;
					const nextToken = tokenizer.next();
					if (nextToken?.type === "identifier") {
						const functionName = nextToken.value;
						const functionScope = currentScope.getInnerScopeByName(
							`function:${functionName}:${token.line}:${token.column}`,
						);
						if (functionScope) currentScope = functionScope;

						const openParen = tokenizer.next();
						if (openParen?.type === "delimiter" && openParen.value === "(") {
							while (tokenizer.next()?.value !== ")") {}
						}
					}
				} else if (token.value === "para") {
					const loopScope = currentScope.getInnerScopeByName(
						`loop:${token.line}:${token.column}`,
					);
					if (loopScope) currentScope = loopScope;
				} else if (token.value === "declare" || token.value === "constante") {
					skipNextIdentifier = true;
				}
				break;

			case "identifier":
				if (!skipNextIdentifier) {
					const identifierCheck = new Promise<monaco.editor.IMarkerData[]>(
						(resolve) => {
							if (!currentScope.isDefined(token.value)) {
								resolve([
									{
										severity: monaco.MarkerSeverity.Error,
										startLineNumber: token.line,
										endLineNumber: token.line,
										startColumn: token.column,
										endColumn: token.column + token.value.length,
										message: `Identificador '${token.value}' é usado mas não é declarado.`,
										code: "cobral.undefinedIdentifier",
									},
								]);
							} else {
								resolve([]);
							}
						},
					);
					undefinedChecks.push(identifierCheck);
				} else {
					skipNextIdentifier = false;
				}
				break;

			case "delimiter":
				if (token.value === "}") {
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

	const results = await Promise.all(undefinedChecks);
	return results.flat();
};
