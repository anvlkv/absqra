import { Expr, ExpressionTypes, Visitor } from './expr';
import { Token, TokenType } from '..';
import { AssignExpr } from './assign';
import { GroupingExpr } from './grouping';


export class ChoiceExpr extends Expr {
    public readonly exprType = ExpressionTypes.CHOICE;
    public readonly chosenName: string;

    static isChoiceExpr(tokens: Token[]) {
        return tokens.length && tokens.length === 2 &&
            tokens[0].tokenType === TokenType.OP && tokens[0].value === '>' &&
            tokens[1].tokenType === TokenType.VAR
    }

    constructor(
        ...tokens: Token[]
    ) {
        super(...tokens);
        this.chosenName = tokens[1].value;
    }

    accept(visitor: Visitor) {
        visitor.visitChoiceExpr(this);
    }

}