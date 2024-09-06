# Markdown Anki Sync

## Architectual idea

The app goes through all the markdown files in a certain directory and finds all the _headings_ with a #card tag at the end. The heading combined with the upper headings will be the front of the card and everything after the section until the next section starts will be the back.

The card id will be added as an HTML comment at the end of the card. If the id is not present we will add the card. Otherwise it will try to update it.

### Rendering Ideas

Math substitutions are: $ -> \[ and $$ -> \(

A mardown to HTML converter might be necessary so that everything is displayed correctly in the card

Images need to be uploaded first to `storeMediaFile` and then we can use it in the card.
