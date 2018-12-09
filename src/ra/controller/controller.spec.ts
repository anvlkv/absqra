import { Controller } from './controller';
import { use, expect, spy } from 'chai';
import spies = require('chai-spies');
import { InputStream } from '../input-stream/input-stream';
import { LineStream } from '../line-stream/line-stream';
import { BlockStream } from '../block-stream/block-stream';
import { Parser } from '../parser/parser';
import * as fs from 'fs';
import { Expr } from '../expr/expr';

use(spies);

describe('Controller', () => {
    let controller: Controller;
    let inputSpy, linesSpy, blocksSpy, parserSpy;
    let fileContent : string;
    before((done) => {
        inputSpy = spy.on(InputStream.prototype, 'next');
        linesSpy = spy.on(LineStream.prototype, 'next');
        blocksSpy = spy.on(BlockStream.prototype, 'next');
        parserSpy = spy.on(Parser.prototype, 'parseProgram');
        fs.readFile('src/ra/example.ra', 'utf8', (err, c) => {
            if (err) throw err;
            fileContent = c;
            done();
        });
    });

    afterEach(() => {
        spy.restore(InputStream.prototype, 'next');
        spy.restore(LineStream.prototype, 'next');
        spy.restore(BlockStream.prototype, 'next');
        spy.restore(Parser.prototype, 'parseProgram');
    });

    beforeEach(() => {
        controller = new Controller();
        controller.code = fileContent;
    });

    it('should create input stream', () => {
        expect(controller.parseChars() instanceof InputStream).to.be.true;
    });

    it('should create line stream', () => {
        const result = controller.parseLines();
        expect(inputSpy).to.have.been.called;
        expect(result instanceof LineStream).to.be.true;
    });

    it('should create block stream', () => {
        const result = controller.parseBlocks();
        expect(inputSpy).to.have.been.called;
        expect(linesSpy).to.have.been.called;
        expect(result instanceof BlockStream).to.be.true;
    });

    it('should parse program', () => {
        const result =  controller.parseCode();
        expect(inputSpy).to.have.been.called;
        expect(linesSpy).to.have.been.called;
        expect(blocksSpy).to.have.been.called;
        expect(parserSpy).to.have.been.called;
        expect(result instanceof Expr).to.be.true;
    });
});
