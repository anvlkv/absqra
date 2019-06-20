import { Block, BlockType } from '../block-stream/block';
import { BlockStream } from '../block-stream/block-stream';
import { Program } from './program';
import { ContentExpr } from '../expr/content';
import {
    BackTickToken,
    ColonToken, EQToken,
    EXCLToken,
    Expr,
    GTToken,
    LParenthesisToken,
    LSToken,
    PipeToken,
    PlusToken,
    Token,
    TokenType,
} from '..';
import { DeclareExpr } from '../expr/declare';
import { GroupingExpr } from '../expr/grouping';


export class Parser {
    constructor (){}



    private parseContentBlock(block: Block): ContentExpr {
        const contentExpr = new ContentExpr(
            ...block.tokens,
        );
        if (block.tokens.length === 1 && block.tokens[0].tokenType === TokenType.INLINE_CONTENT) {
            contentExpr.value = block.tokens[0].value;
        }
        else {
            contentExpr.value = block.content.reduce((output, line) => {
                line.indent -= block.level;
                output += line.value;
                return output;
            }, '');
        }

        return contentExpr;

    }

    private parseChildren(block: Block): Program[] {
        return block.children ? block.children.map(child => this.parseBlock(child)) : null;
    }


    private parseGroupingExpr(tokens: Token[]): Expr<any> {
        throw new Error('Method not implemented.');
    }

    private parseChoiceExpr(tokens: Token[]): Expr<any> {
        throw new Error('Method not implemented.');
    }

    private parsePipeExpr(tokens: Token[]): Expr<any> {
        throw new Error('Method not implemented.');
    }

    private parseOptionsExpr(tokens: Token[]): Expr<any> {
        throw new Error('Method not implemented.');
    }

    private parseValidatorExpr(tokens: Token[]) {
        return undefined;
    }

    private parseBinaryExpr(tokens: Token[]) {
        return undefined;
    }

    private parseShortHand(tokens: Token[]): Expr {
        const tok = tokens[0];
        switch (true) {
            case tok instanceof GTToken: // > something
                return this.parseChoiceExpr(tokens);
            case tok instanceof PlusToken: // + something
                return this.parseOptionsExpr(tokens);
            case tok instanceof EXCLToken: // ! something
                return this.parseValidatorExpr(tokens);
            case tok instanceof PipeToken: // | something
                return this.parsePipeExpr(tokens);
            default:
                throw new SyntaxError(`Unsupported shorthand [${tok.errPrint()}]`)
        }
    }

    private parseDeclareExpr(tokens: Token[]) {
        return undefined;
    }

    private parseUnaryExpr(tokens: Token[]) {
        return undefined;
    }

    parseProgram(blockStream: BlockStream): Program {
        const program = new Program();

        while (!blockStream.eof()) {
            program.children.push(this.parseBlock(blockStream.next()));
        }

        return program;
    }

    parseBlock(block: Block): Program {
        const tokens = block.tokens;
        let i = 0;
        const program = new Program();
        while (i < tokens.length) {
            let expr: Expr;
            const tok = tokens[i];
            switch (true) {
                case tok instanceof LParenthesisToken:
                    expr = this.parseGroupingExpr(tokens);
                    break;
                case tok.tokenType === TokenType.VAR:
                    break;
                case tok instanceof ColonToken:
                    if (i !== 0) {
                        expr =  this.parseDeclareExpr(tokens);
                        break;
                    }
                case tok instanceof EXCLToken:
                    if (i !== 0) {
                        expr = this.parseUnaryExpr(tokens);
                    }
                case tok instanceof GTToken:
                case tok instanceof PlusToken:
                case tok instanceof PipeToken:
                case tok instanceof EQToken:
                case tok instanceof LSToken:
                    if (i === 0) {
                        expr = this.parseShortHand(tokens);
                    }
                    else {
                        expr = this.parseBinaryExpr(tokens);
                    }
                    break;
                case tok instanceof BackTickToken:
                    if (i === 0) {
                        expr = this.parseContentBlock(block);
                    }
                    else {
                        // expr = block
                    }
                    break;
                default:
                    throw new SyntaxError(`Unexpected token: ${tok.errPrint()}`);
            }

            if (!program.root && expr) {
                program.root = expr;
            }
            else if (expr) {
                throw new EvalError(`Program root is already defined: ${tok.errPrint()}`);
            }

            i += expr ? expr.tokens.length : 1;
        }

        return program;
    }
}