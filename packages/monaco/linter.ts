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
	const markers: monaco.editor.IMarkerData[] = [];
	const text = model.getValue();
	const tokenizer = new Tokenizer(text);

	// Extract symbols from the current file
	const scopes = extractSymbols(text);

	// Run enabled linter checks
	const linterChecks = [];

	if (options.checkImports) {
		linterChecks.push(checkImportError(tokenizer, scopes));
	}

	if (options.checkUndefined) {
		linterChecks.push(checkUndefinedDeclarations(tokenizer, scopes));
	}

	if (options.checkUnused) {
		const unusedMarkers = checkUnusedDeclarations(tokenizer, scopes);
		linterChecks.push(unusedMarkers);
	}

	if (options.checkTypes) {
		const typeMarkers = checkIncompatibleComparisons(tokenizer, scopes);
		linterChecks.push(typeMarkers);
	}

	// Wait for all checks to complete and merge markers
	const results = await Promise.all(linterChecks);
	for (const result of results) {
		if (Array.isArray(result)) markers.push(...result);
	}

	return markers;
};
