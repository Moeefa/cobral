import { styleTags, tags as t } from "@lezer/highlight";

export const highlight = styleTags({
  "se senao": t.controlKeyword,
  "declare constante": t.definitionKeyword,
  use: t.moduleKeyword,
  BooleanLiteral: t.bool,
  nao: t.operator,
  null: t.null,
  VariableName: t.variableName,
  "CallExpression/VariableName TaggedTemplateExpression/VariableName":
    t.function(t.variableName),
  VariableDefinition: t.definition(t.variableName),
  "CallExpression/MemberExpression/PropertyName": t.function(t.propertyName),
  "FunctionDeclaration/VariableDefinition": t.function(
    t.definition(t.variableName)
  ),
  UpdateOp: t.updateOperator,
  // "LineComment Hashbang": t.lineComment,
  // BlockComment: t.blockComment,
  Number: t.number,
  String: t.string,
  Escape: t.escape,
  ArithOp: t.arithmeticOperator,
  LogicOp: t.logicOperator,
  CompareOp: t.compareOperator,
  Equals: t.definitionOperator,
  ": Spread": t.punctuation,
  "( )": t.paren,
  "[ ]": t.squareBracket,
  "{ }": t.brace,
  "InterpolationStart InterpolationEnd": t.special(t.brace),
  ".": t.derefOperator,
  ", ;": t.separator,

  TypeName: t.typeName,
  TypeDefinition: t.definition(t.typeName),
});
