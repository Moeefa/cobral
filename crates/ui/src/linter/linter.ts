import { Diagnostic, linter } from "@codemirror/lint";

import { syntaxTree } from "@codemirror/language";

enum Types {
  Integer = "inteiro",
  Float = "real",
  Boolean = "lógico",
  String = "cadeia",
  Unknown = "desconhecido",
}

export const variableTypes = new Map<string, string>();
const functionReturnTypes = new Map<string, string>([
  ["escrever", Types.String],
  ["ler", Types.String],
]);

const reservedKeywords = new Set([
  "declare",
  "constante",
  "se",
  "senao",
  "escrever",
  "ler",
  "nao",
]);

export const cobralLinter = linter((view) => {
  let diagnostics: Diagnostic[] = [];
  const tree = syntaxTree(view.state);

  let declaredVariables: Set<string> = new Set();
  let usedVariables: Set<string> = new Set();
  let constants: Set<string> = new Set();

  tree.cursor().iterate((node) => {
    switch (node.name) {
      case "VariableDeclaration": {
        const variableNode = node.node.getChild("VariableDefinition");
        const variableName = view.state.doc.sliceString(
          variableNode?.from || 0,
          variableNode?.to
        );

        const valueNode = node.node.getChild("Expression");
        const variableValue = view.state.doc.sliceString(
          valueNode?.from || 0,
          valueNode?.to
        );

        const callNode = valueNode?.node.getChild("VariableName");

        let inferredType = inferType(variableValue);

        if (callNode) {
          const functionName = view.state.doc.sliceString(
            callNode.from,
            callNode.to
          );

          if (functionReturnTypes.has(functionName)) {
            inferredType = functionReturnTypes.get(functionName) || "unknown";
          }
        }

        if (reservedKeywords.has(variableName)) {
          diagnostics.push({
            from: variableNode?.from || 0,
            to: variableNode?.to || 0,
            severity: "error",
            message: `Variável '${variableName}' é uma palavra reservada.`,
          });
        }

        if (node.node.getChild("constante")) {
          if (constants.has(variableName)) {
            diagnostics.push({
              from: variableNode?.from || 0,
              to: variableNode?.to || 0,
              severity: "error",
              message: `Variável '${variableName}' é declarada como constante e não pode ser reatribuída.`,
            });
          } else {
            constants.add(variableName);
          }
        }

        variableTypes.set(variableName, inferredType);
        declaredVariables.add(variableName);
        break;
      }

      case "VariableName": {
        const variableName = view.state.doc.sliceString(node.from, node.to);
        if (!reservedKeywords.has(variableName))
          usedVariables.add(variableName);
        break;
      }

      case "BinaryExpression": {
        const leftOperandType = getTypeOfNode(node.node.lastChild!, view);
        const rightOperandType = getTypeOfNode(node.node.lastChild!, view);
        if (leftOperandType !== rightOperandType) {
          diagnostics.push({
            from: node.from,
            to: node.to,
            severity: "error",
            message: `Type mismatch: '${leftOperandType}' and '${rightOperandType}' are incompatible.`,
          });
        }
        break;
      }

      case "senao": {
        if (!hasPrevious(node, "se")) {
          diagnostics.push({
            from: node.from,
            to: node.to,
            severity: "error",
            message: "'senao' sem 'se' correspondente.",
          });
        }
        break;
      }
    }
  });

  tree.cursor().iterate((node) => {
    switch (node.name) {
      case "VariableDeclaration": {
        const variableNode = node.node.getChild("VariableDefinition");
        if (variableNode) {
          const variableName = view.state.doc
            .sliceString(variableNode.from, variableNode.to)
            .trim();

          if (
            declaredVariables.has(variableName) &&
            !usedVariables.has(variableName)
          ) {
            diagnostics.push({
              from: variableNode.from,
              to: variableNode.to,
              severity: "warning",
              message: `Variável '${variableName}' é declarada mas nunca usada.`,
            });
          }
        }
        break;
      }

      case "VariableName": {
        if (node.node.parent?.name === "CallExpression") {
          break; // Skip function calls
        }

        const variableName = view.state.doc.sliceString(node.from, node.to);

        // Skip checking reserved keywords
        if (
          !declaredVariables.has(variableName) &&
          !reservedKeywords.has(variableName.trim())
        ) {
          diagnostics.push({
            from: node.from,
            to: node.to,
            severity: "error",
            message: `Variável '${variableName}' não declarada.`,
          });
        }
        break;
      }
    }
  });

  return diagnostics;
});

function inferType(value: string): string {
  if (/^-?\d+$/.test(value)) {
    return Types.Integer;
  } else if (/^-?\d+\.\d+$/.test(value)) {
    return Types.Float;
  } else if (value === "verdadeiro" || value === "falso") {
    return Types.Boolean;
  } else if (/^".*"$/.test(value)) {
    return Types.String;
  } else {
    return Types.Unknown;
  }
}

function getTypeOfNode(node: any, _view: any): string {
  switch (node.name) {
    case "Integer":
      return "number";
    case "Float":
      return "number";
    case "Boolean":
      return "boolean";
    case "String":
      return "string";
    default:
      return "unknown";
  }
}

function hasPrevious(node: any, prev: string): boolean {
  let currentNode = node.node.prevSibling;

  while (currentNode) {
    if (currentNode.name === prev) return true;

    currentNode = currentNode.prevSibling;
  }

  return false;
}
