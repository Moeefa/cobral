import * as monaco from "monaco-editor-core";

export const checkIncompatibleComparisons = (
  lines: string[]
): monaco.editor.IMarkerData[] => {
  const markers: monaco.editor.IMarkerData[] = [];
  const variables = extractVariables(lines); // Extract declared variables and their types

  lines.forEach((line, lineIndex) => {
    const tokens = tokenize(line);
    const comparisons = extractComparisons(tokens);

    comparisons.forEach(({ leftOperand, rightOperand, range }) => {
      const leftType = inferType(leftOperand, variables);
      const rightType = inferType(rightOperand, variables);

      // Debugging: Log operands, types, and ranges
      console.log("Left Operand:", leftOperand, "Type:", leftType);
      console.log("Right Operand:", rightOperand, "Type:", rightType);

      // Ignore comparisons between compatible types
      if (
        (leftType === "inteiro" && rightType === "real") ||
        (leftType === "real" && rightType === "inteiro")
      ) {
        return;
      }

      // Check for incompatibility
      if (leftType && rightType && leftType !== rightType) {
        markers.push({
          severity: monaco.MarkerSeverity.Error,
          startLineNumber: lineIndex + 1,
          endLineNumber: lineIndex + 1,
          startColumn: range.start + 1, // 1-based index
          endColumn: range.end + 1, // 1-based index
          message: `Comparação incompatível: '${leftOperand}' (${leftType}) e '${rightOperand}' (${rightType}) não podem ser comparados.`,
          code: "cobral.incompatibleComparison",
        });
      }
    });
  });

  return markers;
};

// Extract variable declarations and their types
const extractVariables = (lines: string[]): Record<string, string> => {
  const variables: Record<string, string> = {};
  const declarationRegex = /\bdeclare\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*(.+);/;

  lines.forEach((line) => {
    const match = declarationRegex.exec(line);
    if (match) {
      const variableName = match[1];
      const variableValue = match[2];
      variables[variableName] = evaluateExpression(variableValue, variables);
    }
  });

  console.log("Extracted Variables:", variables);
  return variables;
};

// Tokenizer: Breaks the line into meaningful tokens with positions
const tokenize = (line: string) => {
  const tokenRegex =
    /(\d+\.\d+|\d+|verdadeiro|falso|==|!=|<=|>=|<|>|[a-zA-Z_][a-zA-Z0-9_]*|["'][^"']*["']|[()])/g;
  const tokens = [];
  let match;

  while ((match = tokenRegex.exec(line)) !== null) {
    tokens.push({
      value: match[0],
      start: match.index, // Start character position
      end: match.index + match[0].length, // End character position
    });
  }

  return tokens;
};

// Evaluate an expression and return its resulting type
const evaluateExpression = (
  expression: string,
  variables: Record<string, string>
): string => {
  // Tokenize the expression
  const tokens = tokenize(expression);

  // Check for simple literals or variables
  if (tokens.length === 1) {
    return inferType(tokens[0].value, variables) || "unknown";
  }

  // Check for comparison operations
  const comparisonRegex = /([^\s]+)\s*(==|!=|<|>|<=|>=)\s*([^\s]+)/;
  const match = comparisonRegex.exec(expression);
  if (match) {
    const leftOperand = match[1];
    const rightOperand = match[3];
    const leftType = inferType(leftOperand, variables);
    const rightType = inferType(rightOperand, variables);

    // A comparison always results in a boolean
    if (leftType && rightType) {
      return "lógico";
    }
  }

  // Fallback for unsupported expressions
  return "unknown";
};

// Extract comparisons from tokens with accurate ranges
const extractComparisons = (
  tokens: { value: string; start: number; end: number }[]
): {
  leftOperand: string;
  rightOperand: string;
  range: { start: number; end: number };
}[] => {
  const comparisons: {
    leftOperand: string;
    rightOperand: string;
    range: { start: number; end: number };
  }[] = [];
  let currentComparison: {
    leftOperand?: string;
    operator?: string;
    rightOperand?: string;
    range: { start: number; end: number };
  } = {
    range: { start: 0, end: 0 },
  };

  tokens.forEach((token) => {
    if (["==", "!=", "<", ">", "<=", ">="].includes(token.value)) {
      // Set operator and initial range
      currentComparison.operator = token.value;
      currentComparison.range.start = currentComparison.leftOperand
        ? currentComparison.range.start
        : token.start;
      currentComparison.range.end = token.end;
    } else if (currentComparison.operator && !currentComparison.rightOperand) {
      if (currentComparison.leftOperand) {
        // Set right operand and finalize range
        currentComparison.rightOperand = token.value;
        currentComparison.range.end = token.end;

        comparisons.push({
          leftOperand: currentComparison.leftOperand,
          rightOperand: currentComparison.rightOperand,
          range: { ...currentComparison.range },
        });

        // Reset for next comparison
        currentComparison = { range: { start: 0, end: 0 } };
      } else {
        // Set left operand if not yet set
        currentComparison.leftOperand = token.value;
        currentComparison.range.start = token.start;
      }
    } else if (!currentComparison.operator) {
      // Update left operand and range
      currentComparison.leftOperand = token.value;
      currentComparison.range.start = token.start;
      currentComparison.range.end = token.end;
    }
  });

  return comparisons;
};

// Infer type of a token or variable
const inferType = (
  token: string,
  variables: Record<string, string> = {}
): string | null => {
  if (/^".*"$|^'.*'$/.test(token)) return "cadeia"; // Matches strings
  if (/^\d+$/.test(token)) return "inteiro"; // Matches integers
  if (/^\d+\.\d+$/.test(token)) return "real"; // Matches floats
  if (/^(verdadeiro|falso)$/.test(token)) return "lógico"; // Matches booleans
  if (variables[token]) return variables[token]; // Resolve variable type
  return null; // Unknown type
};
