# Examples of applications in __absQra__

- [Examples of applications in __absQra__](#examples-of-applications-in-absqra)
  - [An information sharing website](#an-information-sharing-website)
    - [functional description](#functional-description)
    - [UI requirements](#ui-requirements)
  - [A data entry app](#a-data-entry-app)
    - [functional description](#functional-description-1)
    - [UI requirements](#ui-requirements-1)
  - [An anthropological study](#an-anthropological-study)
    - [functional description](#functional-description-2)
    - [UI requirements](#ui-requirements-2)

## An information sharing website

Imagine a simple website meant to provide some information on a given topic.

Say it'd be about food recipes.

### functional description

It should have a catalogue of recipes categorized by:

- kind of meal
- country of origin
- by restricted ingredients and treatments

It should present a recipe with following contents:

- pictures
- ingredients
- tools
- kind of meal
- country of origin
- steps to prepare

It should have users.

It should have a way to publish a new recipe.

It should have a way to rate a recipe.

It should use the ratings to sort output.

It should have routing.

It should be multilingual.

### UI requirements

It should be convenient to use on both small and large screens.

It should have navigation allowing to reach for the 3 different categorizations, publication and your user account overview.

It should have a simple search.

It should have an advanced search.

## A data entry app

Imagine an app meant to collect data in a structured way.

Say it'd be a noise pollution study.

### functional description

It should run on mobile.

It should register noise pollution data sets as following:

- geographic location
- timestamp
- level of noise
- information about device used for collection
- number of people passed by in 30 seconds

It should show cumulative results on a map.

### UI requirements

It should inform the user about data being collected.

It should show instruction on how to collect noise pollution data (e.g. don't close your mic).

It should collect noise pollution level, location, timestamp, device information data with one click of a button.

It should show instruction on how to count people passed by.

It should show a button that user can click several times during 30s period. They'll do one click per person they see around them.

It should transition to cumulative results.

## An anthropological study

Imagine an app where a volunteer surveyor could register themselves and collect data for social network study.

Say it'd be a study of communities in neighborhoods.

So we want to identify communities and collect some data about their members, so that we can make conclusions about what makes the members part of their communities. We also want to see if there's an overlap in communities.

### functional description

It should have users - volunteers.

It should collect following data:

- Identity:
  - Name
  - Picture
- Age
- Gender
- Occupation
- Commute habits
- Hobbies
- Values

It should ask the user to feel in data once for themselves.

It should collect additional data about the volunteer:

- Area under survey

It should provide a way for users to add more entries.

It should provide a way to create links between profiles entered earlier.

### UI requirements

It should show an introduction informing respondents about data being collected.

It should require consent to proceed with data collection.

It should show a grid of pictures with names to allow respondents to pick from and tell whom they know personally.
