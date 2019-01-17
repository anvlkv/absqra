import { InputStream } from '../input-stream/input-stream';
import { Token, TokenType } from './token';
import { LineColumnAddress } from '../line-column-address';
import { keyWords } from '../kw';
import { RParenthesisToken } from './tokens/r-parenthesis';
import { LParenthesisToken } from './tokens/l-parenthesis';
import { CommaToken } from './tokens/comma';
import { BackTickToken } from './tokens/back-tick';
import { FWDSlashToken } from './tokens/fwd-slash';
import { GTToken } from './tokens/gt';
import { LSToken } from './tokens/ls';
import { PipeToken } from './tokens/pipe';
import { AsteriskToken } from './tokens/asterisk';
import { BKWSlashToken } from './tokens/bkw-slash';
import { EXCLToken } from './tokens/excl';
import { PercentToken } from './tokens/percent';
import { EQToken } from './tokens/eq';
import { MINToken } from './tokens/min';
import { PlusToken } from './tokens/plus';
import { ColonToken } from './tokens/colon';
import { AtToken } from './tokens/at';
import { IfToken } from './tokens/if';
import { ElseToken } from './tokens/else';
import { SwitchToken } from './tokens/switch';
import { CaseToken } from './tokens/case';
import { DefaultToken } from './tokens/default';
import { FalseToken } from './tokens/false';
import { TrueToken } from './tokens/true';
import { LnToken } from './tokens/ln';
import { LangToken } from './tokens/lang';


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
        return '><|*\\!%=-+:@'.indexOf(ch) >= 0;
    }

    private  isPunct(ch) {
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

    private readNumber(start: LineColumnAddress): Token {
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

    private readIdentity(start: LineColumnAddress): Token {
        const id = this.readWhile(this.isId.bind(this));
        const end: LineColumnAddress = [this.input.line, this.input.col];
        if (this.isKeyword(id)) {
            let tok: Token;
            switch (id) {
                case 'if':
                    tok = new IfToken(start, end);
                    break;
                case 'else':
                    tok = new ElseToken(start, end);
                    break;
                case 'switch':
                    tok = new SwitchToken(start, end);
                    break;
                case 'case':
                    tok = new CaseToken(start, end);
                    break;
                case 'default':
                    tok = new DefaultToken(start, end);
                    break;
                case 'false':
                    tok = new FalseToken(start, end);
                    break;
                case 'true':
                    tok = new TrueToken(start, end);
                    break;
                case 'ln':
                    tok = new LnToken(start, end);
                    break;
                case 'lang':
                    tok = new LangToken(start, end);
                    break;
                default:
                    this.croak(`Unsupported keyword [${id}]`);
            }
            return tok;
        }

        return new Token(
            TokenType.VAR,
            id,
            start,
            end
        );
    }

    private skipComment(start: LineColumnAddress) {
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

    private readPunct(start: LineColumnAddress): Token {
        const val = this.input.next();
        const end: LineColumnAddress = [this.input.line, this.input.col];
        let token: Token;

        switch (val) {
            case ')':
                token = new RParenthesisToken(start, end);
                break;
            case '(':
                token = new LParenthesisToken(start, end);
                break;
            case ',':
                token = new CommaToken(start, end);
                break;
            case '`':
                token = new BackTickToken(start, end);
                break;
            case '/':
                token = new FWDSlashToken(start, end);
                break;
            default:
                this.croak(`Unsupported punctuation: [${val}]`);
        }
        return token;
    }

    private readOpChar(start: LineColumnAddress): Token {
        const val = this.input.next();
        const end: LineColumnAddress = [this.input.line, this.input.col];
        let token: Token;

        switch (val) {
            case '>':
                token = new GTToken(start, end);
                break;
            case '<':
                token = new LSToken(start, end);
                break;
            case '|':
                token = new PipeToken(start, end);
                break;
            case '*':
                token = new AsteriskToken(start, end);
                break;
            case '\\':
                token = new BKWSlashToken(start, end);
                break;
            case '!':
                token = new EXCLToken(start, end);
                break;
            case '%':
                token = new PercentToken(start, end);
                break;
            case '=':
                token = new EQToken(start, end);
                break;
            case '-':
                token = new MINToken(start, end);
                break;
            case '+':
                token = new PlusToken(start, end);
                break;
            case ':':
                token = new ColonToken(start, end);
                break;
            case '@':
                token = new AtToken(start, end);
                break;
            default:
                this.croak(`Unsupported operator: [${val}]`);
        }
        return token;
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
        if (this.isPunct(ch)){
            if (this.isInlineLineContent(ch)) {
                return this.readInlineContent(start);
            }
            else {
                return this.readPunct(start);
            }
        }

        if (this.isOpChar(ch)) {
            return this.readOpChar(start);
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