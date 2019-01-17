import { Expr, ExpressionTypes, Visitor } from './expr';
import { GroupingExpr, Token, TokenType } from '..';


export class DeclareExpr extends Expr {
    public readonly exprType = ExpressionTypes.DECLARE;

    public readonly declaredName: string;
    public readonly prototype: string;

    public static isDeclareExpr(tokens: Token[]) {
        return tokens.length >= 3 &&
            tokens[0].tokenType === TokenType.VAR &&
            tokens[1].tokenType === TokenType.OP && tokens[1].value == ':' &&
            tokens[2].tokenType === TokenType.VAR //&&
            // (!tokens[3] || GroupingExpr.isGroupingExpr(tokens.slice(3, tokens.length)) && )
    }

    constructor(
        ...tokens: Token[]
    ) {
        super(...tokens);
        this.declaredName = tokens[0].value;
        this.prototype = tokens[2].value;
    }

    accept(visitor: Visitor) {
        visitor.visitDeclareExpr(this);
    }

}