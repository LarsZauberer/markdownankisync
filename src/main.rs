use markdownankisync::anki::Card;
use markdownankisync::{api::get_decks, renderer::render};

fn main() {
    /* println!(
            "{}",
            render(
                "# Hello World #card

    \\[a^2 + b^2 = c^2\\]

    $$a^2 + b^2$$

    $$a + b = c$$

    $a^2$

    [source](imgs/something.jpeg)

    [source](assets/imgs/asdf.jpeg)"
            )
        ) */
    println!(
        "{:?}",
        Card::new(
            "This is some front text 7",
            "This is some back text <br> $$a^2 + b^2 = c^2$$ <br> [source](Rust%20Serde.md)"
        )
    );
}
