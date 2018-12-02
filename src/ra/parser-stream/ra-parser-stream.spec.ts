// import { RaParserStream } from './ra-parser-stream';
// import * as fs from "fs";
// import { expect } from 'chai';
// import { exampleParserOutput } from '../example.parser-output.spec';
// import { RaTokenStream } from '../token-stream/ra-token-stream';
// import { RaInputStream } from '../input-stream/ra-input-stream';
//
//
// describe('RaParserStream', () => {
//     let inputStream: RaInputStream;
//     let tokenStream: RaTokenStream;
//     let parserStream: RaParserStream;
//     before((done) => {
//         fs.readFile('src/ra/example.ra', 'utf8', (err, fileContents) => {
//             if (err) throw err;
//             inputStream = new RaInputStream(fileContents);
//             tokenStream = new RaTokenStream(inputStream);
//             parserStream = new RaParserStream(tokenStream);
//             done();
//         });
//     });
//
//     it('should create stream', () => {
//         expect(parserStream, 'stream created').to.exist;
//     });
//
//     it('should read parser stream to programme', () => {
//         expect(parserStream.parse(), 'stream total').to.deep.equal(exampleParserOutput);
//     });
// });