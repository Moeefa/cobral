import { Scope } from "@packages/monaco/helpers/scope";
import { Tokenizer } from "@packages/monaco/helpers/tokenizer";
import { extractSymbols } from "@packages/monaco/helpers/extractSymbols";
import { readTextFile } from "@tauri-apps/plugin-fs";

export const extractImports = async (text: string): Promise<Scope> => {
	const tokenizer = new Tokenizer(text);
	const importedScope = new Scope("imported"); // Create a scope for imported symbols

	while (true) {
		const token = tokenizer.next();
		if (!token) break;
		if (token.type === "keyword" && token.value === "importe") {
			// Get the string literal for the import path
			const nextToken = tokenizer.next();
			if (nextToken?.type === "string") {
				const importPath = nextToken.value.slice(1, -1); // Remove quotes

				try {
					// Read and parse the imported file
					const importedContent = await readTextFile(importPath);
					const importedScopes = extractSymbols(importedContent);

					// Merge imported global scope symbols into our imported scope
					const importedGlobal = importedScopes;

					// Merge variables with their types and parameter status
					for (const [key, value] of importedGlobal.variables) {
						importedScope.addVariable(key, value.type);
					}

					// Merge functions with their parameters and return types
					for (const [key, value] of importedGlobal.functions) {
						importedScope.addFunction(key, value.parameters, value.returnType);
					}

					// Also check for and merge any function scopes that might be relevant
					for (const [scopeName, scope] of Object.entries(importedScopes)) {
						if (scopeName.startsWith("function:")) {
							const funcName = scopeName.split(":")[1];
							const funcInfo = scope.functions.get(funcName);
							if (funcInfo) {
								importedScope.addFunction(
									funcName,
									funcInfo.parameters,
									funcInfo.returnType,
								);
							}
						}
					}
				} catch (error) {
					console.error(
						`Error processing import at line ${token.line}:`,
						error,
					);

					// Add an empty function/variable entry to prevent undefined errors
					importedScope.addVariable(importPath, null);
				}
			}
		}
	}

	return importedScope;
};
