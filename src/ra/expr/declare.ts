import { Expr, ExpressionTypes, Visitor } from './expr';
import { Token } from '..';
import { GroupingExpr } from './grouping';


export class DeclareExpr extends Expr {
    readonly exprType = ExpressionTypes.DECLARE;

    readonly declaredName: string;
    readonly prototype: string;

    constructor(
        public readonly args: GroupingExpr,
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