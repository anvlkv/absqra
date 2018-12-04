import { BlockType, RaBlock } from './block';
import { RaToken, TokenType } from '../token-stream/token';
// import { RaTokenStream } from '../token-stream/ra-token-stream';
import { RaLineStream } from '../line-stream/ra-line-stream';
import { VirtualLineStream } from '../line-stream/virtual-line-stream';
import { LineColumnAddress } from '../line-column-address';
import { RaLine } from '../line-stream/line';


export class RaBlockStream {
    private pos = 1;
    private current: RaBlock;
    private lineStart: LineColumnAddress;

    constructor(
        private input: RaLineStream | VirtualLineStream,
        private level = 0
    ){}

    static isInvocationBlock(tokens: RaToken[]): boolean {
        let result = false;

        if(!tokens.length){
            return result;
        }
        else if(tokens[0].tokenType === TokenType.KW) {
            result = true;
        }
        else if(tokens[0].tokenType === TokenType.OP) {
            result = true;
        }

        return result;
    }

    static isDeclarationBlock(tokens: RaToken[]): boolean {
        let result = false;

        if(!tokens.length){
            return false;
        }
        else if (tokens[0].tokenType === TokenType.VAR) {
            result = tokens.length && (!tokens[1] || tokens[1].tokenType !== TokenType.OP)  ||
                (
                    tokens.length === 3 &&
                    tokens[1].tokenType === TokenType.OP &&
                    tokens[2].tokenType === TokenType.VAR
                );
        }

        return result;
    }

    static isContentBlocTick(tokens: RaToken[]): boolean {
        return tokens.length &&
            tokens.filter(t => t.tokenType === TokenType.PUNCT && t.value === '`').length === 1;
    }

    static endsWithComma(tokens: RaToken[]): boolean {
        let result = false;
        if (tokens.length) {
            const lastToken = tokens[tokens.length - 1];
            if (lastToken.tokenType === TokenType.PUNCT && lastToken.value === ',') {
                result = true;
            }
        }
        return result;
    }

    static endsWithMLCommentOpening(tokens: RaToken[]): boolean {
        let result = false;
        if (tokens.length) {
            const lastToken = tokens[tokens.length - 1];
            if (lastToken.tokenType === TokenType.ML_COMMENT_START) {
                result = true;
            }
        }
        return result;
    }

    private blockTye(tokens: RaToken[]): BlockType {
        if (RaBlockStream.isContentBlocTick(tokens)) {
            return BlockType.CONTENT;
        }
        else if (tokens.length === 1 && tokens[0].tokenType === TokenType.INLINE_CONTENT) {
            return BlockType.CONTENT;
        }
        else if (RaBlockStream.isDeclarationBlock(tokens)) {
            return BlockType.DECLARE;
        }
        else if (RaBlockStream.isInvocationBlock(tokens)) {
            return BlockType.INVOKE;
        }
        else if (tokens.length) {
            const text = tokens.map(t => `[${t.value}]:[${t.tokenType}]`).join(' ');
            this.croak(`Invalid block start [${text}]`);
        }
    }

    private readContent(start: LineColumnAddress): RaBlock {
        const head = this.readBlockOpening();
        const content = [];
        let end;

        while (!this.eof()) {
            const line = this.input.peek();
            if (line.indent > this.level) {
                content.push(this.input.next(true));
            }
            else if (RaBlockStream.isContentBlocTick(line.tokens)) {
                content.push(this.input.next());
                end = [this.input.line, this.input.col];
                break;
            }
            else {
                this.croak(`Indentation err: content block parsing failed [indent: ${line.indent}]`);
            }
        }

        return new RaBlock(
            BlockType.CONTENT,
            [head, ...content],
            start,
            end
        )
    }

    private readBlockOpening(line?: RaLine): RaLine {
        line = line || this.input.next();

        if (RaBlockStream.endsWithComma(line.tokens)) {
            line.concat(
                this.input.concatUntil((ln) => !RaBlockStream.endsWithComma(ln.tokens))
            );
        }
        else if (RaBlockStream.endsWithMLCommentOpening(line.tokens)) {
            line = this.skipComment(line);
            return this.readBlockOpening(line);
        }

        return line;
    }

    private readBlock(start: LineColumnAddress): RaBlock {
        const head = this.readBlockOpening();
        const content = [];

        while (!this.eof()) {
            const line = this.input.peek();
            if (line.indent > this.level) {
                content.push(this.input.next());
            }
            else if (line.tokens.length) {
                break;
            }
            else {
                this.input.next();
            }
        }

        return new RaBlock(
            this.blockTye(head.tokens),
            [head, ...content],
            start,
            [this.input.line, this.input.col]
        )
    }

    private skipComment(line: RaLine): RaLine {
        while (RaBlockStream.endsWithMLCommentOpening(line.tokens)){
            line.concat(this.input.next());
        }

        return line;
    }

    readNext(): RaBlock {
        const start: LineColumnAddress = [this.input.line, this.input.col];

        while (!this.input.eof()) {
            const line = this.input.peek();
            if (line.indent === this.level && line.tokens) {
                switch (this.blockTye(line.tokens)) {
                    case BlockType.CONTENT:
                        return this.readContent(start);
                    default:
                        return this.readBlock(start);
                }
            }
            else if(line.tokens) {
                this.croak(`Expected block with indentation level : [${this.level}], got [${line.indent}]`)
            }
            this.input.next();
        }
    }

    next(): RaBlock {
        const current = this.current;
        this.current = null;
        return current || this.readNext();
    }
    peek(): RaBlock {
        return this.current || (this.current = this.readNext());
    }
    eof(): boolean {
        return this.input.eof();
    }
    croak(msg) {
        this.input.croak(`${msg} [Block: ${this.pos}]`);
    }
}