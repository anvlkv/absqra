import * as fs from 'fs';
import { RaTokenStream } from './ra-token-stream';
import { expect } from 'chai';
import { RaInputStream } from '../input-stream/ra-input-stream';
import { example2TokenizerOutput } from '../example2.tokenizer-output.spec';


describe('RaTokenStream', () => {
    let inputStream: RaInputStream;
    let tokenStream: RaTokenStream;
    let fileContent: string;

    before((done) => {
        fs.readFile('src/ra/example2.ra', 'utf8', (err, c) => {
            if (err) throw err;
            fileContent = c;
            done();
        });
    });

    beforeEach((d) => {
        inputStream = new RaInputStream(fileContent);
        tokenStream = new RaTokenStream(inputStream);
        d();
    });

    it('should create stream', () => {
        expect(tokenStream, 'stream created').to.exist;
    });

    it('should tokenize stream as expected', () => {
        let tokens = [];
        while (!tokenStream.eof()){
            tokens.push(tokenStream.next());
        }
        expect(tokens, 'stream total').to.deep.equal(example2TokenizerOutput);
    });
});