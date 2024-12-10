import { Token, Tokenizer } from "@/lib/monaco/helpers/tokenizer";

export const extractSymbols = (text: string) => {
  const symbols: {
    [scope: string]: { variables: Set<string>; functions: Set<string> };
  } = { global: { variables: new Set(), functions: new Set() } };
  const tokenizer = new Tokenizer(text);

  let currentScope = "global";
  const scopeStack: string[] = [];

  let token: Token | null;

  while ((token = tokenizer.next())) {
    if (token.type === "keyword" && token.value === "funcao") {
      const nextToken = tokenizer.next();
      if (nextToken?.type === "identifier") {
        symbols.global.functions.add(nextToken.value);

        currentScope = `function:${nextToken.value}`;
        symbols[currentScope] = { variables: new Set(), functions: new Set() };
        scopeStack.push(currentScope);
      }
    } else if (
      token.type === "keyword" &&
      (token.value === "declare" || token.value == "constante")
    ) {
      let nextToken = tokenizer.next();
      if (nextToken?.value === "constante") nextToken = tokenizer.next();
      if (nextToken?.type === "identifier") {
        symbols[currentScope].variables.add(nextToken.value);
      }
    } else if (token.type === "delimiter" && token.value === "{") {
      const newScope = `block:${token.line}:${token.column}`;
      symbols[newScope] = { variables: new Set(), functions: new Set() };
      scopeStack.push(currentScope);
      currentScope = newScope;
    } else if (token.type === "delimiter" && token.value === "}") {
      currentScope = scopeStack.pop() || "global";
    }
  }

  return symbols;
};
