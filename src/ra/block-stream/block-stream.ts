import { Block, BlockType } from './block';
import { Token, TokenType } from '../token-stream/token';
// import { TokenStream } from '../token-stream/ra-token-stream';
import { RaLineStream } from '../line-stream/line-stream';
import { LineColumnAddress } from '../line-column-address';
import { Line } from '../line-stream/line';
import { BackTickToken } from '../token-stream/tokens/back-tick';
import { CommaToken } from '../token-stream/tokens/comma';


export class BlockStream {
    private pos = 1;
    private current: Block;

    constructor(
        private input: RaLineStream,
        private level = 0
    ){}

    static isInvocationBlock(tokens: Token[]): boolean {
        if(!tokens.length){
            return false;
        }
        else if(tokens[0].tokenType === TokenType.KW) {
            return true;
        }
        else if(tokens[0].tokenType === TokenType.VAR && !BlockStream.isDeclarationBlock(tokens)) {
            return true;
        }
        else if(tokens[0].tokenType === TokenType.OP) {
            return true;
        }

        return false;
    }

    static isDeclarationBlock(tokens: Token[]): boolean {
        let result = false;

        if(!tokens.length){
            return false;
        }
        else if (tokens[0].tokenType === TokenType.VAR) {
            result = tokens.length >= 3 &&
                tokens[1].tokenType === TokenType.OP;
        }

        return result;
    }

    static isContentBlocTick(tokens: Token[]): boolean {
        return tokens && tokens.length &&
            tokens.filter(t => t instanceof BackTickToken).length === 1;
    }

    static endsWithComma(tokens: Token[]): boolean {
        return tokens.length && tokens[0] instanceof CommaToken;
    }

    static endsWithMLCommentOpening(tokens: Token[]): boolean {
        let result = false;
        if (tokens.length) {
            const lastToken = tokens[tokens.length - 1];
            if (lastToken.tokenType === TokenType.ML_COMMENT_START) {
                result = true;
            }
        }
        return result;
    }

    private blockTye(tokens: Token[]): BlockType {
        if (BlockStream.isContentBlocTick(tokens) &&
            tokens[0] instanceof BackTickToken
        ) {
            return BlockType.CONTENT;
        }
        else if (tokens.length === 1 && tokens[0].tokenType === TokenType.INLINE_CONTENT) {
            return BlockType.CONTENT;
        }
        else if (BlockStream.isDeclarationBlock(tokens)) {
            return BlockType.DECLARE;
        }
        else if (BlockStream.isInvocationBlock(tokens)) {
            return BlockType.INVOKE;
        }
    }

    private readContent(start: LineColumnAddress, head = this.readBlockOpening()): Block {
        const content = [];
        let end;

        while (!this.eof()) {
            const line = this.input.peek();
            if (line.indent > head.indent) {
                content.push(this.input.next(true));
            }
            else if (BlockStream.isContentBlocTick(line.tokens)) {
                content.push(this.input.next());
                end = [this.input.line, this.input.col];
                break;
            }
            else {
                this.croak(`Indentation err: content block parsing failed [indent: ${line.indent}]`);
            }
        }

        return new Block(
            BlockType.CONTENT,
            [head, ...content]
        )
    }

    private readBlock(start: LineColumnAddress, head = this.readBlockOpening()): Block {
        const content = [];
        const hasContentBlock = BlockStream.isContentBlocTick(head.tokens);

        while (!this.eof()) {
            const line = this.input.peek();
            if (line.indent > this.level ||
                (hasContentBlock && line.indent === this.level && BlockStream.isContentBlocTick(line.tokens))
            ) {
                content.push(this.input.next());
            }
            else if (line.tokens && line.tokens.length) {
                break;
            }
            else {
                this.input.next();
            }
        }

        return new Block(
            this.blockTye(head.tokens),
            [head, ...content]
        )
    }

    private readBlockOpening(line?: Line): Line {
        line = line || this.input.next();

        if (BlockStream.endsWithComma(line.tokens)) {
            line.concat(
                this.input.concatUpTo((ln) => {
                    return !BlockStream.endsWithComma(ln.tokens) || BlockStream.endsWithMLCommentOpening(ln.tokens);
                })
            );
        }

        if (BlockStream.endsWithMLCommentOpening(line.tokens)) {
            line = this.readBlockOpening(this.skipComment(line));
        }
        else if (!line.tokens.length) {
            line = this.readBlockOpening(this.input.next());
        }


        return line;
    }

    private skipComment(line: Line): Line {
        line = this.input.concatUpTo((ln) => {
            return !/^.*\*\/.*/m.test(ln.value);
        }, line);

        line.commentSkipped();

        return line;
    }

    readNext(): Block {
        const start: LineColumnAddress = [this.input.line, this.input.col];

        while (!this.input.eof()) {
            const line = this.input.peek();
            if (line.indent === this.level && line.tokens) {
                switch (this.blockTye(line.tokens)) {
                    case BlockType.CONTENT:
                        return this.readContent(start);
                    case BlockType.INVOKE:
                    case BlockType.DECLARE:
                        return this.readBlock(start);
                    default:
                        if (line.tokens.length) {
                            this.croak(`Mixed block not supported: [${line.value}]`)
                        }
                }
            }
            else if(line.tokens) {
                this.croak(`Expected block with indentation level : [${this.level}], got [${line.indent}]`)
            }
            this.input.next();
        }
    }

    next(): Block {
        const current = this.current;
        this.current = null;
        return current || this.readNext();
    }
    peek(): Block {
        return this.current || (this.current = this.readNext());
    }
    eof(): boolean {
        return this.input.eof();
    }
    croak(msg) {
        this.input.croak(`${msg} [Block: ${this.pos}]`);
    }
}