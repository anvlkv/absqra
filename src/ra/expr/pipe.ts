import { Expr, ExpressionTypes, Visitor } from './expr';


export class PipeExpr extends Expr {
    readonly exprType = ExpressionTypes.PIPE;

    accept(visitor: Visitor) {
        visitor.visitPipeExpr(this);
    }

}