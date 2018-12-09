import { Expr, ExpressionTypes, Visitor } from './expr';
import { Token, TokenType } from '..';
import { AssignExpr } from './assign';


export class ValidatorExpr extends Expr {
    public readonly exprType = ExpressionTypes.VALIDATOR;

    static isValidatorExpr(tokens: Token[]) {
        return tokens.length && tokens.length >= 2 &&
            tokens[0].tokenType === TokenType.OP && tokens[0].value === '!' &&
            tokens[1].tokenType === TokenType.VAR &&
            (!tokens[2] || AssignExpr.isAssignExpr(tokens.slice(1, tokens.length)))
    }

    constructor(
        public readonly assigned: AssignExpr,
        ...tokens: Token[]
    ) {
        super(...tokens);
    }

    accept(visitor: Visitor) {
        visitor.visitValidatorExpr(this);
    }

}