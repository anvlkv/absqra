import * as fs from 'fs';
import { TokenStream } from './token-stream';
import { expect } from 'chai';
import { InputStream } from '../input-stream/input-stream';
import { example2TokenizerOutput } from './example2.tokenizer-output.spec';


describe('TokenStream', () => {
    let inputStream: InputStream;
    let tokenStream: TokenStream;
    let fileContent: string;

    before((done) => {
        fs.readFile('src/ra/example2.ra', 'utf8', (err, c) => {
            if (err) throw err;
            fileContent = c;
            done();
        });
    });

    beforeEach((d) => {
        inputStream = new InputStream(fileContent);
        tokenStream = new TokenStream(inputStream);
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