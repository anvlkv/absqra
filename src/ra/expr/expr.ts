import { RaTypes } from '../types.enum';
import { LiteralExpr } from './literal';
import { UnaryExpr } from './unary';
import { BinaryExpr } from './binary';
import { GroupingExpr } from './grouping';
import { OperatorExpr } from './operator';
import { LogicalExpr } from './logical';
import { AssignExpr } from './assign';
import { InvokeExpr } from './invoke';
import { ChoiceExpr } from './choice';
import { ValidatorExpr } from './validator';
import { PipeExpr } from './pipe';
import { OptionsExpr } from './options';
import { ContentExpr } from './content';
import { LineColumnAddress, Token } from '..';
import { DeclareExpr } from './declare';

export enum ExpressionTypes {
    LITERAL = 'lit',
    UNARY = 'uno',
    BINARY = 'bin',
    GROUPING = 'grp',
    OPERATOR = 'opr',
    LOGICAL = 'log',
    ASSIGN = 'asg',
    INVOKE = 'ink',
    CHOICE = 'cho',
    VALIDATOR = 'vld',
    PIPE = 'pip',
    OPTIONS = 'opt',
    CONTENT = 'ctc',
    DECLARE = 'dec'
}

export interface Visitor {
    visitLiteralExpr(expr: LiteralExpr);
    visitUnaryExpr(expr: UnaryExpr);
    visitBinaryExpr(expr: BinaryExpr);
    visitGroupingExpr(expr: GroupingExpr);
    visitOperatorExpr(expr: OperatorExpr);
    visitLogicalExpr(expr: LogicalExpr);
    visitAssignExpr(expr: AssignExpr);
    visitInvokeExpr(expr: InvokeExpr);
    visitChoiceExpr(expr: ChoiceExpr);
    visitValidatorExpr(expr: ValidatorExpr);
    visitPipeExpr(expr: PipeExpr);
    visitOptionsExpr(expr: OptionsExpr);
    visitContentExpr(expr: ContentExpr);
    visitDeclareExpr(expr: DeclareExpr);
}

export abstract class Expr {
    protected tokens: Token[];
    public readonly type = RaTypes.EXPR;
    public abstract readonly exprType: ExpressionTypes;
    public abstract accept(visitor: Visitor);

    get start(): LineColumnAddress {
        return this.tokens[0].start;
    }

    get end(): LineColumnAddress {
        return this.tokens[this.tokens.length - 1].end;
    }

    protected constructor(
        ...tokens: Token[]
    ){
        if (tokens) {
            this.tokens = tokens;
        }
        else {
            throw new SyntaxError(`Empty expression`)
        }
    }
}