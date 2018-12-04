import { RaBlockStream } from './ra-block-stream';
import { RaTypes } from '../types.enum';
import { RaLine } from '../line-stream/line';
import { VirtualLineStream } from '../line-stream/virtual-line-stream';
import { LineColumnAddress } from '../line-column-address';
import { RaToken } from '../token-stream/token';

export enum BlockType {
    CONTENT = 'content',
    DECLARE = 'declare',
    INVOKE = 'invoke'
}

export class RaBlock {
    public type = RaTypes.BLOCK;

    public readonly level: number;
    public readonly children: RaBlock[];

    public get tokens(): RaToken[] {
        return this.content[0].tokens;
    }

    constructor(
        public readonly blockType: BlockType,
        public readonly content: RaLine[],
        public readonly start: LineColumnAddress,
        public readonly end: LineColumnAddress
    ) {
        this.level = this.content[0].indent;

        if (this.blockType !== BlockType.CONTENT) {
            this.children = this.parseChildren();
        }
    }

    private parseChildren() {
        const virtualLineStream = new VirtualLineStream(this.content.slice(1, this.content.length));
        const childrenStream = new RaBlockStream(virtualLineStream, this.level + 1);
        const children = [];
        while (!childrenStream.eof()) {
            children.push(childrenStream.next())
        }
        return children.length ? children : null;
    }
}