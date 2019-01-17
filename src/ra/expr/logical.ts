import { Expr, ExpressionTypes, Visitor } from './expr';
import { logical, Token, TokenType } from '..';
import { UnaryExpr } from './unary';
import { BinaryExpr } from './binary';
import { GroupingExpr } from './grouping';


export class LogicalExpr extends Expr {
    public static logicalKwList = logical;

    public readonly exprType = ExpressionTypes.LOGICAL;

    public static isLogicalExpr(tokens: Token[]) {

        return tokens && tokens.length >= 2 &&
            tokens[0].tokenType === TokenType.KW && LogicalExpr.logicalKwList.indexOf(tokens[0].value) >= 0 &&
            (tokens[1].tokenType === TokenType.VAR || (() => {
                const expr = tokens.slice(1, tokens.length);
                if (UnaryExpr.isUnaryExpr(expr)) {
                    return true;
                }
                // else if (BinaryExpr.isBinaryExpr(expr)) {
                //     return true;
                // }
                // else if (GroupingExpr.isGroupingExpr(expr) && [ExpressionTypes.UNARY, ExpressionTypes.BINARY].indexOf(GroupingExpr.groupingType(expr)) >= 0) {
                //     return true
                // }
                return false;
            })());
    }

    accept(visitor: Visitor) {
        // visitor.visitLogicalExpr(this);
    }

}