import * as fs from 'fs';
import { InputStream } from './input-stream';
import { expect } from 'chai';
import { LineStream } from '../line-stream/line-stream';


describe('InputStream', () => {
    let inputStream: InputStream;
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
        d();
    });

    it('should create stream', () => {
        expect(inputStream, 'stream created').to.exist;
    });

    it('should read character stream till end of file', () => {
        let str = '';
        while (!inputStream.eof()){
            str += inputStream.next();
        }
        expect(str, 'stream total').to.be.equal(fileContent);
    });

    it('should report End Of Line', () => {
        let eol = 0;
        while (!inputStream.eof()){
            inputStream.next();
            if (inputStream.eoLine()) {
                eol ++;
            }
        }
        expect(eol, 'end of line reached times').to.be.equal(8);
    });

    it('should peek character', () => {
        expect(inputStream.peek()).to.be.equal('`');
    });
});