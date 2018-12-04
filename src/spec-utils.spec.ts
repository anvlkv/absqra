import { expect } from 'chai';


export function expectDeeperThanDeepEqual(blocks, exampleBlockOutput) {
    exampleBlockOutput.forEach((b, i) => {
        Object.keys(b).forEach(prop => {
            expect(blocks[i][prop]).to.deep.equal(exampleBlockOutput[i][prop]);
        });
    });
}