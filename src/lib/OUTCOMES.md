# absQra

- [absQra](#absqra)
  - [Overview](#overview)
    - [current scenario](#current-scenario)
    - [what if](#what-if)
    - [new scenario](#new-scenario)
  - [Outcomes](#outcomes)
    - [for the ones building solutions (__developers / designers__)](#for-the-ones-building-solutions-developers--designers)
    - [for the ones using the solution (__users / programs__)](#for-the-ones-using-the-solution-users--programs)
    - [for the ones studying the data (__researchers / analysts / designers__)](#for-the-ones-studying-the-data-researchers--analysts--designers)
  - [Success](#success)
  - [Progress](#progress)
    - [milestones](#milestones)
    - [adoption](#adoption)
  - [Problem - value](#problem---value)
  - [Value discovery](#value-discovery)

__absQra__ is an input-output framework and delivery platform, aimed at building, delivering and measuring better user experiences in a more streamlined and flexible way.

## Overview

### current scenario

Modern apps are built with concern for the codebase, choice of frameworks and programming languages, storage and delivery solutions. All these concerns make it difficult to overcome unforeseen technical challenges and instrument the end user's experience. Moreover as app development continues choices might change, creating integrations that are difficult to maintain and implement. Which in turn puts extra weight on developers and inability of designers to argue due to tremendous amount of time required for the desired change. Certain apps when gaining popularity will require considerable amount of work to get localized and delivered to other mediums. Cross-medium experiences requiring cross-disciplinary teams and solutions that are getting out of sync in a blink of an eye. It seems that only huge companies, running development sweatshops can afford to build and maintain such experiences. Collaborative tools, especially when they are "becoming collaborative", turning out brittle and even more complicated. Solutions are often developed by commercial entities, that would retain data without providing public access. Validation, input and storage are commonly represented as separate concerns, increasing the amount of code required to maintain the three in sync.

### what if

What if we say that app can be naturally built using patches of code in multiple languages and frameworks, and then delivered across different media? What if we say that any software solution can be represented as sequence of input/output actions? What if we say that revisions of code create different epochs in produced data? What if the data belongs to the public? What if the code belongs to the public? What if schema defines inputs, storage and validation.

### new scenario

Apps are essentially user flows, described as sequences of inputs and outputs with flow control. Sequence defined in such a way becomes a schema for data storage and have enough information to validate the inputs. Parts of the sequences are annotated for use with different medias (and localization). In any part of the sequence data context might be adjusted to implement restrictions and collaborations in a variety of configurations. Sequences can be nested and referred to. Sequences are then published, sequences can be changed, creating new epochs in its data. Data collected by a sequence is anonymized and made publicly available.

## Outcomes

### for the ones building solutions (__developers / designers__)

- design and development workflow focused on user flow, __so that__ developers and designers can better realize what kind of experience they are going to deliver, __so that__ solution is built around user experiences and not technical details of it
- annotations allowing to be specific where necessary or share code across mediums, __so that__ code provides an overview of cross-medium experiences and localizations, __so that__ it is easier to built those
- cascading contexts, __so that__ collaborative and restrictive experiences are easier to build and manage, __so that__ solutions built with absQra are easier to build for multi-user collaborations with security in mind
- references to external sequences, __so that__ work done well by other sequence designers can be reused, __so that__ better design patterns are used more often and collaboratively improved by sequence designers, __so that__ better patterns are becoming even better
- references to subsequences, __so that__ building a sequence utilizes a common patter - imports, __so that__ it is easier to do code separation and maintain more complex projects
- use of epochs, __so that__ data created by a run of sequence remains in untouched, __so that__ there're less compatibility issues
- contextual adaptation of previous epochs, __so that__ data from previous epochs can be used in new ones, without actual changes, __so that__ security issues can be covered
- use of any code blocks, __so that__ developers can create or use functions using tools and languages more suitable for a particular task, __so that__ developers are not blocked by limitations of _ra_ language

### for the ones using the solution (__users / programs__)

- universal delivery, __so that__ users or other code can use a solution built with absQra using their preferred medium
- solution design more focused on user flows, __so that__ users get better experiences
- reuse of patterns, __so that__ users will encounter certain design patterns more often, __so that__ they are more exposed to it, __so that__ it's easier for them to understand how to use a solution
- transparency, publicity, anonymization, __so that__ solutions are transparent about what data they keep, and what becomes available in public access, __so that__ users are informed and safe

### for the ones studying the data (__researchers / analysts / designers__)

- data is well coded by default, __so that__ there's less inconsistency, __so that__ it requires less steps to clean and refine, __so that__ there's less entries lost in this process
- access to anonymized data from a public sequence, __so that__ they can study well sampled data
- access to anonymized metadata from a public sequence, __so that__ behavioral patterns can be studied as well, __so that__ their design solutions can be further optimized
- configurable contributions for references, __so that__ well designed sequences when used by others will collect data in accordance with their configuration, __so that__ sequences can have a better outreach while cleared from undesired entries, __so that__ samples are bigger and cleaner

## Success

1. A solution implementation in _absQra_ focuses on describing user flows rather than technical details of implementation (signals to noise)
2. A solution implementation in _absQra_ uses annotations
   1. for localization
   2. for delivery on different medium
   3. for cross-media experiences
3. A solution implementation in _absQra_ uses contexts
   1. to restrict access
   2. to create collaborative experiences
   3. to limit data collection by referrals
4. A solution implementation in _absQra_ uses references
   1. to skip implementation of certain design pattern
   2. to maintain better code structure
   3. An implementation of certain design pattern is rather referred to than re-implemented
      1. by the same designer
      2. by other designers
5. Republication of sequences creates epochs in data
   1. and doesn't require additional changes
   2. and allows adaptation of data from previous epochs
6. Small code blocks in other languages are compact and functional
7. A solution implementation in _absQra_ can be delivered to number of mediums
   1. without any additional change
   2. with annotations to improve experience
8. Collected data not compromising users' privacy
   1. accessed by audience and researchers
   2. exported
9. Collected data has consistent coding

## Progress

### milestones

1. A software solution using _absQra_ can
   1. be defined
   2. be delivered
   3. collect data
2. Annotations can be handled
   1. for different natural languages
   2. for different delivery mediums
3. Data contexts can be used for a part of a sequence
4. Sequences can be referenced
   1. from other sequences
   2. from sequences of other sequence designers
5. Epochs are handled
   1. in data
   2. in context
6. Runtimes for other languages can be built and deployed
7. Publicity and anonymization
   1. identifying information is not accessible for anyone except the ones who entered it
   2. non-identifying information is publicly accessible
   3. sequence design code is publicly accessible without revealing any security threatening information
8. data is consistently coded
   1. at once
   2. per epoch
   3. in export
   4. across multiple sequence uses

### adoption

1. predefined set of example projects
2. own editors and public tools
3. pilot projects
4. citizen science
5. research projects
6. public software solutions

## Problem - value



## Value discovery
