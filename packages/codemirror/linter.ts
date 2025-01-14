import { type Diagnostic, linter } from "@codemirror/lint";

import { syntaxTree } from "@codemirror/language";
import type { EditorView } from "@codemirror/view";
import type { SyntaxNode, SyntaxNodeRef } from "@lezer/common";

enum Types {
	Integer = "inteiro",
	Float = "real",
	Boolean = "lógico",
	String = "cadeia",
	Unknown = "desconhecido",
}

export const variableTypes = new Map<string, string>();
const functionReturnTypes = new Map<string, string>([
	["escrever", Types.String],
	["ler", Types.String],
	["int", Types.Integer],
	["real", Types.Float],
]);

const reservedKeywords = new Set([
	"declare",
	"constante",
	"int",
	"real",
	"se",
	"senao",
	"escrever",
	"ler",
	"nao",
]);

export const cobralLinter = linter((view) => {
	const diagnostics: Diagnostic[] = [];
	const tree = syntaxTree(view.state);

	const declaredVariables: Set<string> = new Set();
	const usedVariables: Set<string> = new Set();
	const functionVariables: Map<string, Set<string>> = new Map();
	const constants: Set<string> = new Set();

	tree.cursor().iterate((node) => {
		switch (node.name) {
			case "FunctionDeclaration": {
				const functionNode = node.node.getChild("VariableDefinition");
				const funtionName = view.state.doc.sliceString(
					functionNode?.from || 0,
					functionNode?.to,
				);
				const paramListNode = node.node.getChild("ParamList");

				if (paramListNode) {
					let paramNode = paramListNode.firstChild;
					while (paramNode) {
						const paramName = view.state.doc.sliceString(
							paramNode.from,
							paramNode.to,
						);

						functionVariables.set(
							funtionName,
							functionVariables.get(funtionName)?.add(paramName) ||
								new Set([paramName]),
						);
						paramNode = paramNode.nextSibling;
					}
				}
				break;
			}

			case "VariableDeclaration": {
				const variableNode = node.node.getChild("VariableDefinition");
				const variableName = view.state.doc.sliceString(
					variableNode?.from || 0,
					variableNode?.to,
				);

				const valueNode = node.node.getChild("Expression");
				const variableValue = view.state.doc.sliceString(
					valueNode?.from || 0,
					valueNode?.to,
				);

				const callNode = valueNode?.node.getChild("VariableName");

				let inferredType = inferType(variableValue);

				if (callNode) {
					const functionName = view.state.doc.sliceString(
						callNode.from,
						callNode.to,
					);

					if (functionReturnTypes.has(functionName)) {
						inferredType =
							functionReturnTypes.get(functionName) || Types.Unknown;
					}
				}

				if (reservedKeywords.has(variableName)) {
					diagnostics.push({
						from: variableNode?.from || 0,
						to: variableNode?.to || 0,
						severity: "error",
						message: `Variável '${variableName}' é uma palavra reservada.`,
					});
				}

				if (node.node.getChild("constante")) {
					if (constants.has(variableName)) {
						diagnostics.push({
							from: variableNode?.from || 0,
							to: variableNode?.to || 0,
							severity: "error",
							message: `Variável '${variableName}' é declarada como constante e não pode ser reatribuída.`,
						});
					} else {
						constants.add(variableName);
					}
				}

				variableTypes.set(variableName, inferredType);
				declaredVariables.add(variableName);
				break;
			}

			case "VariableName": {
				const variableName = view.state.doc.sliceString(node.from, node.to);
				if (!reservedKeywords.has(variableName))
					usedVariables.add(variableName);
				break;
			}

			case "BinaryExpression": {
				const leftOperandType = node.node.lastChild
					? getTypeOfNode(node.node.lastChild, view)
					: Types.Unknown;
				const rightOperandType = node.node.lastChild
					? getTypeOfNode(node.node.lastChild, view)
					: Types.Unknown;
				if (leftOperandType !== rightOperandType) {
					diagnostics.push({
						from: node.from,
						to: node.to,
						severity: "error",
						message: `Type mismatch: '${leftOperandType}' and '${rightOperandType}' are incompatible.`,
					});
				}
				break;
			}

			case "senao": {
				if (!hasPrevious(node, "se")) {
					diagnostics.push({
						from: node.from,
						to: node.to,
						severity: "error",
						message: "'senao' sem 'se' correspondente.",
					});
				}
				break;
			}
		}
	});

	tree.cursor().iterate((node) => {
		switch (node.name) {
			case "VariableDeclaration": {
				const variableNode = node.node.getChild("VariableDefinition");
				if (variableNode) {
					const variableName = view.state.doc
						.sliceString(variableNode.from, variableNode.to)
						.trim();

					if (
						declaredVariables.has(variableName) &&
						!usedVariables.has(variableName)
					) {
						diagnostics.push({
							from: variableNode.from,
							to: variableNode.to,
							severity: "warning",
							message: `Variável '${variableName}' é declarada mas nunca usada.`,
						});
					}
				}
				break;
			}

			case "VariableName": {
				if (node.node.parent?.name === "CallExpression") {
					break; // Skip function calls
				}

				const variableName = view.state.doc.sliceString(node.from, node.to);

				// Check if any node in the parent chain is a function
				let currentNode = node.node.parent;
				while (currentNode) {
					if (currentNode.name === "FunctionDeclaration") {
						const functionNode = currentNode.getChild("VariableDefinition");
						const functionName = view.state.doc.sliceString(
							functionNode?.from || 0,
							functionNode?.to,
						);

						if (
							!functionVariables.get(functionName)?.has(variableName) &&
							!reservedKeywords.has(variableName.trim())
						) {
							diagnostics.push({
								from: node.from,
								to: node.to,
								severity: "error",
								message: `Variável '${variableName}' não declarada na função '${functionName}'.`,
							});
						}
						break;
					}

					currentNode = currentNode.parent;
				}

				if (!currentNode) {
					if (
						!declaredVariables.has(variableName) &&
						!reservedKeywords.has(variableName.trim())
					) {
						diagnostics.push({
							from: node.from,
							to: node.to,
							severity: "error",
							message: `Variável '${variableName}' não declarada.`,
						});
					}
				} else {
					break;
				}

				break;
			}
		}
	});

	return diagnostics;
});

function inferType(value: string): string {
	if (/^-?\d+$/.test(value)) {
		return Types.Integer;
	}

	if (/^-?\d+\.\d+$/.test(value)) {
		return Types.Float;
	}

	if (value === "verdadeiro" || value === "falso") {
		return Types.Boolean;
	}

	if (/^".*"$/.test(value)) {
		return Types.String;
	}

	return Types.Unknown;
}

function getTypeOfNode(node: SyntaxNode, _view: EditorView): string {
	switch (node.name) {
		case "Integer":
			return "number";
		case "Float":
			return "number";
		case "Boolean":
			return "boolean";
		case "String":
			return "string";
		default:
			return "unknown";
	}
}

function hasPrevious(node: SyntaxNodeRef, prev: string): boolean {
	let currentNode = node.node.prevSibling;

	while (currentNode) {
		if (currentNode.name === prev) return true;

		currentNode = currentNode.prevSibling;
	}

	return false;
}
