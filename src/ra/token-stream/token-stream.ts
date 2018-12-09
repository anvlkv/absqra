import { InputStream } from '../input-stream/input-stream';
import { Token, TokenType } from './token';
import { LineColumnAddress } from '../line-column-address';
import { keyWords } from '../kw';


export class TokenStream {
    private current = null;
    private keywords = keyWords;

    constructor(
        private input: InputStream
    ) {
    }

    private  isKeyword(x) {
        return this.keywords.indexOf(x) >= 0;
    }

    private  isDigit(ch) {
        return /[0-9]/i.test(ch);
    }

    private  isIdStart(ch) {
        return /[a-z]/i.test(ch);
    }

    private  isId(ch) {
        return this.isIdStart(ch) || '0123456789#$%^'.indexOf(ch) >= 0;
    }

    private  isOpChar(ch) {
        return '><|*/!%=-+:@'.indexOf(ch) >= 0;
    }

    private  isPunc(ch) {
        return ',`/()'.indexOf(ch) >= 0;
    }

    private  isWhitespace(ch) {
        return /[ \t\n]/.test(ch);
    }

    private isInlineLineContent(lineAhead: string):boolean {
        if (lineAhead !== '`') {
            return false;
        }

        let chShift = 1;
        const regExp = /`.*`/;
        while (!this.input.eoLine(chShift) && !regExp.test(lineAhead)) {
            lineAhead += this.input.peek(chShift++);
        }
        return regExp.test(lineAhead);
    }

    private readInlineContent(start: LineColumnAddress): Token {
        let content = this.readWhile((ch, str) => {
            return !str || str[0] === '`' && (ch !== '`' || str[str.length - 1] === '\\')
        });
        this.input.next(); // skip last tick
        content = content
            .replace(/^`/, '')
            .replace(/\\`/, '`');
        return new Token(
            TokenType.INLINE_CONTENT,
            content,
            start,
            [this.input.line, this.input.col],
        );
    }

    private isRegExpStart(ch: string): boolean {
        if (ch === '/') {
            return !this.isComment(ch);
        }
        return false;
    }

    private isRegExpComplete(expression: string): boolean {
        const chars = expression.split('');
        return chars.length > 2 &&
            chars[0] === '/' &&
            chars[chars.length - 1] === '/' &&
            chars[chars.length - 2] !== '\\' &&
            chars[1] !== '/';
    }

    private readRegExp(start: LineColumnAddress): Token {
        const value = this.readWhile((ch, total) => {
            return !this.isRegExpComplete(total);
        });
        return new Token(
            TokenType.REG_EXP,
            value,
            start,
            [this.input.line, this.input.col]
        );
    }

    private isComment(ch: string): boolean {
        if (ch === '/') {
            if ('/*'.indexOf(this.input.peek(1)) >= 0) {
                return true;
            }
        }
        return false;
    }

    readWhile(fn: (ch: string, total?: string) => boolean, peekShift?) {
        let str = '';
        while (!this.input.eof() &&
            fn(this.input.peek(peekShift), str)
        ) {
            str += this.input.next();
        }
        return str;
    }

    readNumber(start: LineColumnAddress): Token {
        let hasDot = false;
        const number = this.readWhile((ch) => {
            if (ch === '.') {
                if (hasDot) return false;
                hasDot = true;
                return true;
            }
            return this.isDigit(ch);
        });
        return new Token(
            TokenType.NUM,
            parseFloat(number),
            start,
            [this.input.line, this.input.col]
        );
    }

    readIdentity(start: LineColumnAddress): Token {
        const id = this.readWhile(this.isId.bind(this));
        return new Token(
            this.isKeyword(id) ? TokenType.KW : TokenType.VAR,
            id,
            start,
            [this.input.line, this.input.col],
        );
    }

    skipComment(start: LineColumnAddress) {
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
        this.input.next();

        return multiLineCommentClosed ? this.readNext() : new Token(
            TokenType.ML_COMMENT_START,
            null,
            start,
            [this.input.line, this.input.col]
        );
    }

    readNext(): Token {
        this.readWhile(this.isWhitespace.bind(this));
        if (this.input.eof()) return null;
        const ch = this.input.peek();
        const start: LineColumnAddress = [this.input.line, this.input.col];

        if (this.isComment(ch)) {
            return this.skipComment(start)
        }
        if (this.isRegExpStart(ch)) {
            return this.readRegExp(start);
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
                return new Token(
                    TokenType.PUNCT,
                    this.input.next(),
                    start,
                    [this.input.line, this.input.col],
                );
            }
        }

        if (this.isOpChar(ch)) {
            return new Token(
                TokenType.OP,
                this.readWhile(this.isOpChar.bind(this)),
                start,
                [this.input.line, this.input.col]
            );
        }

        if (!this.isWhitespace(ch)) {
            this.input.croak(`Can't handle character: [\`${ch}\`]`);
        }
        else {
            return this.readNext();
        }
    }

    peek() {
        return this.current || (this.current = this.readNext());
    }

    next(): Token {
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