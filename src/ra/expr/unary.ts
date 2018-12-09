import { Expr, ExpressionTypes, Visitor } from './expr';
import { bool, Token, TokenType } from '..';


export class UnaryExpr extends Expr {
    public readonly exprType = ExpressionTypes.UNARY;

    public static unaryExprKwList = bool;

    public static isUnaryExpr(tokens: Token[]) {
        return tokens.length === 1 &&
            tokens[0].tokenType === TokenType.VAR ||
            tokens[0].tokenType === TokenType.NUM ||
            tokens[0].tokenType === TokenType.INLINE_CONTENT ||
            (tokens[0].tokenType === TokenType.KW && UnaryExpr.unaryExprKwList.indexOf(tokens[0].value) >= 0);
    }

    accept(visitor: Visitor) {
        visitor.visitUnaryExpr(this);
    }

}