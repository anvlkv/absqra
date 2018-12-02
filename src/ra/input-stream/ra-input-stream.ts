import { LineColumnAddress } from '../block-stream/block';


export class RaInputStream {
    public pos = 0;
    public line = 1;
    public col = 0;
    constructor(
        private input: string,
        ctx?: LineColumnAddress,
        private errPrefix?: string
    ) {
        if (ctx) {
            this.line = ctx[0];
            this.col = ctx[1];
        }
    }

    peek(shift = 0): string {
        return this.input.charAt(this.pos + shift);
    }
    next(): string {
        const ch = this.input.charAt(this.pos++);
        if (ch === '\n') {
            this.line++;
            this.col = 0;
        } else {
            this.col++;
        }
        return ch;
    }
    eof(shift?): boolean {
        return this.peek(shift) == '';
    }
    eoLine(shift?): boolean {
        return this.eof(shift) || this.peek(shift) === '\n';
    }
    croak(msg: string) {
        throw new Error(`${this.errPrefix ? this.errPrefix : ''}${msg} [${this.line}:${this.col}]`);
    }
}