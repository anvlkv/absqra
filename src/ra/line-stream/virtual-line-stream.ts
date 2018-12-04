import { RaInputStream } from '../input-stream/ra-input-stream';
import { RaLine } from './line';
import { LineStream, RaLineStream } from './ra-line-stream';


export class VirtualLineStream implements LineStream {

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
        private lines: RaLine[],
    ) {}

    concatUntil(fn: (current: RaLine) => boolean): RaLine {
        return RaLineStream.prototype.concatUntil.call(this, fn);
    }
    
    next(): RaLine {
        return this.lines[this.pos++];
    }

    peek(shift = 0): Partial<RaLine> {
        return this.lines[this.pos + shift];
    }

    croak(msg: string): void {
        throw new Error(`${msg} [${this.line}:${this.col}]`)
    }

    eof(shift?): boolean {
        return !this.peek(shift);
    }

}