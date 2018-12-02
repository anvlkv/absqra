import { RaBlockStream } from './ra-block-stream';
import { RaInputStream } from '../input-stream/ra-input-stream';
import { RaTypes } from '../types.enum';
import { RaToken } from '../token-stream/token';
// import { RaTokenStream } from '../token-stream/ra-token-stream';
import { RaLine } from '../line-stream/line';

export type LineColumnAddress = [number, number];

export enum BlockType {
    CONTENT = 'content',
    DECLARE = 'declare',
    INVOKE = 'invoke'
}

export class RaBlock {
    public type = RaTypes.BLOCK;
    constructor(
        // public readonly blockType: BlockType,
        // public readonly blockHeader: RaToken[],
        public readonly content: RaLine[],
        public readonly start: LineColumnAddress,
        public readonly end: LineColumnAddress
    ) {
    }
}