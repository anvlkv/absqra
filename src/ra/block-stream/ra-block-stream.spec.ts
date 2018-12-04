import { expect } from 'chai';
import { RaInputStream } from '../input-stream/ra-input-stream';
import * as fs from "fs";
import { RaBlockStream } from './ra-block-stream';
import { RaLineStream } from '../line-stream/ra-line-stream';
import { exampleBlockOutput } from '../example.block-output.spec';
import { example1BlockOutput } from '../example1.block-output.spec';
import { expectDeeperThanDeepEqual } from '../../spec-utils.spec';
import { RaBlock } from './block';


describe ('RaBlockStream', () => {
    let blockStream: RaBlockStream;
    let inputStream: RaInputStream;
    let lineStream: RaLineStream;
    let fileContent: string;

    describe('with example', () => {
        before((done) => {
            fs.readFile('src/ra/example.ra', 'utf8', (err, c) => {
                if (err) throw err;
                fileContent = c;
                done();
            });
        });

        beforeEach((d) => {
            inputStream = new RaInputStream(fileContent, null, '[src/ra/example.ra] ');
            lineStream = new RaLineStream(inputStream);
            blockStream = new RaBlockStream(lineStream);
            d();
        });

        it('should create block stream', () => {
            expect(blockStream).to.exist;
        });

        it('should read blocks', () => {
            const blocks = [];

            while (!blockStream.eof()) {
                blocks.push(blockStream.next());
            }

            expect(blocks.length).to.be.equal(2);
        });

        it('should parse blocks as expected', () => {
            const blocks = [];

            while (!blockStream.eof()) {
                blocks.push(blockStream.next());
            }
            expectDeeperThanDeepEqual(blocks, exampleBlockOutput);
        });
    });

    describe('with example1', () => {
        before((done) => {
            fs.readFile('src/ra/example1.ra', 'utf8', (err, c) => {
                if (err) throw err;
                fileContent = c;
                done();
            });
        });

        beforeEach((d) => {
            inputStream = new RaInputStream(fileContent, null, '[src/ra/example1.ra] ');
            lineStream = new RaLineStream(inputStream);
            blockStream = new RaBlockStream(lineStream);
            d();
        });

        it('should create block stream', () => {
            expect(blockStream).to.exist;
        });

        it('should read blocks', () => {
            const blocks = [];

            while (!blockStream.eof()) {
                blocks.push(blockStream.next());
            }

            expect(blocks.length).to.be.equal(example1BlockOutput.length);
        });

        it('should parse blocks as expected', () => {
            const blocks = [];

            while (!blockStream.eof()) {
                blocks.push(blockStream.next());
            }
            
            expect(blocks).to.deep.equal(example1BlockOutput);
        });

        it('should skip multi line comments', () => {
            const blocks: RaBlock[] = [];

            while (!blockStream.eof()) {
                blocks.push(blockStream.next());
            }
            expect(blocks[0])
        });
    })

});
