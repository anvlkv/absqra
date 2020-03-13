
# absQra

<table>
    <tr>
        <td>
            <i>abs</i>
        </td>
        <td>
            abstract
        </td>
    </tr>
    <tr>
        <td>
            <i>Q</i>
        </td>
        <td>
            questioning
        </td>
    </tr>
    <tr>
        <td>
            <i>ra</i>
        </td>
        <td>
            rationale
        </td>
    </tr>
</table>

## __ra__ - *rationale* - the programming langiuage

### __ra__.principles

1. structure is everything
2. program either accepts or moderates input
3. output requires input
4. conten is anything

### __ra__.fucntional units

#### items

Items are basic units of output, they are named and produce values by accepting __inputs__.

Values and therefore items can be reused later on.

Items are poppulating __contexts__.

Items can be nested.

Items are always part of a __sequence__.

#### sequences

Sequences are like files, they can be read from top to bottom and their only purpose is to group items.

Sequences are named.

#### inputs

Inputs are units that actually accept values.

Inputs can be made convenient for different consumers.

Inputs accept rules and default values.

_By default_ inputs are plain text.

#### rules

Rules are telling whether an input can accept certain value, and how to handle acceptance.

A set of rules is also used to determine what's a convenient input.

Rules determine how __consumers__ or __items__ are part of a __context__.

#### contexts

Contexts describe how multitudes of __items__ are related to multitudes of __consumers__.

Contexts are configured with __rules__.

_By default_ one consumer can fill in one set of items.

#### consumers

Consumers are any kinds of clients where items can accept inputs. Could be browser, could be drone, could be another server.

Consumers can be identified using values __items__ that provide __identity__.

_By default_ consumer is a web browser.

#### identities

Identites are unique within a __sequence__ of __items__.

Identites are meant to distinguish __consumers__.

### __ra__.features

#### contents with metadata

Contents can use different annotations, for example different natural languages (russian, english, japanese).

Contents can be written in different artificial languages (python, typescript, markdown).

Contents can be meant for different __environments__.

#### environments

Environments are distinguishing different kinds of __consumers__.

Environments put a limitaion on contents. For example there's no way to run python code in browser same as markdown doesn't do much on a server

#### refernces

References are meant to include items, rules or entire sequences from another source

Refernces will contribute to output of both original and referring sequences

## __Q__ - _questioning_ - why all this

1. transparency
2. privacy
3. expirementation

## __abs__ - _abstract_ - the runtime

Runtime of a __sequence__ is spread between the server running the __sequence__ and its __consumers__.

The way and choice of __sequnce__'s __items__ depends on __consumer__ and therefore on __environments__, __context__ and __rules__ determined for given __item__ or __content__.
