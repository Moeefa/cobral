import * as monaco from "monaco-editor-core";

import { Token, Tokenizer } from "@/lib/monaco/helpers/tokenizer";

const builtInFunctions: {
  [name: string]: { args: string[]; returnType: string };
} = {
  ler: { args: ["cadeia"], returnType: "cadeia" }, // Reads input and returns a string
  int: { args: ["cadeia"], returnType: "inteiro" }, // Converts data to integer
  real: { args: ["inteiro"], returnType: "real" }, // Converts data to float
};

// Check for incompatible comparisons
export const checkIncompatibleComparisons = (
  tokenizer: Tokenizer,
  scopes: {
    [scope: string]: { variables: Set<string>; functions: Set<string> };
  }
): monaco.editor.IMarkerData[] => {
  tokenizer.reset(); // Start from the beginning of the tokens
  const markers: monaco.editor.IMarkerData[] = [];
  const scopeStack: string[] = []; // Track nested scopes
  const variableTypes: Map<string, string> = new Map(); // Map to track variable types

  let currentScope = "global"; // Start in the global scope

  let previousToken: Token | null = null; // Track the previous token
  let token: Token | null;

  while ((token = tokenizer.next())) {
    if (token.type === "keyword") {
      if (token.value === "funcao") {
        // Enter a new function scope
        const nextToken = tokenizer.next();
        if (nextToken?.type === "identifier") {
          currentScope = `function:${nextToken.value}`;
          scopeStack.push(currentScope);
          scopes[currentScope] = { variables: new Set(), functions: new Set() }; // Initialize function scope
        }
      } else if (token.value === "declare") {
        // Handle variable declaration in the current scope
        tokenizer.peek()?.value === "constante" ? tokenizer.next() : null;

        const nextToken = tokenizer.next();
        if (nextToken?.type === "identifier") {
          scopes[currentScope] = scopes[currentScope] || {
            variables: new Set(),
            functions: new Set(),
          };
          scopes[currentScope].variables.add(nextToken.value);

          // Infer and store the variable's type
          const assignmentToken = tokenizer.peek();
          if (assignmentToken?.value === "=") {
            tokenizer.next(); // Consume "="
            const valueToken = tokenizer.next();
            if (valueToken) {
              const inferredType = inferType(valueToken.value);
              variableTypes.set(nextToken.value, inferredType || "unknown");
            }
          }
        }
      }
    }

    if (token.type === "delimiter") {
      if (token.value === "{") {
        // Enter a new block scope
        const blockScope = `block:${token.line}`;
        scopeStack.push(currentScope); // Save current scope
        currentScope = blockScope;
        scopes[currentScope] = { variables: new Set(), functions: new Set() }; // Initialize block scope
      } else if (token.value === "}") {
        // Exit the current block or function scope
        currentScope = scopeStack.pop() || "global";
      }
    }

    // Handle comparisons
    if (["==", "!=", "<", ">", "<=", ">="].includes(token.value)) {
      // Ensure previous token exists and is valid
      if (
        !previousToken ||
        !["identifier", "string", "number"].includes(previousToken.type)
      ) {
        markers.push({
          severity: monaco.MarkerSeverity.Error,
          startLineNumber: token.line,
          endLineNumber: token.line,
          startColumn: token.column,
          endColumn: token.column + token.value.length,
          message: "Operação de comparação incompleta.",
          code: "cobral.incompleteComparison",
        });

        continue; // Skip incomplete comparisons
      }

      const leftOperandToken = previousToken;
      const rightOperandToken = tokenizer.next();

      // Ensure the right operand exists and is valid
      if (
        !rightOperandToken ||
        !["identifier", "string", "number"].includes(rightOperandToken.type)
      ) {
        markers.push({
          severity: monaco.MarkerSeverity.Error,
          startLineNumber: token.line,
          endLineNumber: token.line,
          startColumn: leftOperandToken.column,
          endColumn: token.column + token.value.length,
          message: "Operação de comparação incompleta.",
          code: "cobral.incompleteComparison",
        });

        continue; // Skip incomplete comparisons
      }

      if (
        !leftOperandToken ||
        !["identifier", "string", "number"].includes(leftOperandToken.type)
      ) {
        markers.push({
          severity: monaco.MarkerSeverity.Error,
          startLineNumber: token.line,
          endLineNumber: token.line,
          startColumn: token.column,
          endColumn: rightOperandToken.column + rightOperandToken.value.length,
          message: "Operação de comparação incompleta.",
          code: "cobral.incompleteComparison",
        });

        continue; // Skip incomplete comparisons
      }

      const rightOperand = rightOperandToken.value;
      const leftOperand = leftOperandToken.value;

      const leftType = resolveType(
        leftOperand,
        currentScope,
        scopes,
        variableTypes
      );
      const rightType = resolveType(
        rightOperand,
        currentScope,
        scopes,
        variableTypes
      );

      // Ignore comparisons between compatible types
      if (
        (leftType === "inteiro" && rightType === "real") ||
        (leftType === "real" && rightType === "inteiro")
      ) {
        previousToken = token; // Update previous token and continue
        continue;
      }

      // Check for incompatibility
      if (leftType && rightType && leftType !== rightType) {
        markers.push({
          severity: monaco.MarkerSeverity.Error,
          startLineNumber: token.line,
          endLineNumber: token.line,
          startColumn: leftOperandToken.column,
          endColumn: rightOperandToken.column + rightOperand.length,
          message: `Comparação incompatível: '${leftOperand}' (${leftType}) e '${rightOperand}' (${rightType}) não podem ser comparados.`,
          code: "cobral.incompatibleComparison",
        });
      }
    }

    previousToken = token; // Update the previous token at the end of the loop
  }

  return markers;
};

// Resolve the type of a variable or literal
const resolveType = (
  name: string,
  scope: string,
  scopes: {
    [scope: string]: { variables: Set<string>; functions: Set<string> };
  },
  variableTypes: Map<string, string>
): string | null => {
  // Check if it's a literal
  if (/^".*"$|^'.*'$/.test(name)) return "cadeia"; // Matches strings
  if (/^\d+$/.test(name)) return "inteiro"; // Matches integers
  if (/^\d+\.\d+$/.test(name)) return "real"; // Matches floats
  if (/^(verdadeiro|falso)$/.test(name)) return "lógico"; // Matches booleans

  // Check if the name is a built-in function
  if (builtInFunctions[name]) {
    return builtInFunctions[name].returnType;
  }

  // Resolve variable type from the current or parent scopes
  let currentScope = scope;
  while (currentScope) {
    const variables = scopes[currentScope]?.variables;

    if (variables && variables.has(name)) {
      return variableTypes.get(name) || "unknown";
    }

    // Exit the loop if we are already in the global scope
    if (currentScope === "global") {
      break;
    }

    // Update currentScope to its parent
    currentScope = currentScope.includes(":")
      ? currentScope.split(":")[0]
      : "global";
  }

  return null; // Unknown type
};

// Infer type of a literal or value
const inferType = (value: string): string | null => {
  if (/^".*"$|^'.*'$/.test(value)) return "cadeia"; // String
  if (/^\d+$/.test(value)) return "inteiro"; // Integer
  if (/^\d+\.\d+$/.test(value)) return "real"; // Float
  if (/^(verdadeiro|falso)$/.test(value)) return "lógico"; // Boolean

  // Check if the name is a built-in function
  if (builtInFunctions[value]) {
    return builtInFunctions[value].returnType;
  }

  return null; // Unknown type
};
