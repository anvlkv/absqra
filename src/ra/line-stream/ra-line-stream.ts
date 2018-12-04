import { RaInputStream } from '../input-stream/ra-input-stream';
import { RaLine } from './line';
import { Environment } from '../environment/ra.environment';
import { EnvironmentUtils } from '../environment/environment-utils';
import { LineColumnAddress } from '../line-column-address';

export interface LineStream {
    line: number;
    col: number;
    concatUntil(fn: (current: RaLine) => boolean): RaLine;
    peek(shift: number): Partial<RaLine>;
    next(): RaLine;
    eof(): boolean;
    croak(msg: string): void;
}

export class RaLineStream implements LineStream{
    get line() {
        return this.input.line;
    }
    get col() {
        return this.input.col
    }

    constructor(
        private input: RaInputStream
    ) {
    }

    concatUntil(fn: (current: RaLine) => boolean): RaLine {
        const output = this.next();
        const lines = [];
        while (!this.eof()) {
            const line = this.next();
            lines.push(line);
            if (fn(line)) {
                break;
            }
        }
        output.concat(...lines);

        return output;
    }

    peek(shift = 0): RaLine {
        let line = '';
        let chShift = 0;
        let lines = -1;
        while (!this.input.eof() && lines <= shift) {
            while (!this.input.eoLine(chShift)) {
                line += this.input.peek(chShift++);
            }
            line += this.input.peek(chShift++);
            lines ++;
            if (lines === shift) {
                break;
            }
            line = '';
        }
        return new RaLine(
            lineIndent(line),
            line
        );
    }

    next(skipParsing?: boolean): RaLine {
        const start: LineColumnAddress = [this.input.line, this.input.col];
        let line = '';
        while (!this.input.eoLine()) {
            line += this.input.next();
        }
        const end: LineColumnAddress = [this.input.line, this.input.col];
        line += this.input.next();
        return new RaLine(
            lineIndent(line),
            line,
            start,
            end,
            skipParsing
        );
    }

    eof(): boolean {
        return this.input.eof();
    }

    croak(msg: string) {
        this.input.croak(msg);
    }
}

export function isIndented(line: string): boolean {
    return EnvironmentUtils.indentationCharacterRegExp.test(line);
}
export function lineIndent(line: string): number {
    if (isIndented(line)) {
        return Math.floor(EnvironmentUtils.indentationRegExp.exec(line)[0].length / Environment.indentationWidth);
    }
    return 0;
}
