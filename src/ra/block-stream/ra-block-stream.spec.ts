import { expect } from 'chai';
import { RaInputStream } from '../input-stream/ra-input-stream';
import * as fs from "fs";
import { RaBlockStream } from './ra-block-stream';
import { RaLineStream } from '../line-stream/ra-line-stream';


describe ('RaBlockStream', () => {
    let blockStream: RaBlockStream;
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

});
