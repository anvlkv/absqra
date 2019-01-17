import * as fs from 'fs';
import { TokenStream } from './token-stream';
import { expect, use } from 'chai';
import { InputStream } from '../input-stream/input-stream';
import { example2TokenizerOutput } from './example2.tokenizer-output.spec';
import { Token } from './token';
import chaiJsonEqual  = require('chai-json-equal');
import { GTToken } from './tokens/gt';
import { LSToken } from './tokens/ls';
import { ColonToken } from './tokens/colon';
import { CommaToken } from './tokens/comma';
import { EQToken } from './tokens/eq';
import { FWDSlashToken } from './tokens/fwd-slash';

use(chaiJsonEqual);

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

    context('with valid input', () => {
        let tokens: Token[];
        beforeEach((d) => {
            tokens = [];
            while (!tokenStream.eof()){
                tokens.push(tokenStream.next());
            }
            d();
        });

        it('should tokenize stream as expected', () => {
            expect(tokens).to.be.jsonEqual(example2TokenizerOutput);
        });

        it('should have distinct instances for "static" tokens', () => {
            expect(tokens[5] instanceof GTToken, '>').to.be.true;
            expect(tokens[5] instanceof LSToken, '!<').to.be.false;
            expect(tokens[1] instanceof ColonToken, ':').to.be.true;
            expect(tokens[1] instanceof CommaToken, '!,').to.be.false;
            expect(tokens[11] instanceof EQToken, '=').to.be.true;
            expect(tokens[11] instanceof FWDSlashToken, '!/').to.be.false;
        });
    });

});