import { LineColumn, LineColumnAddress } from './line-column-address';
import { expect } from 'chai';


describe('LineColumn', () => {
    let lc01: LineColumnAddress = [0, 1],
        lc20: LineColumnAddress = [2, 0],
        lc22: LineColumnAddress = [2, 2];

    it('should check if address is ahead by col', () => {
        expect(LineColumn.isAhead(lc22, lc20)).to.be.true;
        expect(LineColumn.isAhead(lc20, lc22)).to.be.false;
    });

    it('should check if address is ahead by line', () => {
        expect(LineColumn.isAhead(lc20, lc01)).to.be.true;
        expect(LineColumn.isAhead(lc01, lc20)).to.be.false;
    });

    it('should check if address is behind by col', () => {
        expect(LineColumn.isBehind(lc20, lc22)).to.be.true;
        expect(LineColumn.isBehind(lc22, lc20)).to.be.false;

    });

    it('should check if address is behind by line', () => {
        expect(LineColumn.isBehind(lc01, lc20)).to.be.true;
        expect(LineColumn.isBehind(lc20, lc01)).to.be.false;
    });


});