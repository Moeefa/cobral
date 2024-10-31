import { styleTags, tags as t } from "@lezer/highlight";

export const highlight = styleTags({
  "se senao": t.controlKeyword,
  "declare constante": t.definitionKeyword,
  // use: t.moduleKeyword,
  "vedadeiro falso": t.bool,
  "e ou": t.logicOperator,
  nao: t.operator,
  "para funcao retorne": t.keyword,

  VariableName: t.variableName,
  "CallExpression/VariableName VariableName": t.function(t.variableName),
  VariableDefinition: t.definition(t.variableName),
  CallExpression: t.function(t.propertyName),
  "FunctionDeclaration/VariableDefinition": t.function(
    t.definition(t.variableName)
  ),
  LineComment: t.lineComment,
  BlockComment: t.blockComment,
  Number: t.number,
  String: t.string,
  ArithOp: t.arithmeticOperator,
  CompareOp: t.compareOperator,
  Equals: t.definitionOperator,
  "( )": t.paren,
  "[ ]": t.squareBracket,
  "{ }": t.brace,
  ".": t.derefOperator,
  ", ;": t.separator,
});
