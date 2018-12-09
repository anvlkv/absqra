import { Expr, ExpressionTypes, Visitor } from './expr';


export class LiteralExpr extends Expr {
    readonly exprType = ExpressionTypes.LITERAL;

    accept(visitor: Visitor) {
        visitor.visitLiteralExpr(this);
    }

}