import { Diagnostic, linter } from "@codemirror/lint";

import { syntaxTree } from "@codemirror/language";

// Example pseudocode linter
export const cobralLinter = linter((view) => {
  let diagnostics: Diagnostic[] = [];
  const tree = syntaxTree(view.state);

  // Variables to track declared variables and if conditions
  let declaredVariables: Set<string> = new Set();
  let usedVariables: Set<string> = new Set();
  let constants: Set<string> = new Set();

  tree.cursor().iterate((node) => {
    // Check for variable declarations
    switch (node.name) {
      case "VariableDeclaration": {
        const variableNode = node.node.getChild("VariableDefinition");
        const variableName = view.state.doc.sliceString(
          variableNode?.from || 0,
          variableNode?.to
        );

        if (node.node.getChild("constante")) {
          if (constants.has(variableName)) {
            diagnostics.push({
              from: variableNode?.from || 0,
              to: variableNode?.to || 0,
              severity: "error",
              message: `Variável '${variableName}' é declarada como constante e não pode ser reatribuída.`,
            });
          } else {
            constants.add(variableName); // Add the constant to the set after checking
          }
        }

        declaredVariables.add(variableName);
        break;
      }

      case "VariableName": {
        const variableName = view.state.doc.sliceString(node.from, node.to);
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
        // TODO: Improve this check to handle nested scopes
        if (node.node.parent?.name === "CallExpression") {
          // Skip function calls, no need to check if the variable is declared
          break;
        }

        const variableName = view.state.doc.sliceString(node.from, node.to);
        if (!declaredVariables.has(variableName)) {
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

// Helper functions for type-checking (example)
function getTypeOfNode(node: any, _view: any): string {
  // Placeholder logic: In real pseudocode, you'd inspect the node to determine its type
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
