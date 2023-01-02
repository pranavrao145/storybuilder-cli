# StoryBuilder CLI

CLI client for StoryBuilder. Made in Rust.

## How to Play

StoryBuilder is a simple game where several players join a room. The host, or
Player 1, starts by supplying the first line of a story. After that, Player 2,
enters the next line of the story, and is shown Player 1's line as context.
Then, Player 3 enters the next line of the story with only Player 2's line as
context, and so on. This means that the overall story is built up with each
player having _only one line as context_, which, in most instances, makes the
story hilarious. At the end, when the host decides to end the story on their
turn, the full story is revealed for everyone to enjoy.

## Running the Game

### Requirements

- A running instance of [StoryBuilder server](https://github.com/pranavrao145/storybuilder-server) (check the README at the link for how to run the server), and the URL of that instance.
- `make`
- `cargo`

### How to Run

First, if it's not already there, clone this repository to your local machine:

```
git clone https://github.com/pranavrao145/storybuilder-cli.git
```

Change into the directory you cloned:

```
cd storybuilder-cli
```

To run the CLI, execute the following command:

```
SERVER="<your server url here>" make
```
