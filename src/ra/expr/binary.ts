import { Expr, ExpressionTypes, Visitor } from './expr';
import { Token } from '..';
import { UnaryExpr } from './unary';


export class BinaryExpr extends Expr {
    public readonly exprType = ExpressionTypes.BINARY;

    public static isBinaryExpr(tokens: Token[]) {
        // return tokens.length >= 3 &&

    }

    accept(visitor: Visitor) {
        visitor.visitBinaryExpr(this);
    }

}