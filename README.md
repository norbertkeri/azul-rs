# Azul in rust

My pet project for learning rust, an implementation of the board game called [Azul](https://boardgamegeek.com/boardgame/230802/azul) (the first version).

It looks like this right now:

![image of a game in progress](./assets/1650565479.png)

# Legend
The two players (Alice and Bob) are fixed at the moment. The empty rectangles are the tiling area, where you place the tiles. To the right of the tiling area is the wall, where you place the tiles, if you finish a row. When a tile is finished on the wall, it gets replaced with an X. The 1122233 numbers are the floor line, this is where tiles that you cannot put anywhere else will be placed, and earn minus points at the end of a round.

The factories section are the tiles you can pick from, each letter represents a color. The "common" section is where the non-picked tiles land.

# Controls

* j - Selection down
* k - Selection up
* enter - Confirm
* esc - Back

At the moment it's two players only, with no remote multiplayer (so both players would have to sit at the same computer). There is a really basic TUI interface that I threw together using [termion](https://crates.io/crates/termion). Unfortunately this UI doesn't really lend itself to actually teaching the game, so unless you already know the rules, there is a good chance the game won't make much sense.

## Possible improvements as I go along

The game logic is pretty much 95% there, the only thing that is missing is being able to put all tiles on the floor line, even if you have open rows. This was actually something that I completely missed from the rules.

There are also some things left on my todo list that I'd like to eventually implement:

- [ ] Have some kind of rules/legend explanation
- [ ] Play with 3-4 players
- [ ] Remote multiplayer

## Tests
A major reason why I wanted to write my own TUI, and keep it without any styling (like colors) is that I can write tests like this:

```rust
#[test]
fn test_two_panels_horizontally() {
    let hellos = ["Hello", "Hello"]
        .into_iter()
        .map(|s| {
            let panel = PanelBuilder::default()
                .component(Box::new(TextView::new(String::from(s))) as Box<_>)
                .build()
                .unwrap();
            Box::new(panel) as Box<_>
        })
        .collect();
    let panel = PanelBuilder::default()
        .component(Box::new(Layout::horizontal(0, hellos)))
        .build()
        .unwrap();
    let expected = r#"
┌──────────────┐
│┌─────┐┌─────┐│
││Hello││Hello││
│└─────┘└─────┘│
└──────────────┘"#
        .trim_start();
    expect_component(panel, expected);
}
```
Basically, I can write the ASCII representation of a component inside the test, and assert against that, which is really neat.
