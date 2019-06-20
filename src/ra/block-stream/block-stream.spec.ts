import { expect } from 'chai';
import { InputStream } from '../input-stream/input-stream';
import * as fs from "fs";
import { BlockStream } from './block-stream';
import { LineStream } from '../line-stream/line-stream';
import { exampleBlockOutput } from './example.block-output.spec';
import { expectDeeperThanDeepEqual } from '../../spec-utils.spec';
import { Block } from './block';
import { use } from 'chai';
import { example1Output } from './example1.block-output.spec';
import chaiExclude  = require('chai-exclude');
import chaiJsonEqual  = require('chai-json-equal');

use(chaiExclude);
use(chaiJsonEqual);

describe ('BlockStream', () => {
    let blockStream: BlockStream;
    let inputStream: InputStream;
    let lineStream: LineStream;
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
            inputStream = new InputStream(fileContent, null, '[src/ra/example.ra] ');
            lineStream = new LineStream(inputStream);
            blockStream = new BlockStream(lineStream);
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

        it('should parse blocks as expected', (done) => {
            const result = {
                blocks: []
            };

            while (!blockStream.eof()) {
                result.blocks.push(blockStream.next());
            }
            expect(result, 'JSON result').to.be.jsonEqual(example1Output);
            // // expect(blocks).to.deep.equal(example1BlockOutput);
            // expectDeeperThanDeepEqual(blocks, exampleBlockOutput, (val, i, prop) => {
            //     if (prop === 'content') {
            //         expect(blocks[i][prop], `${i}:${prop}`).excluding('_tokens').to.deep.equal(exampleBlockOutput[i][prop]);
            //     }
            //     else {
            //         expect(blocks[i][prop], `${i}:${prop}`).to.deep.equal(exampleBlockOutput[i][prop]);
            //     }
            // }, done);
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
            inputStream = new InputStream(fileContent, null, '[src/ra/example1.ra] ');
            lineStream = new LineStream(inputStream);
            blockStream = new BlockStream(lineStream);
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

            expect(blocks.length).to.be.equal(3);
        });

        it('should parse blocks as expected', () => {
            const result = {
                blocks: []
            };

            while (!blockStream.eof()) {
                result.blocks.push(blockStream.next());
            }

            expect(result, 'JSON result').to.be.jsonEqual(example1Output);
        });
    })

});
