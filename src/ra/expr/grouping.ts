import { Expr, ExpressionTypes, Visitor } from './expr';
import { BinaryExpr, bool, DeclareExpr, LiteralExpr, Token, TokenType, Tree, UnaryExpr } from '..';


export class GroupingExpr<G extends UnaryExpr | BinaryExpr | LiteralExpr | DeclareExpr = any> extends Expr<G> {
    public readonly exprType = ExpressionTypes.GROUPING;

    public static allowedKwList = bool;

    public static isGroupingExpr(tokens: Token[]) {
        let hasStart = false,
            hasEnd = false;
        return tokens.length &&
            tokens.every((t, i) => {
                let result;
                if (i === 0) {
                    result = t.tokenType === TokenType.PUNCT && t.value === '(';
                    hasStart = result;
                }
                else if (i === tokens.length - 1) {
                    result = t.tokenType === TokenType.PUNCT && t.value === ')';
                    hasEnd = result;
                }
                else {
                    result = t.tokenType === TokenType.VAR ||
                        t.tokenType === TokenType.INLINE_CONTENT ||
                        t.tokenType === TokenType.REG_EXP ||
                        t.tokenType === TokenType.NUM ||
                        (t.tokenType === TokenType.KW && GroupingExpr.allowedKwList.indexOf(t.value) >= 0) ||
                        (t.tokenType === TokenType.PUNCT && ')('.indexOf(t.value) < 0 ) ||
                        (t.tokenType === TokenType.OP && t.value === ':') ||
                        GroupingExpr.isGroupingExpr(tokens.slice(i, tokens.length - 1));
                }

                // console.log(result, `${i+1}/${tokens.length}`, t.value, t.tokenType);

                return result;
            }) && hasStart && hasEnd;
    }

    public static isGroupOf(tokens: Token[], type) {

    }

    constructor(
        ...tokens: Token[]
    ) {
        super(...tokens);
    }

    accept(visitor: Visitor) {
        visitor.visitGroupingExpr(this);
    }

}