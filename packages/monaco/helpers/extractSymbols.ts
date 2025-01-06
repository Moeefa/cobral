import { Scope } from "@packages/monaco/helpers/scope";
import { Tokenizer } from "@packages/monaco/helpers/tokenizer";

export const extractSymbols = (text: string) => {
	const globalScope = new Scope();
	const scopeStack: Scope[] = [globalScope];
	let currentScope: Scope = globalScope;

	const tokenizer = new Tokenizer(text);

	let currentFunctionName: string | null = null;
	let currentParameters: string[] = [];

	while (true) {
		const token = tokenizer.next();
		if (!token) break;

		if (token.type === "keyword") {
			switch (token.value) {
				case "para": {
					// Create a loop scope
					const loopScope = currentScope.addInnerScope(
						`loop:${token.line}:${token.column}`,
					);
					scopeStack.push(currentScope);
					currentScope = loopScope;

					// Handle loop initialization
					const openParen = tokenizer.next();
					if (openParen?.type === "delimiter" && openParen.value === "(") {
						let initToken = tokenizer.next();
						while (initToken && initToken.value !== ";") {
							if (initToken.type === "keyword") {
								if (
									initToken.value === "declare" ||
									initToken.value === "constante"
								) {
									let nextToken = tokenizer.next();
									if (
										initToken.value === "declare" &&
										nextToken?.value === "constante"
									) {
										nextToken = tokenizer.next();
									}
									if (nextToken?.type === "identifier") {
										currentScope.addVariable(nextToken.value);
									}
								}
							}
							initToken = tokenizer.next();
						}

						// Skip condition and increment parts
						let depth = 1;
						while (depth > 0) {
							const headerToken = tokenizer.next();
							if (!headerToken) break;
							if (headerToken.type === "delimiter") {
								if (headerToken.value === "(") depth++;
								if (headerToken.value === ")") depth--;
							}
						}
					}
					break;
				}

				case "funcao": {
					const nextToken = tokenizer.next();
					if (nextToken?.type === "identifier") {
						currentFunctionName = nextToken.value;
						currentParameters = [];

						const functionScope = currentScope.addInnerScope(
							`function:${currentFunctionName}:${token.line}:${token.column}`,
						);

						// Parse parameter list
						const openParen = tokenizer.next();
						if (openParen?.type === "delimiter" && openParen.value === "(") {
							let paramToken = tokenizer.next();
							while (paramToken && paramToken.value !== ")") {
								if (paramToken.type === "identifier") {
									currentParameters.push(paramToken.value);
									functionScope.addVariable(paramToken.value);
								}
								paramToken = tokenizer.next();
							}
						}

						// Add function to current scope
						currentScope.addFunction(currentFunctionName, currentParameters);

						scopeStack.push(currentScope);
						currentScope = functionScope;
					}
					break;
				}

				case "declare":
				case "constante": {
					const isConstant = token.value === "constante";
					let nextToken = tokenizer.next();

					// Skip "constante" token if we started with "declare"
					if (!isConstant && nextToken?.value === "constante") {
						nextToken = tokenizer.next();
					}

					if (nextToken?.type === "identifier") {
						// Look ahead for type annotation
						let type: string | null = null;
						const peekToken = tokenizer.peek();
						if (peekToken?.type === "identifier") {
							type = peekToken.value;
							tokenizer.next(); // Consume the type token
						}

						currentScope.addVariable(nextToken.value, type);
					}
					break;
				}
			}
		} else if (token.type === "delimiter") {
			if (token.value === "}") {
				// Only pop scope if we're in a function or loop scope
				if (
					currentScope.name.startsWith("function:") ||
					currentScope.name.startsWith("loop:")
				) {
					currentScope = scopeStack.pop() || globalScope;
				}
			}
		}
	}

	return globalScope;
};
