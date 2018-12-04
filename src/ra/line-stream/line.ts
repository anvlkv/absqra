
import { RaToken } from '../token-stream/token';
import { RaInputStream } from '../input-stream/ra-input-stream';
import { RaTokenStream } from '../token-stream/ra-token-stream';
import { LineColumn, LineColumnAddress } from '../line-column-address';


export class RaLine {
    private _tokens: RaToken[];
    public readonly isPreview: boolean;

    public get value(): string {
        return this._value;
    }

    public get tokens(): RaToken[] {
        return this._tokens;
    }

    public get span(): number {
        return this.end[0] - this.start[0] + 1;
    }

    constructor(
        public readonly indent: number,
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

        let tokens: RaToken[] = [];

        try {
            tokens = this.parse();
        }
        catch (e) {
            tokens = null;
        }
        this._tokens = tokens;
    }

    concat(...lines: RaLine[]) {
        while (lines.length) {
            const line  = lines.splice(0, 1)[0];
            if (LineColumn.isBehind(this.end, line.start)) {
                this.end = line.end;
                this._value += line.value;
            }
            else {
                throw new Error(`Will not concat lines from behind [${this.end[0]}:${this.end[1]}] : [${line.start[0]}:${line.start[1]}]`);
            }
        }
        this.tryParseTokens();
    }

    parse(): RaToken[] {
        let tokens: RaToken[] = [];
        const lineStream = new RaInputStream(this.value, this.start);
        const tokenStream = new RaTokenStream(lineStream);
        while (!tokenStream.eof()) {
            tokens.push(tokenStream.next())
        }

        return tokens;
    }
}