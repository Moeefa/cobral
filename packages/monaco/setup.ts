import * as monaco from "monaco-editor-core";

import cobral from "./cobral.json";
import { createHighlighter } from "shiki";
import { shikiToMonaco } from "@shikijs/monaco";

const shikiHighlight = async () => {
	const highlighter = await createHighlighter({
		themes: ["vitesse-dark", "vitesse-light"],
		langs: [cobral],
	});

	shikiToMonaco(highlighter, monaco);
};

shikiHighlight();

monaco.languages.register({ id: "cobral" });

monaco.languages.setMonarchTokensProvider("cobral", {
	tokenizer: {
		root: [
			[/funcao/, "keyword"],
			[
				/se|senao|escolha|caso|padrao|para|enquanto|retorne|falso|verdadeiro|nao/,
				"keyword",
			],
			[/{|}/, "delimiter.bracket"],
			[/\(|\)/, "delimiter.parenthesis"],
			[/\[|\]/, "delimiter.square"],
			[/"([^"\\]|\\.)*"/, "string"],
			[/\b\d+(\.\d+)?\b/, "number"],
			[/\/\/.*/, "comment"],
		],
	},
});

monaco.languages.setLanguageConfiguration("cobral", {
	autoClosingPairs: [
		{ open: "(", close: ")" },
		{ open: "{", close: "}" },
		{ open: "[", close: "]" },
		{ open: '"', close: '"' },
	],
	brackets: [
		["{", "}"],
		["[", "]"],
		["(", ")"],
	],
	surroundingPairs: [
		{ open: "(", close: ")" },
		{ open: "{", close: "}" },
		{ open: "[", close: "]" },
		{ open: '"', close: '"' },
	],
	indentationRules: {
		increaseIndentPattern:
			/^\s*(funcao|escolha|caso|padrao|se|para|enquanto).*{\s*$/,
		decreaseIndentPattern: /^\s*}\s*$/,
	},
	comments: {
		blockComment: ["/*", "*/"],
		lineComment: "//",
	},
});
