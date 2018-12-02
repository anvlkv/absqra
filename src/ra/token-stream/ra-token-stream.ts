import { RaInputStream } from '../input-stream/ra-input-stream';
import { RaToken, TokenType } from './token';
import { LineColumnAddress } from '../block-stream/block';


export class RaTokenStream {
    private current = null;
    private keywords = [
        'sequence',
        'question',
        'task',
        'if',
        'else',
        'else if',
        'switch',
        'case',
        'default'
    ];

    constructor(
        private input: RaInputStream
    ) {
    }

    isKeyword(x) {
        return this.keywords.indexOf(x) >= 0;
    }

    isDigit(ch) {
        return /[0-9]/i.test(ch);
    }

    isIdStart(ch) {
        return /[a-z]/i.test(ch);
    }

    isId(ch) {
        return this.isIdStart(ch) || '0123456789#$%^'.indexOf(ch) >= 0;
    }

    isOpChar(ch) {
        return '>|!`=+:@'.indexOf(ch) >= 0;
    }

    isPunc(ch) {
        return ',`'.indexOf(ch) >= 0;
    }

    isWhitespace(ch) {
        return /[ \t\n]/.test(ch);
    }

    private isInlineLineContent(lineAhead: string) {
        let chShift = 1;
        while (!this.input.eoLine(chShift)) {
            lineAhead += this.input.peek(chShift++);
        }
        return /`.*`/.test(lineAhead);
    }

    private readInlineContent(start: LineColumnAddress): RaToken {
        let content = this.readWhile((ch, str) => {
            return !str || str[0] === '`' && ch !== '`'
        });
        content += this.input.next();
        return new RaToken(
            TokenType.INLINE_CONTENT,
            content,
            start,
            [this.input.line, this.input.col],
        );
    }

    readWhile(fn: (ch: string, read?: string) => boolean, peekShift?) {
        let str = '';
        while (!this.input.eof() && fn(this.input.peek(peekShift), str))
            str += this.input.next();
        return str;
    }

    readNumber(start: LineColumnAddress): RaToken {
        let hasDot = false;
        const number = this.readWhile((ch) => {
            if (ch === '.') {
                if (hasDot) return false;
                hasDot = true;
                return true;
            }
            return this.isDigit(ch);
        });
        return new RaToken(
            TokenType.NUM,
            parseFloat(number),
            start,
            [this.input.line, this.input.col]
        );
    }

    readIdentity(start: LineColumnAddress): RaToken {
        const id = this.readWhile(this.isId.bind(this));
        return new RaToken(
            this.isKeyword(id) ? TokenType.KW : TokenType.VAR,
            id,
            start,
            [this.input.line, this.input.col],
        );
    }

    // readIndent(start: LineColumnAddress) {
    //     const indent = this.readWhile((ch) => ch === Environment.indentationCharacter);
    //     return new RaToken(
    //         start,
    //         [this.input.line, this.input.col],
    //         TokenType.INDENT,
    //         indent.length / Environment.indentationWidth
    //     )
    // }

    // peekIndent(): number {
    //
    // }

    // readContent(start: LineColumnAddress): RaToken {
    //     let isMultiLine = false;
    //     let content = '';
    //     let contentHeader;
    //     this.input.next();
    //     while (!this.input.eof()) {
    //         let ch = this.input.next();
    //         content += ch;
    //         if (ch === '`' && !isMultiLine) {
    //             break;
    //         }
    //         if (this.input.eoLine() && !isMultiLine) {
    //             isMultiLine = true;
    //         }
    //
    //         if (this.input.col > 0 || this.peekIndent()) {
    //             content += ch;
    //         }
    //         // el
    //     }
    // }

    skipComment() {
        const commentType = this.input.peek(1);
        let multiLineCommentClosed = true;
        if (commentType === '/') {
            this.readWhile((ch) => {
                return ch !== '\n'
            });
        }
        else if (commentType === '*') {
            this.readWhile((ch) => {
                multiLineCommentClosed = ch === '*' && this.input.peek(1) === '/';
                if(multiLineCommentClosed) {
                    this.input.next();
                }
                return !multiLineCommentClosed;
            });
        }
        else {
            this.croak(`Invalid comment: expected * or /, got [${commentType}]`)
        }

        if (!multiLineCommentClosed) {
            this.croak(`Multi line comment not closed`);
        }
        this.input.next();
    }



    readNext(): RaToken {
        this.readWhile(this.isWhitespace.bind(this));
        if (this.input.eof()) return null;
        const ch = this.input.peek();
        const start: LineColumnAddress = [this.input.line, this.input.col];

        if (ch === '/') {
            this.skipComment();
            return this.readNext();
        }
        if (this.isDigit(ch)) {
            return this.readNumber(start);
        }
        if (this.isIdStart(ch)) {
            return this.readIdentity(start);
        }
        if (this.isPunc(ch)){
            if (this.isInlineLineContent(ch)) {
                return this.readInlineContent(start);
            }
            else {
                return new RaToken(
                    TokenType.PUNCT,
                    this.input.next(),
                    start,
                    [this.input.line, this.input.col],
                );
            }
        }

        if (this.isOpChar(ch)) {
            return new RaToken(
                TokenType.OP,
                this.readWhile(this.isOpChar.bind(this)),
                start,
                [this.input.line, this.input.col]
            );
        }

        if (!this.isWhitespace(ch)) {
            this.input.croak(`Can't handle character: ''${ch}'`);
        }
        else {
            return this.readNext();
        }
    }

    peek() {
        return this.current || (this.current = this.readNext());
    }

    next(): RaToken {
        const tok = this.current;
        this.current = null;
        return tok || this.readNext();
    }

    eof() {
        return this.peek() === null;
    }

    croak(err) {
        this.input.croak(err);
    }

}