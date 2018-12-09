import { Expr, ExpressionTypes, Visitor } from './expr';
import { Token, TokenType } from '..';


export class AssignExpr extends Expr {
    readonly exprType = ExpressionTypes.ASSIGN;

    static isAssignExpr(tokens: Token[]) {
        return tokens.length === 3 &&
            tokens[0].tokenType === TokenType.VAR &&
            tokens[1].tokenType === TokenType.OP && tokens[1].value === '=' &&
            (
                tokens[2].tokenType === TokenType.NUM ||
                tokens[2].tokenType === TokenType.INLINE_CONTENT ||
                tokens[2].tokenType === TokenType.KW ||
                tokens[2].tokenType === TokenType.VAR ||
                tokens[2].tokenType === TokenType.PUNCT && tokens[2].value === '`'
            );
    }

    constructor(
        ...tokens: Token[]
    ) {
        super(...tokens);
    }

    accept(visitor: Visitor) {
        visitor.visitAssignExpr(this);
    }
}