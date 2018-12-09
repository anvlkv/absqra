import { expect } from 'chai';


export function expectDeeperThanDeepEqual(
    blocks: Array<any>,
    exampleBlockOutput: Array<any>,
    expectation: (val: any, i: number, prop: string) => void,
    done?: Function) {
    exampleBlockOutput.forEach((b, i) => {
        Object.keys(b).forEach(prop => {
            expectation(blocks[i][prop], i, prop);
        });
    });
    done();
}