export class Scope {
	name: string;
	variables: Map<string, { type: string | null }>;
	functions: Map<string, { parameters: string[]; returnType: string | null }>;
	parentScope: Scope | null;
	innerScopes: Scope[];

	constructor(name = "global", parentScope: Scope | null = null) {
		this.name = name;
		this.variables = new Map();
		this.functions = new Map([
			["escrever", { parameters: ["mensagem"], returnType: "cadeia" }],
			["ler", { parameters: [], returnType: "cadeia" }],
			["raiz", { parameters: ["numero"], returnType: "inteiro" }],
			["potencia", { parameters: ["base", "expoente"], returnType: "inteiro" }],
			["int", { parameters: ["numero"], returnType: "inteiro" }],
			["real", { parameters: ["numero"], returnType: "real" }],
		]);
		this.parentScope = parentScope;
		this.innerScopes = []; // Initialize as an empty array
	}

	addInnerScope(name: string): Scope {
		const newScope = new Scope(name, this);
		this.innerScopes.push(newScope);
		return newScope;
	}

	addVariable(variable: string, type: string | null = null) {
		this.variables.set(variable, { type });
	}

	addFunction(
		func: string,
		parameters: string[] = [],
		returnType: string | null = null,
	) {
		this.functions.set(func, { parameters, returnType });
	}

	// Check if a variable or function is defined in this scope or its parents
	isDefined(name: string): boolean {
		if (this.variables.has(name) || this.functions.has(name)) {
			return true;
		}
		return this.parentScope ? this.parentScope.isDefined(name) : false;
	}

	// Traverse the tree to check if a variable is defined in a specific scope or its descendants
	isDefinedInScope(name: string): boolean {
		if (this.variables.has(name) || this.functions.has(name)) {
			return true;
		}
		return this.innerScopes.some((childScope) =>
			childScope.isDefinedInScope(name),
		);
	}

	isChildOf(parentScope: Scope | null): boolean {
		if (!parentScope) return false;
		let current = this.parentScope;
		while (current) {
			if (current === parentScope) return true;
			current = current.parentScope;
		}
		return false;
	}

	hasInnerScope(name: string): boolean {
		return this.innerScopes.some((scope) => scope.name === name);
	}

	getInnerScopeByName(name: string): Scope | null {
		return this.innerScopes.find((scope) => scope.name === name) || null;
	}

	// Traverses and prints the tree structure for debugging
	print(indent = 0): void {
		console.log(`${" ".repeat(indent)}Scope: ${this.name}`);
		for (const childScope of this.innerScopes) {
			childScope.print(indent + 2);
		}
	}
}
