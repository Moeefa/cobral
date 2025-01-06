import * as monaco from "monaco-editor-core";

import { extractImports } from "@packages/monaco/helpers/extractImports";

export const completionItemProvider =
	monaco.languages.registerCompletionItemProvider("cobral", {
		provideCompletionItems: async (model, position) => {
			const word = model.getWordUntilPosition(position);
			const range = {
				startLineNumber: position.lineNumber,
				endLineNumber: position.lineNumber,
				startColumn: word.startColumn,
				endColumn: word.endColumn,
			};

			const textBeforeCursor = model.getValueInRange({
				startLineNumber: 1,
				startColumn: 1,
				endLineNumber: position.lineNumber,
				endColumn: position.column,
			});

			/*
			 * Static suggestions
			 */
			const suggestions: monaco.languages.CompletionItem[] = [
				{
					label: "declare",
					kind: monaco.languages.CompletionItemKind.Keyword,
					insertText: "declare ${1:variavel} = ${2:valor};",
					insertTextRules:
						monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
					documentation: "Declara uma nova variável.",
					range: range,
				},
				{
					label: "funcao",
					kind: monaco.languages.CompletionItemKind.Keyword,
					insertText: "funcao ${1:nome}() {\n\t$0\n};",
					insertTextRules:
						monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
					documentation: "Define uma nova função.",
					range: range,
				},
				{
					label: "se",
					kind: monaco.languages.CompletionItemKind.Keyword,
					insertText: "se (${1}) {\n\t$0\n};",
					insertTextRules:
						monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
					documentation: "Condicional.",
					range: range,
				},
				{
					label: "senao",
					kind: monaco.languages.CompletionItemKind.Keyword,
					insertText: "senao {\n\t$0\n};",
					insertTextRules:
						monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
					documentation: "Senão para blocos condicionais.",
					range: range,
				},
				{
					label: "para",
					kind: monaco.languages.CompletionItemKind.Keyword,
					insertText:
						"para (${1:inicializacao}; ${2:condicao}; ${3:atualizacao}) {\n\t$0\n};",
					insertTextRules:
						monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
					documentation: "Loop para.",
					range: range,
				},
				{
					label: "retorne",
					kind: monaco.languages.CompletionItemKind.Keyword,
					insertText: "retorne ${1:valor};",
					insertTextRules:
						monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
					documentation: "Retorna o valor de uma função.",
					range: range,
				},
			];

			/*
			 * Dynamic suggestions
			 */
			const keywords = [
				{
					regex: /funcao\s+([\p{L}_][\p{L}0-9_]*)\s*\(/gu,
					kind: monaco.languages.CompletionItemKind.Function,
					documentation: "Função declarada.",
				},
				{
					regex: /\bdeclare\s+([\p{L}_][\p{L}0-9_]*)/gu,
					kind: monaco.languages.CompletionItemKind.Variable,
					documentation: "Variável declarada.",
				},
			];

			for (const keyword of keywords) {
				let match: RegExpExecArray | null;
				while (true) {
					match = keyword.regex.exec(textBeforeCursor);
					if (match === null) break;
					suggestions.push({
						label: match[1],
						kind: keyword.kind,
						insertText: match[1],
						range: range,
						detail: match[0],
						documentation: {
							isTrusted: true,
							value: `${keyword.documentation}`,
						},
					});
				}
			}

			const symbols = await extractImports(model.getValue());

			symbols.variables.forEach((_, variable) => {
				suggestions.push({
					label: variable,
					kind: monaco.languages.CompletionItemKind.Variable,
					insertText: variable,
					range: range,
					documentation: "Variável importada.",
				});
			});

			symbols.functions.forEach((_, func) => {
				suggestions.push({
					label: func,
					kind: monaco.languages.CompletionItemKind.Function,
					insertText: `${func}($0)`,
					insertTextRules:
						monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
					range: range,
					documentation: {
						isTrusted: true,
						value: `Função importada.\n\`\`\`cobral\n\n${func}\n\`\`\``,
					},
				});
			});

			return { suggestions };
		},
	});
