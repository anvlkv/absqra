import { Block } from '../block-stream/block';
import { Parser } from './parser';
import { Expr } from '../expr/expr';
import { Controller } from '../controller/controller';
import * as fs from 'fs';
import { expect } from 'chai';
import { Program } from './program';
import { BlockStream } from '..';


describe('Parser', () => {
    let parser: Parser;
    let block: Block;
    let controller: Controller;
    before(() => {
        parser = new Parser();
        controller = new Controller();
    });

    it('should parse content block', (done) => {
        fs.readFile('src/ra/example-content-block.ra', 'utf8', (err, c) => {
            if (err) throw err;
            controller.code = c;
            const parsed = parser.parseBlock(controller.parseBlocks().next());
            expect(parsed).to.be.instanceof(Program);
            done();
        });
    });

    it('should parse invocation block', (done) => {
        fs.readFile('src/ra/example-invoke-block.ra', 'utf8', (err, c) => {
            if (err) throw err;
            controller.code = c;
            const parsed = parser.parseBlock(controller.parseBlocks().next());
            expect(parsed).to.be.instanceof(Program);
            done();
        });
    });

    context('with src/ra/example-declare-block.ra', () => {
        let blk: BlockStream;
        before((done) => {
            fs.readFile('src/ra/example-declare-block.ra', 'utf8', (err, c) => {
                if (err) throw err;
                controller.code = c;
                blk = controller.parseBlocks();
                done();
            });
        });

        it('should parse declaration block', () => {
            const parsed = parser.parseBlock(blk.next());
            expect(parsed).to.be.instanceof(Program);
        });

        it('should parse declaration block with arguments declaration', () => {
            const parsed = parser.parseBlock(blk.next());
            expect(parsed).to.be.instanceof(Program);
        });
    });


    it('should parse block and it\'s children', (done) => {
        fs.readFile('src/ra/example-invoke-block.ra', 'utf8', (err, c) => {
            if (err) throw err;
            controller.code = c;
            const parsed = parser.parseBlock(controller.parseBlocks().next());
            expect(parsed).to.be.instanceof(Program);
            done();
        });
    });


});
