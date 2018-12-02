import { LineColumnAddress } from '../block-stream/block';
import { RaInputStream } from '../input-stream/ra-input-stream';
import { RaLine } from './line';
import { Environment } from '../environment/ra.environment';
import { EnvironmentUtils } from '../environment/environment-utils';


export class RaLineStream {


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

    private lineIndent(line: string): number {
        if (this.isIndented(line)) {
            return Math.floor(EnvironmentUtils.indentationRegExp.exec(line)[0].length / Environment.indentationWidth);
        }
        return 0;
    }

    private isIndented(line: string): boolean {
        return EnvironmentUtils.indentationCharacterRegExp.test(line);
    }

    peek(shift = 0): Partial<RaLine> {
        let line = '';
        let chShift = 0;
        let lines = -1;
        while (!this.input.eof() && lines <= shift) {
            while (!this.input.eoLine(chShift)) {
                line += this.input.peek(chShift++);
            }
            lines ++;
            if (lines === shift) {
                break;
            }
            chShift++;
            line = '';
        }
        return {
            indent: this.lineIndent(line),
            value: line
        };
    }

    next(): RaLine {
        const start: LineColumnAddress = [this.input.line, this.input.col];
        let line = '';
        while (!this.input.eoLine()) {
            line += this.input.next();
        }
        line += this.input.next();
        return new RaLine(
            this.lineIndent(line),
            line,
            start,
            [this.input.line, this.input.col]
        );
    }

    eof(): boolean {
        return this.input.eof();
    }

    croak(msg: string) {
        this.input.croak(msg);
    }
}