# Markdown Anki Sync

## Architectual idea

The app goes through all the markdown files in a certain directory and finds all the _headings_ with a #card tag at the end. The heading combined with the upper headings will be the front of the card and everything after the section until the next section starts will be the back.

The card id will be added as an HTML comment at the end of the card. If the id is not present we will add the card. Otherwise it will try to update it.

### Rendering Ideas

Math substitutions are: $ -> \[ and $$ -> \(

Links to other files are replaced with a correct absolute path to the file.

Images need to be uploaded first to `storeMediaFile` and then we can use it in the card.

### Card Types

> Fact/Situation:
>
> Anki creates for card types that have 2 cards (e.g. the "and reverse" type) two different cards with two different ids. But these cards are linked in some way.

The app will create for all the double card types two seperate basic cards that are linked by a seperate internal structure that holds two objects of type `Card`.

#### Card Type Regex

- [ ] WIP

## Roadmap

### Quick Editor

Reads only the `basic` cards from the markdown wiki and creates cards from it.

Features:

- Support only for basic cards
- Hiragana support
- Image support
- Update support
- From section to next section
