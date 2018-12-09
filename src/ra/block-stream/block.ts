import { BlockStream } from './block-stream';
import { RaTypes } from '../types.enum';
import { Line } from '../line-stream/line';
import { VirtualLineStream } from '../line-stream/virtual-line-stream';
import { LineColumnAddress } from '../line-column-address';
import { Token } from '../token-stream/token';

export enum BlockType {
    CONTENT = 'content',
    DECLARE = 'declare',
    INVOKE = 'invoke'
}

export class Block {
    public type = RaTypes.BLOCK;

    public readonly level: number;
    public readonly children: Block[];

    public get tokens(): Token[] {
        return this.content[0].tokens;
    }

    public get start(): LineColumnAddress {
        return this.content[0].start;
    }

    public get end(): LineColumnAddress {
        return this.content[this.content.length - 1].end;
    }

    constructor(
        public readonly blockType: BlockType,
        public readonly content: Line[]
    ) {
        this.level = this.content[0].indent;

        if (this.blockType !== BlockType.CONTENT) {
            this.children = this.parseChildren();
        }
    }

    private parseChildren() {
        const virtualLineStream = new VirtualLineStream(this.content.slice(1, this.content.length));
        const childrenStream = new BlockStream(virtualLineStream, this.level + 1);
        const children = [];
        while (!childrenStream.eof()) {
            children.push(childrenStream.next())
        }
        return children.length ? children : null;
    }
}