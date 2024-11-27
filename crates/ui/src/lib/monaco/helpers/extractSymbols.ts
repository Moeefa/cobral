export const extractSymbols = (text: string) => {
  const lines = text.split("\n");

  const symbols: {
    [scope: string]: { variables: Set<string>; functions: Set<string> };
  } = {
    ["global"]: { variables: new Set(), functions: new Set() },
  };

  let currentScope = "global";
  let scopeStack: string[] = [];

  lines.forEach((line, index) => {
    const lineNumber = index + 1;

    // Check for function declarations
    const functionMatch = /funcao\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(/g.exec(line);
    if (functionMatch) {
      const functionName = functionMatch[1];
      symbols["global"].functions.add(functionName);

      // Enter a new scope for the function
      currentScope = `function:${functionName}`;
      symbols[currentScope] = {
        variables: new Set(),
        functions: new Set(),
      };

      scopeStack.push(currentScope);
    }

    // Check for variable declarations
    const variableMatch = /\bdeclare\s+([a-zA-Z_][a-zA-Z0-9_]*)/g.exec(line);
    if (variableMatch) symbols[currentScope].variables.add(variableMatch[1]);

    // Handle block scope
    if (/\{/g.test(line)) {
      const newScope = `block:${lineNumber}`;
      symbols[newScope] = { variables: new Set(), functions: new Set() };
      scopeStack.push(currentScope);
      currentScope = newScope;
    }

    if (/\}/g.test(line)) currentScope = scopeStack.pop() || "global";
  });

  return symbols;
};
