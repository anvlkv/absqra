import { Expr, ExpressionTypes, Visitor } from './expr';


export class OperatorExpr extends Expr {
    readonly exprType = ExpressionTypes.OPERATOR;

    accept(visitor: Visitor) {
        visitor.visitOperatorExpr(this);
    }

}