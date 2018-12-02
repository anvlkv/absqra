import { RaInputStream } from '../input-stream/ra-input-stream';
import { BlockType, LineColumnAddress, RaBlock } from './block';
import { Environment } from '../environment/ra.environment';
import { RaToken, TokenType } from '../token-stream/token';
// import { RaTokenStream } from '../token-stream/ra-token-stream';
import { RaLineStream } from '../line-stream/ra-line-stream';
import { RaLine } from '../line-stream/line';
import { RaTokenStream } from '../token-stream/ra-token-stream';


export class RaBlockStream {
    private blocks: RaBlock[] = [];
    private pos = 1;
    private current: Partial<RaBlock>;


    get count() {
        return this.blocks.length;
    }

    constructor(
        private input: RaLineStream
    ){}

    private isInvocationBlock(tokens: RaToken[]): boolean {
        let result = false;

        if(!tokens.length){
            return false;
        }
        else if(tokens[0].tokenType === TokenType.KW) {
            result = true;
        }
        else if(tokens[0].tokenType === TokenType.OP) {
            result = true;
        }

        return result;
    }

    private isDeclarationBlock(tokens: RaToken[]): boolean {
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

    private isContentBlock(tokens: RaToken[] | string): boolean {
        if (typeof tokens === 'string') {
            tokens = this.tokenizeLine(tokens);
        }
        tokens = (<RaToken[]>tokens);
        return tokens.length &&
            tokens[0].tokenType === TokenType.PUNCT &&
            tokens[0].value === '`';
    }

    private tokenizeLine(line: string): RaToken[] {
        const lineStream = new RaInputStream(line, [this.input.line, this.input.col]);
        const tokenStream = new RaTokenStream(lineStream);
        const tokens: RaToken[] = [];
        while (!tokenStream.eof()) {
            tokens.push(tokenStream.next())
        }
        return tokens;
    }

    readNext(): RaBlock {
        const start: LineColumnAddress = [this.input.line, this.input.col];
        const initialIndent = this.input.peek().indent;
        const content = [];
        while (!this.input.eof()) {
            const line = this.input.peek();
            if (line.indent >= initialIndent) {
                if (line.indent === initialIndent ) {
                    if (!content.length || this.isContentBlock(line.value)) {
                        content.push(this.input.next());
                    }
                    else {
                        break;
                    }
                }
                else {
                    content.push(this.input.next());
                }
            }
            else {
                break;
            }
        }

        if (!content.length) {
            this.croak(`No content`);
        } else {
            this.pos++;
            return new RaBlock(
                content,
                start,
                [this.input.line, this.input.col],
            )
        }

    }

    next() {
        const tok = this.current;
        this.current = null;
        return tok || this.readNext();
    }
    peek(): Partial<RaBlock> {
        return this.current || (this.current = this.readNext());
    }
    eof(): boolean {
        return this.input.eof();
    }
    croak(msg) {
        this.input.croak(`${msg} [Block: ${this.pos}]`);
    }
}