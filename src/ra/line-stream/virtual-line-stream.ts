import { InputStream } from '../input-stream/input-stream';
import { Line } from './line';
import { RaLineStream, LineStream } from './line-stream';


export class VirtualLineStream implements RaLineStream {

    get line(): number {
        if (!this.eof()) {
            return this.lines[this.pos].start[0];
        }
        else {
            return this.lines[this.pos - 1].end[0];
        }
    }
    get col(): number {
        if (!this.eof()) {
            return this.lines[this.pos].start[1];
        }
        else {
            return this.lines[this.pos - 1].end[1];
        }
    }

    private pos = 0;

    constructor(
        private lines: Line[],
    ) {}

    concatUpTo(fn: (current: Line) => boolean, ln?): Line {
        return LineStream.prototype.concatUpTo.call(this, fn, ln);
    }
    
    next(skipParsing?: boolean): Line {
        return this.lines[this.pos++];
    }

    peek(shift = 0): Partial<Line> {
        return this.lines[this.pos + shift];
    }

    /*
    * peek() {
        return this.current || (this.current = this.readNext());
    }

    next(): Token {
        const tok = this.current;
        this.current = null;
        return tok || this.readNext();
    }
    * */

    croak(msg: string): void {
        throw new SyntaxError(`${msg} [${this.line}:${this.col}]`)
    }

    eof(shift?): boolean {
        return !this.peek(shift);
    }

}