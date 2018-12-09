import { Block, BlockType } from '../block-stream/block';
import { BlockStream } from '../block-stream/block-stream';
import { Program } from './program';
import { ContentExpr } from '../expr/content';
import { TokenType } from '..';
import { DeclareExpr } from '../expr/declare';
import { GroupingExpr } from '../expr/grouping';
import { InvokeExpr } from '../expr/invoke';
import { ChoiceExpr } from '../expr/choice';
import { AssignExpr } from '../expr/assign';
import { ValidatorExpr } from '../expr/validator';
import { LogicalExpr } from '../expr/logical';


export class Parser {
    constructor (){}



    private parseContentBlock(block: Block): Program {
        const contentExpr = new ContentExpr(...block.tokens);
        if (block.tokens.length === 1 && block.tokens[0].tokenType === TokenType.INLINE_CONTENT) {
            contentExpr.value = block.tokens[0].value;
        }
        else {
            contentExpr.value = block.content.reduce((output, line) => {
                line.indent -= block.level;
                output += line.value;
                return output;
            }, '');
        }

        if (contentExpr.value) {
            const program = new Program();

            program.root = contentExpr;

            return program;
        }
        else {
            throw new TypeError(`Empty content not allowed [${block.start[0]}:${block.start[1]}]`);
        }
    }

    private parseDeclarationBloc(block: Block): Program {
        let args = null;
        if (block.tokens.length > 3) {
            const exprTokens = block.tokens.slice(3, block.tokens.length);
            if (GroupingExpr.isGroupingExpr(exprTokens)) {
                args = new GroupingExpr(...exprTokens);
            }
            else {
                throw new SyntaxError(`Unexpected statement after block declaration: [${exprTokens.map(t => t.value).join(' ')}] [${block.start[0]}:${block.start[1]}]`)
            }
        }
        const declarationExpr = new DeclareExpr(
            args,
            ...block.tokens
        );

        const program = new Program();
        program.root = declarationExpr;
        program.children = this.parseChildren(block);

        return program;
    }

    private parseInvocationBlock(block: Block): Program {
        const program = new Program();
        if (InvokeExpr.isInvokeExpr(block.tokens)) {
            program.root = new InvokeExpr(...block.tokens)
        }
        else if (ChoiceExpr.isChoiceExpr(block.tokens)) {
            program.root = new ChoiceExpr(...block.tokens);
        }
        else if (ValidatorExpr.isValidatorExpr(block.tokens)) {
            let assignment: AssignExpr = null;
            const valueTokens = block.tokens.slice(1, block.tokens.length);
            if (AssignExpr.isAssignExpr(valueTokens)) { // ! minLength = 1
                assignment = new AssignExpr(...valueTokens);
            }
            program.root = new ValidatorExpr(
                assignment,
                ...block.tokens
            )
        }
        else if (LogicalExpr.isLogicalExpr(block.tokens)) {

        }

        program.children = this.parseChildren(block);


        return program;
    }

    private parseChildren(block: Block): Program[] {
        return block.children ? block.children.map(child => this.parseBlock(child)) : null;
    }

    parseProgram(blockStream: BlockStream): Program {
        const program = new Program();

        while (!blockStream.eof()) {
            program.children.push(this.parseBlock(blockStream.next()));
        }

        return program;
    }

    parseBlock(block: Block): Program {
        switch (block.blockType) {
            case BlockType.CONTENT:
                return this.parseContentBlock(block);
            case BlockType.DECLARE:
                return this.parseDeclarationBloc(block);
            case BlockType.INVOKE:
                return this.parseInvocationBlock(block);
            default:
                throw new TypeError(`Unexpected type of block: [${block.type}] [${block.start[0]}:${block.start[1]}]`)
        }
    }
}