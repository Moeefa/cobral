import * as monaco from "monaco-editor-core";

import type { Token, Tokenizer } from "@packages/monaco/helpers/tokenizer";

import type { Scope } from "@packages/monaco/helpers/scope";

const builtInFunctions: {
	[name: string]: { args: string[]; returnType: string };
} = {
	ler: { args: ["cadeia"], returnType: "cadeia" },
	int: { args: ["cadeia"], returnType: "inteiro" },
	real: { args: ["inteiro"], returnType: "real" },
};

export const checkIncompatibleComparisons = (
	tokenizer: Tokenizer,
	rootScope: Scope,
): monaco.editor.IMarkerData[] => {
	tokenizer.reset();
	const markers: monaco.editor.IMarkerData[] = [];
	let currentScope: Scope = rootScope;
	let previousToken: Token | null = null;
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

						if (functionScope) {
							currentScope = functionScope;
						}

						// Skip function parameters
						const openParen = tokenizer.next();
						if (openParen?.type === "delimiter" && openParen.value === "(") {
							while (tokenizer.next()?.value !== ")") {
								// Skip tokens until closing parenthesis
							}
						}
					}
				} else if (token.value === "declare" || token.value === "constante") {
					skipNextIdentifier = true;
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

			default:
				if (["==", "!=", "<", ">", "<=", ">="].includes(token.value)) {
					if (
						!previousToken ||
						!["identifier", "string", "number"].includes(previousToken.type)
					) {
						markers.push({
							severity: monaco.MarkerSeverity.Error,
							startLineNumber: token.line,
							endLineNumber: token.line,
							startColumn: token.column,
							endColumn: token.column + token.value.length,
							message: "Operação de comparação incompleta.",
							code: "cobral.incompleteComparison",
						});
						break;
					}

					const leftOperandToken = previousToken;
					const rightOperandToken = tokenizer.next();

					if (
						!rightOperandToken ||
						!["identifier", "string", "number"].includes(rightOperandToken.type)
					) {
						markers.push({
							severity: monaco.MarkerSeverity.Error,
							startLineNumber: token.line,
							endLineNumber: token.line,
							startColumn: leftOperandToken.column,
							endColumn: token.column + token.value.length,
							message: "Operação de comparação incompleta.",
							code: "cobral.incompleteComparison",
						});
						break;
					}

					const leftType = resolveType(leftOperandToken.value, currentScope);
					const rightType = resolveType(rightOperandToken.value, currentScope);

					if (
						(leftType === "inteiro" && rightType === "real") ||
						(leftType === "real" && rightType === "inteiro")
					) {
						break;
					}

					if (leftType && rightType && leftType !== rightType) {
						markers.push({
							severity: monaco.MarkerSeverity.Error,
							startLineNumber: token.line,
							endLineNumber: token.line,
							startColumn: leftOperandToken.column,
							endColumn:
								rightOperandToken.column + rightOperandToken.value.length,
							message: `Comparação incompatível: '${leftOperandToken.value}' (${leftType}) e '${rightOperandToken.value}' (${rightType}) não podem ser comparados.`,
							code: "cobral.incompatibleComparison",
						});
					}
				}
				break;
		}

		if (!skipNextIdentifier) {
			previousToken = token;
		} else {
			skipNextIdentifier = false;
		}
	}

	return markers;
};

const resolveType = (name: string, scope: Scope): string | null => {
	if (/^".*"$|^'.*'$/.test(name)) return "cadeia";
	if (/^\d+$/.test(name)) return "inteiro";
	if (/^\d+\.\d+$/.test(name)) return "real";
	if (/^(verdadeiro|falso)$/.test(name)) return "lógico";

	if (builtInFunctions[name]) {
		return builtInFunctions[name].returnType;
	}

	let currentScope: Scope | null = scope;
	while (currentScope) {
		const variable = currentScope.variables.get(name);
		if (variable) {
			return variable.type;
		}
		currentScope = currentScope.parentScope;
	}

	return null;
};
