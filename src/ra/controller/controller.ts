import { Expr } from '../expr/expr';
import { InputStream } from '../input-stream/input-stream';
import { LineStream } from '../line-stream/line-stream';
import { BlockStream } from '../block-stream/block-stream';
import { Parser } from '../parser/parser';
import { Program } from '../parser/program';


export class Controller {
    private program: Program = null;

    public code: string;

    constructor(
        private parser = new Parser()
    ) {}


    public parseCode(): Program {
        const blockStream = this.parseBlocks();
        this.program = this.parser.parseProgram(blockStream);
        return this.program;
    }

    public parseChars(): InputStream {
        return new InputStream(this.code)
    }

    public parseLines(): LineStream {
        return new LineStream(this.parseChars())
    }

    public parseBlocks(): BlockStream {
        return new BlockStream(this.parseLines())
    }
}