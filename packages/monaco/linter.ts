import type * as monaco from "monaco-editor-core";

import { Tokenizer } from "@packages/monaco/helpers/tokenizer";
import { checkImportError } from "./rules/importError";
import { checkIncompatibleComparisons } from "@packages/monaco/rules/incompatibleComparison";
import { checkUndefinedDeclarations } from "./rules/undefinedDeclarations";
import { checkUnusedDeclarations } from "./rules/unusedDeclarations";
import { extractSymbols } from "./helpers/extractSymbols";

export interface LinterOptions {
	checkImports?: boolean;
	checkTypes?: boolean;
	checkUnused?: boolean;
	checkUndefined?: boolean;
}

export const linter = async (
	model: monaco.editor.ITextModel,
	options: LinterOptions = {
		checkImports: true,
		checkTypes: true,
		checkUnused: true,
		checkUndefined: true,
	},
) => {
	const text = model.getValue();
	const tokenizer = new Tokenizer(text);

	// Extract symbols from the current file
	const scopes = extractSymbols(text);

	// Run enabled linter checks
	const linterChecks = [];

	options.checkImports &&
		linterChecks.push(checkImportError(tokenizer, scopes));
	options.checkUndefined &&
		linterChecks.push(checkUndefinedDeclarations(tokenizer, scopes));
	options.checkUnused &&
		linterChecks.push(checkUnusedDeclarations(tokenizer, scopes));
	options.checkTypes &&
		linterChecks.push(checkIncompatibleComparisons(tokenizer, scopes));

	const results = await Promise.all(linterChecks);
	return results.flat();
};
