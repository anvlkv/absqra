import { InputStream } from '../input-stream/input-stream';
import * as fs from "fs";
import { exampleLineOutput } from './example.line-output.spec';
import { expect } from 'chai';
import { VirtualLineStream } from './virtual-line-stream';
import { Line } from './line';


describe('VirtualLineStream', () => {
    let inputStream: InputStream;
    let lineStream: VirtualLineStream;
    let fileContent: string;

    before((done) => {
        fs.readFile('src/ra/example.ra', 'utf8', (err, c) => {
            if (err) throw err;
            fileContent = c;
            done();
        });
    });

    beforeEach((d) => {
        inputStream = new InputStream(fileContent);
        lineStream = new VirtualLineStream(exampleLineOutput.map(mock => {
            return new Line(
                mock._indent,
                mock._value,
                // @ts-ignore
                mock.start,
                mock.end
            )
        }));
        d();
    });

    it('should create stream', () => {
        expect(lineStream, 'stream created').to.exist;
    });

    it('should read lines', () => {
        let lines = [];
        while (!lineStream.eof()){
            lines.push(lineStream.next());
        }
        expect(lines.length, 'lines count').to.be.equal(8);
    });

    it('should have parsed tokens', () => {
        let lines = [];
        while (!lineStream.eof()){
            lines.push(lineStream.next());
        }
        expect(lines[1]._tokens).to.exist;
    });

    it('should read indents', () => {
        let lines = [];
        while (!lineStream.eof()){
            lines.push(lineStream.next());
        }
        expect(lines.map(l => l.indent), 'lines count').to.deep.equal([ 0, 1, 1, 1, 2, 1, 0, 0 ]);
    });

    it('should peek line', () => {
        expect(lineStream.peek().value).to.be.equal('`\n');
    });

    it('should peek line with shift', () => {
        expect(lineStream.peek(1).value).to.be.equal('\tthis is one string\n');
        expect(lineStream.peek(3).value).to.be.equal('\t`even now`\n');
    });

    it('should concatenate lines', () => {
        const line = lineStream.concatUpTo((ln) => ln.end[0] === 3);
        expect(line.span).to.be.equal(3);
        expect(line.value).to.be.equal(exampleLineOutput.reduce((total, ln) => {
            if(ln.end[0] <= 3) {
                total+=ln._value;
            }
            return total;
        }, ''));
    });
});