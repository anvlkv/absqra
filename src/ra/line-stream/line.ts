
import { Token } from '../token-stream/token';
import { InputStream } from '../input-stream/input-stream';
import { TokenStream } from '../token-stream/token-stream';
import { LineColumn, LineColumnAddress } from '../line-column-address';
import { Environment } from '..';


export class Line {
    private _tokens: Token[];
    public readonly isPreview: boolean;

    public get value(): string {
        return this._value;
    }

    public get tokens(): Token[] {
        return this._tokens;
    }

    public get span(): number {
        return this.end[0] - this.start[0] + 1;
    }

    public get indent(): number {
        return this._indent;
    }

    public set indent(v: number) {
        if (v !== this.indent) {
            const newIndent = new Array(v * Environment.indentationWidth).join(Environment.indentationCharacter);
            const oldIndent = new Array(this.indent * Environment.indentationWidth).join(Environment.indentationCharacter);
            this._value.replace(
                new RegExp('^' + oldIndent, 'gm'),
                    newIndent
            );
            this._indent = v;
        }
    }

    constructor(
        private _indent: number,
        private _value: string,
        public start?: LineColumnAddress,
        public end?: LineColumnAddress,
        private skipParsing?: boolean
    ) {
        this.tryParseTokens();

        if (!this.start || !this.end) {
            this.isPreview = true;
        }
    }

    private tryParseTokens() {
        if (this.skipParsing) {
            this._tokens = null;
            return;
        }

        let tokens: Token[] = [];

        try {
            tokens = this.parse();
        }
        catch (e) {
            tokens = null;
        }
        this._tokens = tokens;
    }

    concat(...lines: Line[]) {
        while (lines.length) {
            const line  = lines.splice(0, 1)[0];
            if (LineColumn.isBehind(this.end, line.start)) {
                this.end = line.end;
                this._value += line.value;
            }
            else {
                throw new EvalError(`Will not concat lines from behind [${this.end[0]}:${this.end[1]}] : [${line.start[0]}:${line.start[1]}]`);
            }
        }
        this.tryParseTokens();
    }

    parse(): Token[] {
        let tokens: Token[] = [];
        const lineStream = new InputStream(this.value, this.start);
        const tokenStream = new TokenStream(lineStream);
        while (!tokenStream.eof()) {
            tokens.push(tokenStream.next())
        }

        return tokens;
    }

    commentSkipped() {
        this._value = this.tokens.reduce((val, tok) => {
            val += tok.value ? tok.value : ' ';
            return val;
        }, '');
        this.tryParseTokens();
    }

    splitAt(tickPosition: number): [Line, Line] {
        const aPart = this.tokens.slice(0, tickPosition);
        const bPart = this.tokens.slice(tickPosition, this.tokens.length);
        const aLine = new Line(
            this.indent,
            aPart.map(t => t.value).join(' '),
            aPart[0].start,
            aPart[aPart.length - 1].end
        );
        const bLine = new Line(
            this.indent,
            bPart.map(t => t.value).join(' '),
            bPart[0].start,
            bPart[bPart.length - 1].end
        );

        return [aLine, bLine];
    }
}