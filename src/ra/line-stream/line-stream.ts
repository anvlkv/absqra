import { InputStream } from '../input-stream/input-stream';
import { Line } from './line';
import { Environment } from '../environment/ra.environment';
import { EnvironmentUtils } from '../environment/environment-utils';
import { LineColumnAddress } from '../line-column-address';

export interface RaLineStream {
    line: number;
    col: number;
    concatUpTo(fn: (current: Line) => boolean, line?: Line): Line;
    peek(shift?: number): Partial<Line>;
    next(skipParsing?: boolean): Line;
    eof(): boolean;
    croak(msg: string): void;
}

export class LineStream implements RaLineStream{
    get line() {
        return this.input.line;
    }
    get col() {
        return this.input.col
    }

    constructor(
        private input: InputStream
    ) {
    }

    concatUpTo(fn: (current: Line) => boolean, ln?: Line): Line {
        const output = ln || this.next();
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

    peek(shift = 0): Line {
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
        return new Line(
            lineIndent(line),
            line
        );
    }

    next(skipParsing?: boolean): Line {
        const start: LineColumnAddress = [this.input.line, this.input.col];
        let line = '';
        while (!this.input.eoLine()) {
            line += this.input.next();
        }
        const end: LineColumnAddress = [this.input.line, this.input.col];
        line += this.input.next();
        return new Line(
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
