import { Expr, ExpressionTypes, Visitor } from './expr';
import { Token, TokenType } from '..';


export class InvokeExpr extends Expr {
    public readonly exprType = ExpressionTypes.INVOKE;
    public readonly invokedType: string;
    public readonly emitsName: string;

    static isInvokeExpr(tokens: Token[]) {
        return tokens.length && tokens.length <=2 &&
            tokens[0].tokenType === TokenType.VAR &&
            (!tokens[1] || tokens[1].tokenType === TokenType.VAR);
    }

    constructor(
        ...tokens: Token[]
    ) {
        super(...tokens);
        this.invokedType = tokens[0].value;
        this.emitsName = tokens[1] ? tokens[1].value : null;
        // this.declaredName = tokens[0].value;
        // this.prototype = tokens[2].value;
    }

    accept(visitor: Visitor) {
        visitor.visitInvokeExpr(this);
    }

}