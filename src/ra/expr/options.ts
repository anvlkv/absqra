import { Expr, ExpressionTypes, Visitor } from './expr';


export class OptionsExpr extends Expr {
    readonly exprType = ExpressionTypes.OPTIONS;

    accept(visitor: Visitor) {
        visitor.visitOptionsExpr(this);
    }

}