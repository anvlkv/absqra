
# absQra

- [absQra](#absqra)
  - [__ra__ - *rationale* - the programming language](#ra---rationale---the-programming-language)
    - [__ra__.principles](#raprinciples)
    - [__ra__.functional units](#rafunctional-units)
      - [items](#items)
      - [sequences](#sequences)
      - [inputs](#inputs)
      - [rules](#rules)
      - [contexts](#contexts)
      - [clients](#clients)
      - [identities](#identities)
    - [__ra__.features](#rafeatures)
      - [contents with metadata](#contents-with-metadata)
      - [environments](#environments)
      - [references](#references)
  - [__Q__ - _questioning_ - platform | motivation](#q---questioning---platform--motivation)
    - [principles](#principles)
    - [implementations](#implementations)
      - [transparency](#transparency)
      - [privacy](#privacy)
      - [experimentation](#experimentation)
  - [__abs__ - _abstract_ - the runtime](#abs---abstract---the-runtime)

## __ra__ - *rationale* - the programming language

### __ra__.principles

1. structure is everything
2. program either accepts or moderates input
3. output requires input
4. conten is anything

### __ra__.functional units

#### items

Items are basic units of output, they are named and produce values by accepting __inputs__.

Outputs and therefore __items__ can be reused later on.

Items are populating __contexts__.

Items can be nested.

Items are always part of a __sequence__.

#### sequences

Sequences are like files, they can be read from top to bottom and their only purpose is to group items.

Sequences are named.

Sequences are essentially same as __items__.

#### inputs

Inputs are units that actually accept values.

Inputs can be made convenient for different clients.

Inputs accept rules and default values.

_By default_ inputs are plain text.

#### rules

Rules are telling whether an input can accept certain value, and how to handle acceptance.

A set of rules is also used to determine what's a convenient input.

Rules determine how __clients__ or __items__ are part of a __context__.

Rules determine flow of a __sequence__.

#### contexts

Contexts describe how multitudes of __items__ are related to multitudes of __clients__.

Contexts are configured with __rules__.

_By default_ one client can fill in one set of items.

#### clients

Clients are any kinds of clients where items can accept inputs. Could be browser, could be drone, could be another server.

Clients can be identified using output from __items__ that provide __identity__.

_By default_ client is a web browser.

#### identities

Identities are unique within a __sequence__ of __items__.

Identities are meant to distinguish __clients__.

### __ra__.features

#### contents with metadata

Contents can use different annotations, for example different natural languages (russian, english, japanese).

Contents can be written in different artificial languages (python, typescript, markdown).

Contents can be meant for different __environments__.

#### environments

Environments are distinguishing different kinds of __clients__.

Environments put a limitation on contents. For example there's no way to run python code in browser same as markdown doesn't do much on a server

#### references

References are meant to include items, rules or entire sequences from another source.

References will contribute to output of both original and referring sequences.

Referring __sequence__ may use a reference in a modified __context__.

## __Q__ - _questioning_ - platform | motivation

### principles

1. transparency
2. privacy
3. experimentation

### implementations

#### transparency

1. Data collected by __sequences__ is publicly available and can be used in another __sequence__ by __referring__ it.
2. __Sequence__ designs (the *.ra files) are publicly available and can be __refereed__ to by other __sequences__.

#### privacy

1. Data is anonymous. Both users and designers are encouraged to avoid entering or asking for identifying information. In public parts of __sequences__.
2. Sometimes identifying information is needed. For example when item says to provide __identity__ of a __client__. In such case information is hidden and not available to anything other than __runtime__.
3. Information exposed to __runtime__ is an __identity__ unique per consuming __sequence__ and __client__.

#### experimentation

1. The __references__ to sequences are meant to mix, match and contribute.
2. The __contents__ are pieces of any code that can be run
3. The __contexts__ allowing different modes of entry and collaboration

## __abs__ - _abstract_ - the runtime

Runtime of a __sequence__ is spread between a server running the __sequence__ and its __clients__.

The way and choice of __sequence__'s __items__ depends on __client__ and therefore on __environments__, __contexts__ and __rules__ determined for given __item__ or __content__.

Particular implementation of runtime depends on __client__ that is accessing it.
