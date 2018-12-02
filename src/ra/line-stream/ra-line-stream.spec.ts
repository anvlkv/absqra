import * as fs from 'fs';
import { RaLineStream } from './ra-line-stream';
import { expect } from 'chai';
import { RaInputStream } from '../input-stream/ra-input-stream';
import { Environment } from '../environment/ra.environment';


describe('RaLineStream', () => {
    let inputStream: RaInputStream;
    let lineStream: RaLineStream;
    let fileContent: string;

    before((done) => {
        fs.readFile('src/ra/example.ra', 'utf8', (err, c) => {
            if (err) throw err;
            fileContent = c;
            done();
        });
    });

    beforeEach((d) => {
        inputStream = new RaInputStream(fileContent);
        lineStream = new RaLineStream(inputStream);
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

    it('should read indents', () => {
        let lines = [];
        while (!lineStream.eof()){
            lines.push(lineStream.next());
        }
        expect(lines.map(l => l.indent), 'lines count').to.deep.equal([ 0, 1, 1, 1, 2, 1, 0, 0 ]);
    });

    it('should peek line', () => {
        expect(lineStream.peek().value).to.be.equal('`');
    });

    it('should peek line with shift', () => {
        expect(lineStream.peek(1).value).to.be.equal('\tthis is one string');
        expect(lineStream.peek(3).value).to.be.equal('\t`even now`');
    });
});